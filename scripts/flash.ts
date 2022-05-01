// @ts-ignore
import { ethers } from "hardhat";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/dist/src/signer-with-address";
import * as dotenv from "dotenv";
import {
  FlashLoanTerminator__factory,
  ITerminator,
  Terminator__factory,
} from "../types/ethers-v5";
import {
  AddressProvider__factory,
  CreditFilter,
  CreditFilter__factory,
  CreditManager,
  CreditManager__factory,
  formatBN,
  ISwapRouter,
  ISwapRouter__factory,
  UNISWAP_V3_ROUTER,
  WAD,
  WETHGateway,
  WETHGateway__factory,
  WETHToken,
  YEARN_DAI_ADDRESS,
  YEARN_USDC_ADDRESS,
  YearnAdapter,
  YearnAdapter__factory,
} from "@diesellabs/gearbox-sdk";
import { getClosePath, UniV2Params } from "./getPaths";
import { setLT } from "./setLT";
import { BigNumber, EventFilter } from "ethers";
import { deploy, waitForTransaction } from "../utils/transaction";
import { sleep } from "../utils/sleep";
import fs from "fs";

const tVersion = 1; // TERMINATOR VERSION
const external = false;
const creditManagerAddr = "0x968f9a68a98819e2e6bb910466e191a7b6cf02f0";

export interface StoredData {
  tokens: Array<string>;
  lts: Array<number>;
}

class TerminatorTestSuite {
  // ACCOUNTS
  deployer: SignerWithAddress;
  user: SignerWithAddress;
  liquidator1: SignerWithAddress;
  liquidator2: SignerWithAddress;

  // CONTRACTS
  creditManager: CreditManager;
  creditFilter: CreditFilter;
  uniswapAdapter: ISwapRouter;
  yearnUSDCAdapter: YearnAdapter;
  yearnDAIAdapter: YearnAdapter;
  wethGateway: WETHGateway;

  tokens: Array<string>;
  lts: Array<number>;
  allowedTokensQty: number;

  async setup() {
    dotenv.config({ path: ".env.local" });
    const accounts = (await ethers.getSigners()) as Array<SignerWithAddress>;
    this.deployer = accounts[0];
    this.user = accounts[1];
    this.liquidator1 = accounts[2];
    this.liquidator2 = accounts[3];

    const addressProvider = process.env.REACT_APP_ADDRESS_PROVIDER || "";
    const ap = AddressProvider__factory.connect(addressProvider, this.deployer);

    console.log("Deployer:", this.deployer.address);
    console.log("Address provider:", addressProvider);
    console.log("CreditManager:", creditManagerAddr);

    console.log("Transferring ETH to this.user account");

    await this.deployer.sendTransaction({
      to: this.user.address,
      value: WAD.mul(100),
    });

    this.creditManager = CreditManager__factory.connect(
      creditManagerAddr,
      this.user
    );
    this.creditFilter = CreditFilter__factory.connect(
      await this.creditManager.creditFilter(),
      this.deployer
    );
    this.uniswapAdapter = ISwapRouter__factory.connect(
      await this.creditFilter.contractToAdapter(UNISWAP_V3_ROUTER),
      this.user
    );

    this.yearnUSDCAdapter = YearnAdapter__factory.connect(
      await this.creditFilter.contractToAdapter(YEARN_USDC_ADDRESS),
      this.user
    );
    this.yearnDAIAdapter = YearnAdapter__factory.connect(
      await this.creditFilter.contractToAdapter(YEARN_DAI_ADDRESS),
      this.user
    );

    this.allowedTokensQty = (
      await this.creditFilter.allowedTokensCount()
    ).toNumber();

    this.wethGateway = WETHGateway__factory.connect(
      await ap.getWETHGateway(),
      this.user
    );

    await this.getLTS();

    console.log("Opened accounts:", await this.openedContracts());
  }

  //
  //
  //
  //
  async deploy() {
    await this.setup();
    // TERMINATOR DEPLOYMENT

    const terminator = await this.deployTerminator("terminator");
    const flashLoanTerminator = await this.deployTerminator("flashTerminator");

    await waitForTransaction(
      this.deployer.sendTransaction({ to: terminator.address, value: WAD })
    );

    console.log(`TERMINATOR_ADDRESS=${terminator.address}`);
    console.log(`TERMINATOR_FLASH_ADDRESS=${flashLoanTerminator.address}`);

    // STARTING MAIN CYCLE

    for (let i = 0; i < this.allowedTokensQty; i++) {
      if (
        !(await this.creditManager.hasOpenedCreditAccount(this.user.address))
      ) {
        console.log("Open new account");

        await waitForTransaction(
          this.wethGateway.openCreditAccountETH(
            creditManagerAddr,
            this.user.address,
            300,
            0,
            {
              value: WAD,
            }
          )
        );
      }

      console.log(
        "Liquidated accounts:",
        await this.countEvents(
          this.creditManager.filters.LiquidateCreditAccount()
        )
      );

      await this.changeLTS(true);

      for (let c = 0; c <= i; c++) {
        const tokenOut = this.tokens[c];
        if (
          tokenOut.toLowerCase() !==
          "0x1494CA1F11D487c2bBe4543E90080AeBa4BA3C2b".toLowerCase()
        ) {
          if (c > 0) {
            switch (tokenOut) {
              case YEARN_USDC_ADDRESS:
                console.log("YEARN-USDC");

                await waitForTransaction(
                  this.yearnUSDCAdapter["deposit(uint256)"](
                    BigNumber.from(10)
                      .pow(6)
                      .div(this.allowedTokensQty + 25)
                  )
                );

                console.log("YEARN-USDC DONE");

                break;

              case YEARN_DAI_ADDRESS:
                console.log("YEARN-DAI");

                await waitForTransaction(
                  this.yearnDAIAdapter["deposit(uint256)"](
                    WAD.div(this.allowedTokensQty + 25)
                  )
                );
                break;

              default:
                await waitForTransaction(
                  this.uniswapAdapter.exactInputSingle({
                    tokenIn: await this.creditManager.underlyingToken(),
                    tokenOut,
                    fee: 3000,
                    recipient: this.user.address,
                    deadline: Math.floor(Date.now() / 1000 + 3600 * 365),
                    amountIn: WAD.div(this.allowedTokensQty + 5),
                    amountOutMinimum: 0,
                    sqrtPriceLimitX96: 0,
                  })
                );
            }
          }
        }
      }

      console.log(`[${i}]: ${this.tokens[i]}`);

      const borrower = this.user.address; // "0x4694ad734d08667a8612e638df28d1471bf3fdb2";

      const routes = await getClosePath(creditManagerAddr, borrower, 500);

      const reducedRoutes = routes
        .slice(1)
        .filter((e) => BigNumber.from(e.amountIn).gt(0));

      // this.printRoutes(reducedRoutes);

      // await sleep(1400);

      await this.changeLTS(false);

      if (!external) {
        try {
          // @ts-ignore
          const receipt = await (tVersion === 1
            ? terminator
            : flashLoanTerminator
          ).liquidate(creditManagerAddr, borrower, reducedRoutes, []);

          const r2 = await receipt.wait();

          // console.log("Tokens on account: ", i);
          console.log(
            `Gas used [${i}]:, ${r2.gasUsed
              .div(1000)
              .toNumber()}K @ 200 gwei ${formatBN(r2.gasUsed.mul(200), 9)} ETH`
          );
          // console.log("Gas price:", receipt.gasPrice!.toString());
        } catch (e) {
          console.log(e);
        }
      } else {
        let opened = 1;
        while (opened > 0) {
          opened = await this.openedContracts();

          if (opened > 0) {
            printProgress(
              `WAITING FOR LIQUIDATION BOT. i: ${i}, liquidated: ${opened}`
            );
          }

          await sleep(1400);
        }
      }
    }
  }

  countEvents = async (filter: EventFilter) =>
    (await this.creditManager.queryFilter(filter)).length;

  protected async openedContracts(): Promise<number> {
    // @ts-ignore
    // console.log(await this.creditManager.queryFilter("*"))

    return (
      (await this.countEvents(this.creditManager.filters.OpenCreditAccount())) -
      (await this.countEvents(
        this.creditManager.filters.CloseCreditAccount()
      )) -
      (await this.countEvents(
        this.creditManager.filters.RepayCreditAccount()
      )) -
      (await this.countEvents(
        this.creditManager.filters.LiquidateCreditAccount()
      ))
    );
  }

  protected async deployTerminator(
    terminatorType: "terminator" | "flashTerminator"
  ): Promise<ITerminator> {
    const terminatorFactory =
      terminatorType === "terminator"
        ? ((await ethers.getContractFactory(
            "Terminator"
          )) as Terminator__factory)
        : ((await ethers.getContractFactory(
            "TerminatorFlash"
          )) as FlashLoanTerminator__factory);

    const terminator = await terminatorFactory.deploy(
      WETHToken.Mainnet,
      this.deployer.address
    );

    await terminator.deployed();
    await waitForTransaction(
      terminator.provideAllowance([creditManagerAddr], this.tokens)
    );

    for (let exec of [this.deployer, this.liquidator1, this.liquidator2]) {
      await waitForTransaction(terminator.allowExecutor(exec.address));
    }

    return terminator;
  }

  protected printRoutes(routes: Array<UniV2Params>) {
    for (let r of routes) {
      console.log("amountIn: ", r.amountIn.toString());
      console.log("path: ", r.path);
      console.log("amountOutMin: ", r.amountOutMin.toString());
    }
  }

  protected async getLTS() {
    const filename = "lt.json";

    if (fs.existsSync(filename)) {
      console.log("getting lts from file");
      const ltsJSON = fs.readFileSync(filename, "utf-8");
      const storedData = JSON.parse(ltsJSON) as StoredData;
      this.tokens = storedData.tokens;
      this.lts = storedData.lts;
    } else {
      // STORING TOKENS INTO ARRAY
      for (let i = 0; i < this.allowedTokensQty; i++) {
        const token = this.tokens[i];
        this.lts.push(
          (await this.creditFilter.liquidationThresholds(token)).toNumber()
        );
        this.tokens.push(token);
        fs.writeFileSync(
          filename,
          JSON.stringify({ tokens: this.tokens, lts: this.lts })
        );
      }
    }
  }

  protected async changeLTS(restore?: boolean) {
    for (let c = 0; c < this.allowedTokensQty; c++) {
      const tokenOut = this.tokens[c];
      if (
        tokenOut.toLowerCase() !==
        "0x1494CA1F11D487c2bBe4543E90080AeBa4BA3C2b".toLowerCase()
      ) {
        await setLT(creditManagerAddr, tokenOut, restore ? this.lts[c] : 1);
      }
    }
  }
}

function printProgress(line: string) {
  process.stdout.clearLine(0);
  process.stdout.cursorTo(0);
  process.stdout.write(line);
}

const ts = new TerminatorTestSuite();
ts.deploy()
  .then(() => console.log("Ok"))
  .catch((e) => console.log(e));

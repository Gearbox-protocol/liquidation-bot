// @ts-ignore
import { ethers } from "hardhat";
import {
  AdapterInterface,
  CreditFilter__factory,
  CreditManager__factory,
  getConnectors,
  PathFinder__factory,
  PERCENTAGE_FACTOR,
  SwapType,
  UNISWAP_V2_ROUTER,
} from "@diesellabs/gearbox-sdk";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/dist/src/signer-with-address";
import { BigNumberish } from "ethers";
import { config as dotEnvConfig } from "dotenv";

dotEnvConfig({ path: ".env.local" });
const pathFinderAddr = process.env.REACT_APP_PATHFINDER || "";

export interface UniV2Params {
  amountIn: BigNumberish;
  path: string[];
  amountOutMin: BigNumberish;
}

export async function getClosePath(
  creditManagerAddress: string,
  borrower: string,
  slippage: number
): Promise<Array<UniV2Params>> {
  const accounts = (await ethers.getSigners()) as Array<SignerWithAddress>;
  const deployer = accounts[0];

  const creditManager = CreditManager__factory.connect(
    creditManagerAddress,
    deployer
  );

  const underlyingToken = await creditManager.underlyingToken();

  const creditFilter = CreditFilter__factory.connect(
    await creditManager.creditFilter(),
    deployer
  );

  const pathFinder = PathFinder__factory.connect(pathFinderAddr, deployer);

  const connectors = getConnectors("Mainnet");

  const address = await creditManager.getCreditAccountOrRevert(borrower);
  const count = await creditFilter.allowedTokensCount();
  const getPath = (num: number) =>
    new Promise<UniV2Params>(async (resolve, reject) => {
      try {
        const [token, amount] = await creditFilter.getCreditAccountTokenById(
          address,
          num
        );

        const found = await pathFinder.callStatic.bestUniPath(
          AdapterInterface.UniswapV2,
          UNISWAP_V2_ROUTER,
          SwapType.ExactInput,
          token,
          underlyingToken,
          amount,
          connectors
        );

        resolve({
          amountIn: amount,
          path: found.path,
          amountOutMin:
          found.expectedAmount
            .mul(PERCENTAGE_FACTOR)
            .div(PERCENTAGE_FACTOR + slippage),
        });
      } catch (e) {
        reject(e);
      }
    });

  const jobs = [];
  for (let i = 0; i < count.toNumber(); i++) {
    jobs.push(getPath(i));
  }

  return await Promise.all(jobs);
}


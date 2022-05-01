// @ts-ignore
import { ethers, network } from "hardhat";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/dist/src/signer-with-address";
import { ICreditFilter__factory } from "../types/ethers-v5";
import { CreditManager__factory, WAD } from "@diesellabs/gearbox-sdk";
import { waitForTransaction } from "../utils/transaction";

const multisig = "0xA7D5DDc1b8557914F158076b228AA91eF613f1D5";

export async function setLT(
  creditManagerAddr: string,
  token: string,
  lt: number = 1
) {
  const accounts = (await ethers.getSigners()) as Array<SignerWithAddress>;
  const deployer = accounts[0];

  const creditManager = CreditManager__factory.connect(
    creditManagerAddr,
    deployer
  );

  // const token = await creditManager.underlyingToken();

  const creditFilterAddress = await creditManager.creditFilter();
  const creditFilter = ICreditFilter__factory.connect(
    creditFilterAddress,
    deployer
  );

  await network.provider.request({
    method: "hardhat_impersonateAccount",
    params: [multisig],
  });

  const signer = (await ethers.provider.getSigner(
    multisig
  )) as unknown as SignerWithAddress;

  const underlyingToken = await creditFilter.underlyingToken();

  await deployer.sendTransaction({
    to: multisig,
    value: WAD.div(1),
  });
  if (token === underlyingToken) {

    await waitForTransaction(
      creditManager
        .connect(signer)
        .setParams(
          await creditManager.minAmount(),
          await creditManager.maxAmount(),
          await creditManager.maxLeverageFactor(),
          await creditManager.feeInterest(),
          await creditManager.feeLiquidation(),
          await creditManager.liquidationDiscount()
        )
    );
  }
  await waitForTransaction(creditFilter.connect(signer).allowToken(token, lt));
}

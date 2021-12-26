// @ts-ignore
import {ethers} from "hardhat";
import * as dotenv from "dotenv";
import {Terminator__factory} from "../types/ethers-v5";
import {SignerWithAddress} from "@nomiclabs/hardhat-ethers/signers";

async function deploy() {
  dotenv.config();

  const accounts = (await ethers.getSigners()) as Array<SignerWithAddress>;
  const deployer = accounts[0];

  const botAddress = process.env.BOT_ADDRESS || "";


  console.log("Allow executro");
  const terminator = Terminator__factory.connect(botAddress, deployer)
  const r2 = await terminator.allowExecutor('0x445302b05DbB5d3d499cD797FcaA15297A84b348')
  await r2.wait()
}

deploy()
  .then(() => {
    process.exit(0);
  })
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });

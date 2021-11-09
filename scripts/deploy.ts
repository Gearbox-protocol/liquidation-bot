// @ts-ignore
import { ethers } from "hardhat";
import { Terminator__factory } from "../types/ethers-v5";
import fs from "fs";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
import { WETHToken } from "@diesellabs/gearbox-sdk";

async function deploy() {
  const terminatorFactory = (await ethers.getContractFactory(
    "Terminator"
  )) as Terminator__factory;

  const accounts = (await ethers.getSigners()) as Array<SignerWithAddress>;
  const deployer = accounts[0];

  const chainId = await deployer.getChainId();

  console.log("Deploying terminator");
  const terminator = await terminatorFactory.deploy(
    chainId === 42 ? WETHToken.Kovan : WETHToken.Mainnet
  );
  await terminator.deployed();
  console.log(`Terminator contract deployed at ${terminator.address}`);
  
  const yearn = [
   "0x67A022C14E1e6517F45E92BF7C76249c0967569d",
      "0xe5267045739E4d6FcA15BB4a79190012F146893b",
      "0x3B55a47d6ffE0b7bb1762109faFa5B84180c1111",
    "0x980E4d8A22105c2a2fA2252B7685F32fc7564512"
];

  for(let yearnAddress of yearn) {
    const r = await terminator.addYearn(yearnAddress);
    await r.wait()
  }


  console.log("Writing .env file");
  const envFile = `BOT_ADDRESS=${terminator.address}`;
  fs.writeFileSync("./.env.local", envFile);
}

deploy()
  .then(() => {
    process.exit(0);
  })
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });

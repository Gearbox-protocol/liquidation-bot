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

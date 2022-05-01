// @ts-ignore
import { ethers } from "hardhat";
import {
  Terminator__factory,
  TerminatorFlash__factory,
} from "../types/ethers-v5";
import fs from "fs";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
import {
  AddressProvider__factory,
  DataCompressor__factory,
  WETHToken,
} from "@diesellabs/gearbox-sdk";
import { waitForTransaction } from "../utils/transaction";
import { StoredData } from "./flash";
import * as dotenv from "dotenv";

dotenv.config({ path: ".env.local" });

const beneficiary = "0x19301B8e700925E850C945a28256b6A6FDe5904C"; // "0x391fdaB873d3AD86Ed03509A8830dF60a7851068";

async function deploy() {
  const terminatorFactory = (await ethers.getContractFactory(
    "Terminator"
  )) as Terminator__factory;

  const accounts = (await ethers.getSigners()) as Array<SignerWithAddress>;
  const deployer = accounts[0];

  console.log("Deployer", deployer.address);

  const chainId = await deployer.getChainId();

  console.log(`Chain id: ${chainId}`);
  console.log("Deploying terminator");
  const terminator = await terminatorFactory.deploy(
    chainId === 42 ? WETHToken.Kovan : WETHToken.Mainnet,
    beneficiary
  );
  await terminator.deployed();
  await waitForTransaction(
    terminator.allowExecutor("0x445302b05DbB5d3d499cD797FcaA15297A84b348")
  );

  console.log("DEP");
  console.log("DEP");

  const addressProvider = process.env.REACT_APP_ADDRESS_PROVIDER || "";
  const ap = AddressProvider__factory.connect(addressProvider, deployer);
  const dc = await ap.getDataCompressor();
  const dataCompressor = DataCompressor__factory.connect(dc, deployer);

  const cmList = await dataCompressor.getCreditManagersList(deployer.address);

  const cms = cmList.map((c) => c.addr);

  // const terminator = TerminatorFlash__factory.connect(
  //   "0xAD4B61F5bce7841e0c5DeA42F8F00A17e95bFd9A",
  //   deployer
  // );

  const tokenMap: Record<string, boolean> = {};

  cmList.forEach((cm) => {
    tokenMap[cm.underlyingToken] = true;
    cm.allowedTokens.forEach((t) => (tokenMap[t] = true));
  });

  const tokens = Object.keys(tokenMap);

  console.log(cms);
  console.log(tokens);

  await waitForTransaction(terminator.provideAllowance(cms, tokens));

  console.log(`Terminator contract deployed at ${terminator.address}`);

  // console.log("Writing .env file");
  // const envFile = `BOT_ADDRESS=${terminator.address}`;
  // fs.writeFileSync("./.env.local", envFile);
}

deploy()
  .then(() => {
    process.exit(0);
  })
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });

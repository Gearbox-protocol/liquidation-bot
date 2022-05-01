// @ts-ignore
import { ethers, network } from "hardhat";
import { TransactionReceipt } from "@ethersproject/providers";
import { ContractTransaction, ContractFactory, Contract } from "ethers";
import { Logger } from "tslog";
import { formatBN } from "./formatter";

export async function waitForTransaction(
  transaction: Promise<ContractTransaction>,
  logger?: Logger,
): Promise<TransactionReceipt> {
  const request = await transaction;
  const txReceipt = await request.wait();

  if (logger) {
    logger.debug(`Tx: ${txReceipt.transactionHash}`);
    logger.debug(`Gas used: ${txReceipt.gasUsed.toString()} @ ${formatBN(txReceipt.effectiveGasPrice, 9)} gwei.  Total: ${formatBN(txReceipt.gasUsed.mul(txReceipt.effectiveGasPrice))} ETH`);
  }

  return txReceipt;
}

export type ContractFactoryConstructor<T extends ContractFactory> = new (
  ...args: any[]
) => T;
export type ContractConstructor<T extends Contract> = new (...args: any[]) => T;

export async function deploy<T extends Contract>(
  name: string,
  logger: Logger,
  ...args: any[]
): Promise<T> {
  const artifact = await ethers.getContractFactory(name);

  const contract = (await artifact.deploy(...args)) as T;
  logger.debug(`Deploying ${name}...`);
  await contract.deployed();
  const txReceipt = await contract.deployTransaction.wait();
  logger.debug(`Deployed ${name} to ${contract.address}`);
  logger.debug(`Tx: ${txReceipt.transactionHash}`);
  logger.debug(`Gas used: ${txReceipt.gasUsed.toString()} @ ${formatBN(txReceipt.effectiveGasPrice, 9)} gwei.  Total: ${formatBN(txReceipt.gasUsed.mul(txReceipt.effectiveGasPrice))} ETH`);
  return contract;
}

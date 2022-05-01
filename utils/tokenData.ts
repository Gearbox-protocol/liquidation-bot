/*
 * SPDX-License-Identifier: BUSL-1.1
 * Gearbox. Generalized leverage protocol, which allows to take leverage and then use it across other DeFi protocols and platforms in a composable way.
 * (c) Gearbox.fi, 2021
 */

import {
  TokenData,
  tokenDataByNetwork,
  WETHToken,
  YEARN_DAI_ADDRESS,
  YEARN_USDC_ADDRESS
} from "@diesellabs/gearbox-sdk";
import { IAppERC20__factory } from "../types/ethers-v5";
import { Signer } from "ethers";

export function getTokenData(
  address: string,
  signer: Signer
): Promise<TokenData> {
  return new Promise<TokenData>(async (resolve) => {
    const tokenContract = IAppERC20__factory.connect(address, signer);
    const symbol = await tokenContract.symbol();
    const decimals = await tokenContract.decimals();
    resolve(
      new TokenData({
        symbol,
        decimals,
        addr: address,
      })
    );
  });
}

export async function getMainnetTokenData(
  signer: Signer
): Promise<Array<TokenData>> {
  const jobs = Object.values(tokenDataByNetwork.Mainnet).map((e) =>
    getTokenData(e.address, signer)
  );
  return [
    ...(await Promise.all(jobs)),
    new TokenData({ addr: WETHToken.Mainnet, decimals: 18, symbol: "WETH" }),
    new TokenData({addr: YEARN_DAI_ADDRESS, decimals: 18, symbol: "yDAI"}),
    new TokenData({addr: YEARN_USDC_ADDRESS, decimals: 6, symbol: "yUSDC"}),
  ];
}

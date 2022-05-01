import { BigNumber, BigNumberish } from "ethers";

export function formatBN(
  num?: BigNumberish,
  decimals?: number,
  precision: number = 4
): string {
  if (!num) return "-";

  if (BigNumber.from(num).gt(BigNumber.from(10).pow(28))) {
    return "MAX";
  }

  if (BigNumber.from(num).gt(BigNumber.from(10).pow(21))) {
    precision=2;
  }

  if (BigNumber.from(num).gt(BigNumber.from(10).pow(24))) {
    precision=0;
  }

  return (
    BigNumber.from(num)
      .div(BigNumber.from(10).pow((decimals || 18) - 4))
      .toNumber() / 10000
  ).toFixed(precision);
}

export function toBN(num: number, decimals?: number): BigNumber {
  return BigNumber.from(Math.floor(num * 10000)).mul(
    BigNumber.from(10).pow((decimals || 18) - 4)
  );
}



export const formatRate = (rate: BigNumberish | undefined) =>
  rate
    ? (BigNumber.from(rate)
      .div(BigNumber.from(10).pow(14))
      .toNumber() / 100)
      .toFixed(2)+ "%"
    : "0.00%";

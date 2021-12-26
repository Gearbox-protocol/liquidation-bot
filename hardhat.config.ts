import { config as dotEnvConfig } from "dotenv";
import { HardhatUserConfig } from "hardhat/types";

import "@nomiclabs/hardhat-waffle";
import "@nomiclabs/hardhat-etherscan";
import "@nomiclabs/hardhat-ethers";
import "@typechain/hardhat";
import "hardhat-gas-reporter";
import "hardhat-abi-exporter";
import "hardhat-contract-sizer";
import "solidity-coverage";
import { LOCAL_NETWORK, MAINNET_NETWORK } from "@diesellabs/gearbox-sdk";

// gets data from .env file
dotEnvConfig();

const PRIVATE_KEY =
  process.env.PRIVATE_KEY! ||
  "0xc87509a1c067bbde78beb793e6fa76530b6382a4c0241e5e4a9ec0a0f44dc0d3"; // well known private key

const config: HardhatUserConfig = {
  defaultNetwork: "hardhat",
  solidity: {
    compilers: [{ version: "0.7.6", settings: {} }],
  },
  networks: {
    hardhat: {
      chainId: LOCAL_NETWORK,
      initialBaseFeePerGas: 0,
    },
    localhost: {
      accounts: [PRIVATE_KEY],
    },
    mainnet: {
      url: process.env.ETH_MAINNET_PROVIDER,
      accounts: [PRIVATE_KEY],
      chainId: MAINNET_NETWORK,
    },

    kovan: {
      url: process.env.ETH_KOVAN_PROVIDER,
      accounts: [PRIVATE_KEY],
      gasPrice: 2e9,
      minGasPrice: 1e9,
    },
  },

  gasReporter: {
    enabled: false,
    currency: "USD",
    gasPrice: 21,
  },
  typechain: {
    outDir: "types/ethers-v5",
    target: "ethers-v5",
  },
  abiExporter: {
    path: "./abi",
    clear: true,
    flat: true,
    spacing: 2,
  },
  contractSizer: {
    alphaSort: false,
    disambiguatePaths: false,
    runOnCompile: true,
  },
};

export default config;

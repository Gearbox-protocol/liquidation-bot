pub use ilendingpool_mod::*;
#[allow(clippy::too_many_arguments)]
mod ilendingpool_mod {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::{
        contract::{
            self as ethers_contract,
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            self as ethers_core,
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::{self as ethers_providers, Middleware},
    };
    #[doc = "ILendingPool was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ILENDINGPOOL_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"reserve\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"borrowRateMode\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"borrowRate\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint16\",\n        \"name\": \"referral\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"Borrow\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"reserve\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint16\",\n        \"name\": \"referral\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"Deposit\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"target\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"initiator\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"premium\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint16\",\n        \"name\": \"referralCode\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"FlashLoan\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"collateralAsset\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"debtAsset\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"debtToCover\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidatedCollateralAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"liquidator\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"receiveAToken\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"LiquidationCall\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [],\n    \"name\": \"Paused\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"reserve\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RebalanceStableBorrowRate\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"reserve\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"repayer\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Repay\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"reserve\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidityRate\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"stableBorrowRate\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"variableBorrowRate\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidityIndex\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"variableBorrowIndex\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ReserveDataUpdated\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"reserve\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ReserveUsedAsCollateralDisabled\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"reserve\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ReserveUsedAsCollateralEnabled\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"reserve\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"rateMode\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Swap\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [],\n    \"name\": \"Unpaused\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"reserve\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Withdraw\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"interestRateMode\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"referralCode\",\n        \"type\": \"uint16\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"borrow\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"referralCode\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"deposit\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"balanceFromAfter\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"balanceToBefore\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"finalizeTransfer\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"receiverAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"assets\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amounts\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"modes\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"params\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"referralCode\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"flashLoan\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getAddressesProvider\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ILendingPoolAddressesProvider\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getConfiguration\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"data\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.ReserveConfigurationMap\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getReserveData\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"data\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DataTypes.ReserveConfigurationMap\",\n            \"name\": \"configuration\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"liquidityIndex\",\n            \"type\": \"uint128\"\n          },\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"variableBorrowIndex\",\n            \"type\": \"uint128\"\n          },\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"currentLiquidityRate\",\n            \"type\": \"uint128\"\n          },\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"currentVariableBorrowRate\",\n            \"type\": \"uint128\"\n          },\n          {\n            \"internalType\": \"uint128\",\n            \"name\": \"currentStableBorrowRate\",\n            \"type\": \"uint128\"\n          },\n          {\n            \"internalType\": \"uint40\",\n            \"name\": \"lastUpdateTimestamp\",\n            \"type\": \"uint40\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"aTokenAddress\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"stableDebtTokenAddress\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"variableDebtTokenAddress\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"interestRateStrategyAddress\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"id\",\n            \"type\": \"uint8\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.ReserveData\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getReserveNormalizedIncome\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getReserveNormalizedVariableDebt\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getReservesList\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getUserAccountData\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"totalCollateralETH\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"totalDebtETH\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"availableBorrowsETH\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"currentLiquidationThreshold\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"ltv\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"healthFactor\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getUserConfiguration\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"data\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.UserConfigurationMap\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"reserve\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"aTokenAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"stableDebtAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"variableDebtAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"interestRateStrategyAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"initReserve\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"collateralAsset\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"debtAsset\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"debtToCover\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"receiveAToken\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"liquidationCall\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"paused\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"rebalanceStableBorrowRate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"rateMode\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"repay\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"reserve\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"configuration\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setConfiguration\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"val\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setPause\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"reserve\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"rateStrategyAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setReserveInterestRateStrategyAddress\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"useAsCollateral\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setUserUseReserveAsCollateral\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"rateMode\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"swapBorrowRateMode\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"asset\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"withdraw\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct ILendingPool<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for ILendingPool<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for ILendingPool<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ILendingPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> ILendingPool<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), ILENDINGPOOL_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `borrow` (0xa415bcad) function"]
        pub fn borrow(
            &self,
            asset: ethers_core::types::Address,
            amount: ethers_core::types::U256,
            interest_rate_mode: ethers_core::types::U256,
            referral_code: u16,
            on_behalf_of: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [164, 21, 188, 173],
                    (
                        asset,
                        amount,
                        interest_rate_mode,
                        referral_code,
                        on_behalf_of,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xe8eda9df) function"]
        pub fn deposit(
            &self,
            asset: ethers_core::types::Address,
            amount: ethers_core::types::U256,
            on_behalf_of: ethers_core::types::Address,
            referral_code: u16,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [232, 237, 169, 223],
                    (asset, amount, on_behalf_of, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `finalizeTransfer` (0xd5ed3933) function"]
        pub fn finalize_transfer(
            &self,
            asset: ethers_core::types::Address,
            from: ethers_core::types::Address,
            to: ethers_core::types::Address,
            amount: ethers_core::types::U256,
            balance_from_after: ethers_core::types::U256,
            balance_to_before: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 237, 57, 51],
                    (
                        asset,
                        from,
                        to,
                        amount,
                        balance_from_after,
                        balance_to_before,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `flashLoan` (0xab9c4b5d) function"]
        pub fn flash_loan(
            &self,
            receiver_address: ethers_core::types::Address,
            assets: Vec<ethers_core::types::Address>,
            amounts: Vec<ethers_core::types::U256>,
            modes: Vec<ethers_core::types::U256>,
            on_behalf_of: ethers_core::types::Address,
            params: Vec<u8>,
            referral_code: u16,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [171, 156, 75, 93],
                    (
                        receiver_address,
                        assets,
                        amounts,
                        modes,
                        on_behalf_of,
                        params,
                        referral_code,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAddressesProvider` (0xfe65acfe) function"]
        pub fn get_addresses_provider(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([254, 101, 172, 254], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getConfiguration` (0xc44b11f7) function"]
        pub fn get_configuration(
            &self,
            asset: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, (ethers_core::types::U256,)> {
            self.0
                .method_hash([196, 75, 17, 247], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveData` (0x35ea6a75) function"]
        pub fn get_reserve_data(
            &self,
            asset: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                (ethers_core::types::U256,),
                u128,
                u128,
                u128,
                u128,
                u128,
                u64,
                ethers_core::types::Address,
                ethers_core::types::Address,
                ethers_core::types::Address,
                ethers_core::types::Address,
                u8,
            ),
        > {
            self.0
                .method_hash([53, 234, 106, 117], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveNormalizedIncome` (0xd15e0053) function"]
        pub fn get_reserve_normalized_income(
            &self,
            asset: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([209, 94, 0, 83], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveNormalizedVariableDebt` (0x386497fd) function"]
        pub fn get_reserve_normalized_variable_debt(
            &self,
            asset: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([56, 100, 151, 253], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReservesList` (0xd1946dbc) function"]
        pub fn get_reserves_list(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::Address>> {
            self.0
                .method_hash([209, 148, 109, 188], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserAccountData` (0xbf92857c) function"]
        pub fn get_user_account_data(
            &self,
            user: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash([191, 146, 133, 124], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserConfiguration` (0x4417a583) function"]
        pub fn get_user_configuration(
            &self,
            user: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, (ethers_core::types::U256,)> {
            self.0
                .method_hash([68, 23, 165, 131], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initReserve` (0x7a708e92) function"]
        pub fn init_reserve(
            &self,
            reserve: ethers_core::types::Address,
            a_token_address: ethers_core::types::Address,
            stable_debt_address: ethers_core::types::Address,
            variable_debt_address: ethers_core::types::Address,
            interest_rate_strategy_address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [122, 112, 142, 146],
                    (
                        reserve,
                        a_token_address,
                        stable_debt_address,
                        variable_debt_address,
                        interest_rate_strategy_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidationCall` (0x00a718a9) function"]
        pub fn liquidation_call(
            &self,
            collateral_asset: ethers_core::types::Address,
            debt_asset: ethers_core::types::Address,
            user: ethers_core::types::Address,
            debt_to_cover: ethers_core::types::U256,
            receive_a_token: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [0, 167, 24, 169],
                    (
                        collateral_asset,
                        debt_asset,
                        user,
                        debt_to_cover,
                        receive_a_token,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(&self) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rebalanceStableBorrowRate` (0xcd112382) function"]
        pub fn rebalance_stable_borrow_rate(
            &self,
            asset: ethers_core::types::Address,
            user: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 17, 35, 130], (asset, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repay` (0x573ade81) function"]
        pub fn repay(
            &self,
            asset: ethers_core::types::Address,
            amount: ethers_core::types::U256,
            rate_mode: ethers_core::types::U256,
            on_behalf_of: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([87, 58, 222, 129], (asset, amount, rate_mode, on_behalf_of))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setConfiguration` (0xb8d29276) function"]
        pub fn set_configuration(
            &self,
            reserve: ethers_core::types::Address,
            configuration: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 210, 146, 118], (reserve, configuration))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPause` (0xbedb86fb) function"]
        pub fn set_pause(&self, val: bool) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 219, 134, 251], val)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReserveInterestRateStrategyAddress` (0x1d2118f9) function"]
        pub fn set_reserve_interest_rate_strategy_address(
            &self,
            reserve: ethers_core::types::Address,
            rate_strategy_address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 33, 24, 249], (reserve, rate_strategy_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUserUseReserveAsCollateral` (0x5a3b74b9) function"]
        pub fn set_user_use_reserve_as_collateral(
            &self,
            asset: ethers_core::types::Address,
            use_as_collateral: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 59, 116, 185], (asset, use_as_collateral))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapBorrowRateMode` (0x94ba89a2) function"]
        pub fn swap_borrow_rate_mode(
            &self,
            asset: ethers_core::types::Address,
            rate_mode: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 186, 137, 162], (asset, rate_mode))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x69328dec) function"]
        pub fn withdraw(
            &self,
            asset: ethers_core::types::Address,
            amount: ethers_core::types::U256,
            to: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([105, 50, 141, 236], (asset, amount, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Borrow` event"]
        pub fn borrow_filter(&self) -> ethers_contract::builders::Event<M, BorrowFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(&self) -> ethers_contract::builders::Event<M, DepositFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FlashLoan` event"]
        pub fn flash_loan_filter(&self) -> ethers_contract::builders::Event<M, FlashLoanFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LiquidationCall` event"]
        pub fn liquidation_call_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, LiquidationCallFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(&self) -> ethers_contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RebalanceStableBorrowRate` event"]
        pub fn rebalance_stable_borrow_rate_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, RebalanceStableBorrowRateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Repay` event"]
        pub fn repay_filter(&self) -> ethers_contract::builders::Event<M, RepayFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveDataUpdated` event"]
        pub fn reserve_data_updated_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ReserveDataUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveUsedAsCollateralDisabled` event"]
        pub fn reserve_used_as_collateral_disabled_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ReserveUsedAsCollateralDisabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveUsedAsCollateralEnabled` event"]
        pub fn reserve_used_as_collateral_enabled_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ReserveUsedAsCollateralEnabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Swap` event"]
        pub fn swap_filter(&self) -> ethers_contract::builders::Event<M, SwapFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(&self) -> ethers_contract::builders::Event<M, UnpausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Withdraw` event"]
        pub fn withdraw_filter(&self) -> ethers_contract::builders::Event<M, WithdrawFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, ILendingPoolEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "Borrow",
        abi = "Borrow(address,address,address,uint256,uint256,uint256,uint16)"
    )]
    pub struct BorrowFilter {
        #[ethevent(indexed)]
        pub reserve: ethers_core::types::Address,
        pub user: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
        pub borrow_rate_mode: ethers_core::types::U256,
        pub borrow_rate: ethers_core::types::U256,
        #[ethevent(indexed)]
        pub referral: u16,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "Deposit",
        abi = "Deposit(address,address,address,uint256,uint16)"
    )]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub reserve: ethers_core::types::Address,
        pub user: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
        #[ethevent(indexed)]
        pub referral: u16,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "FlashLoan",
        abi = "FlashLoan(address,address,address,uint256,uint256,uint16)"
    )]
    pub struct FlashLoanFilter {
        #[ethevent(indexed)]
        pub target: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub initiator: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub asset: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
        pub premium: ethers_core::types::U256,
        pub referral_code: u16,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "LiquidationCall",
        abi = "LiquidationCall(address,address,address,uint256,uint256,address,bool)"
    )]
    pub struct LiquidationCallFilter {
        #[ethevent(indexed)]
        pub collateral_asset: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub debt_asset: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers_core::types::Address,
        pub debt_to_cover: ethers_core::types::U256,
        pub liquidated_collateral_amount: ethers_core::types::U256,
        pub liquidator: ethers_core::types::Address,
        pub receive_a_token: bool,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Paused", abi = "Paused()")]
    pub struct PausedFilter();
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "RebalanceStableBorrowRate",
        abi = "RebalanceStableBorrowRate(address,address)"
    )]
    pub struct RebalanceStableBorrowRateFilter {
        #[ethevent(indexed)]
        pub reserve: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Repay", abi = "Repay(address,address,address,uint256)")]
    pub struct RepayFilter {
        #[ethevent(indexed)]
        pub reserve: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub repayer: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "ReserveDataUpdated",
        abi = "ReserveDataUpdated(address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ReserveDataUpdatedFilter {
        #[ethevent(indexed)]
        pub reserve: ethers_core::types::Address,
        pub liquidity_rate: ethers_core::types::U256,
        pub stable_borrow_rate: ethers_core::types::U256,
        pub variable_borrow_rate: ethers_core::types::U256,
        pub liquidity_index: ethers_core::types::U256,
        pub variable_borrow_index: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "ReserveUsedAsCollateralDisabled",
        abi = "ReserveUsedAsCollateralDisabled(address,address)"
    )]
    pub struct ReserveUsedAsCollateralDisabledFilter {
        #[ethevent(indexed)]
        pub reserve: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "ReserveUsedAsCollateralEnabled",
        abi = "ReserveUsedAsCollateralEnabled(address,address)"
    )]
    pub struct ReserveUsedAsCollateralEnabledFilter {
        #[ethevent(indexed)]
        pub reserve: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Swap", abi = "Swap(address,address,uint256)")]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub reserve: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers_core::types::Address,
        pub rate_mode: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Unpaused", abi = "Unpaused()")]
    pub struct UnpausedFilter();
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Withdraw", abi = "Withdraw(address,address,address,uint256)")]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub reserve: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ILendingPoolEvents {
        BorrowFilter(BorrowFilter),
        DepositFilter(DepositFilter),
        FlashLoanFilter(FlashLoanFilter),
        LiquidationCallFilter(LiquidationCallFilter),
        PausedFilter(PausedFilter),
        RebalanceStableBorrowRateFilter(RebalanceStableBorrowRateFilter),
        RepayFilter(RepayFilter),
        ReserveDataUpdatedFilter(ReserveDataUpdatedFilter),
        ReserveUsedAsCollateralDisabledFilter(ReserveUsedAsCollateralDisabledFilter),
        ReserveUsedAsCollateralEnabledFilter(ReserveUsedAsCollateralEnabledFilter),
        SwapFilter(SwapFilter),
        UnpausedFilter(UnpausedFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ethers_core::abi::Tokenizable for ILendingPoolEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BorrowFilter::from_token(token.clone()) {
                return Ok(ILendingPoolEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::from_token(token.clone()) {
                return Ok(ILendingPoolEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = FlashLoanFilter::from_token(token.clone()) {
                return Ok(ILendingPoolEvents::FlashLoanFilter(decoded));
            }
            if let Ok(decoded) = LiquidationCallFilter::from_token(token.clone()) {
                return Ok(ILendingPoolEvents::LiquidationCallFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::from_token(token.clone()) {
                return Ok(ILendingPoolEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = RebalanceStableBorrowRateFilter::from_token(token.clone()) {
                return Ok(ILendingPoolEvents::RebalanceStableBorrowRateFilter(decoded));
            }
            if let Ok(decoded) = RepayFilter::from_token(token.clone()) {
                return Ok(ILendingPoolEvents::RepayFilter(decoded));
            }
            if let Ok(decoded) = ReserveDataUpdatedFilter::from_token(token.clone()) {
                return Ok(ILendingPoolEvents::ReserveDataUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ReserveUsedAsCollateralDisabledFilter::from_token(token.clone()) {
                return Ok(ILendingPoolEvents::ReserveUsedAsCollateralDisabledFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ReserveUsedAsCollateralEnabledFilter::from_token(token.clone()) {
                return Ok(ILendingPoolEvents::ReserveUsedAsCollateralEnabledFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SwapFilter::from_token(token.clone()) {
                return Ok(ILendingPoolEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::from_token(token.clone()) {
                return Ok(ILendingPoolEvents::UnpausedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::from_token(token.clone()) {
                return Ok(ILendingPoolEvents::WithdrawFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                ILendingPoolEvents::BorrowFilter(element) => element.into_token(),
                ILendingPoolEvents::DepositFilter(element) => element.into_token(),
                ILendingPoolEvents::FlashLoanFilter(element) => element.into_token(),
                ILendingPoolEvents::LiquidationCallFilter(element) => element.into_token(),
                ILendingPoolEvents::PausedFilter(element) => element.into_token(),
                ILendingPoolEvents::RebalanceStableBorrowRateFilter(element) => {
                    element.into_token()
                }
                ILendingPoolEvents::RepayFilter(element) => element.into_token(),
                ILendingPoolEvents::ReserveDataUpdatedFilter(element) => element.into_token(),
                ILendingPoolEvents::ReserveUsedAsCollateralDisabledFilter(element) => {
                    element.into_token()
                }
                ILendingPoolEvents::ReserveUsedAsCollateralEnabledFilter(element) => {
                    element.into_token()
                }
                ILendingPoolEvents::SwapFilter(element) => element.into_token(),
                ILendingPoolEvents::UnpausedFilter(element) => element.into_token(),
                ILendingPoolEvents::WithdrawFilter(element) => element.into_token(),
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for ILendingPoolEvents {}
    impl ethers_contract::EthLogDecode for ILendingPoolEvents {
        fn decode_log(log: &ethers_core::abi::RawLog) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = FlashLoanFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::FlashLoanFilter(decoded));
            }
            if let Ok(decoded) = LiquidationCallFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::LiquidationCallFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = RebalanceStableBorrowRateFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::RebalanceStableBorrowRateFilter(decoded));
            }
            if let Ok(decoded) = RepayFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::RepayFilter(decoded));
            }
            if let Ok(decoded) = ReserveDataUpdatedFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::ReserveDataUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ReserveUsedAsCollateralDisabledFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::ReserveUsedAsCollateralDisabledFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ReserveUsedAsCollateralEnabledFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::ReserveUsedAsCollateralEnabledFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::UnpausedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(ILendingPoolEvents::WithdrawFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}

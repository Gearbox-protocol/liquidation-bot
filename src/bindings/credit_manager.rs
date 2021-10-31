pub use creditmanager_mod::*;
#[allow(clippy::too_many_arguments)]
mod creditmanager_mod {
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
    #[doc = "CreditManager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CREDITMANAGER_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_addressProvider\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_minAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_maxAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_maxLeverage\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_poolService\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditFilterAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_defaultSwapContract\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"AddCollateral\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"remainingFunds\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"CloseCreditAccount\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"target\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ExecuteOrder\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"IncreaseBorrowedAmount\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"liquidator\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"remainingFunds\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"LiquidateCreditAccount\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"minAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"maxAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"maxLeverage\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"feeInterest\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"feeLiquidation\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidationDiscount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"NewParameters\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"borrowAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"referralCode\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"OpenCreditAccount\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"Paused\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RepayCreditAccount\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"oldOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"TransferAccount\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"Unpaused\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"totalValue\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isLiquidated\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"_calcClosePayments\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_borrowedAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountToPool\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"remainingFunds\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"profit\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"loss\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"totalValue\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isLiquidated\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"borrowedAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"cumulativeIndexAtCreditAccountOpen_RAY\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"cumulativeIndexNow_RAY\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"_calcClosePaymentsPure\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_borrowedAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountToPool\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"remainingFunds\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"profit\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"loss\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"addCollateral\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"targetContract\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"approve\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isLiquidated\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"calcRepayAmount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"path\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amountOutMin\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.Exchange[]\",\n        \"name\": \"paths\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"closeCreditAccount\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"creditAccounts\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"creditFilter\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ICreditFilter\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"defaultSwapContract\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"target\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"executeOrder\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeInterest\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeLiquidation\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getCreditAccountOrRevert\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"hasOpenedCreditAccount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"increaseBorrowedAmount\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"force\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"liquidateCreditAccount\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"liquidationDiscount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"maxAmount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"maxLeverageFactor\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"minAmount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"minHealthFactor\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"leverageFactor\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"referralCode\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"openCreditAccount\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"pause\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"paused\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"poolService\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"targetContract\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"provideCreditAccountAllowance\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"repayCreditAccount\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"repayCreditAccountETH\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_minAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_maxAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_maxLeverageFactor\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_feeInterest\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_feeLiquidation\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_liquidationDiscount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setParams\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferAccountOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"underlyingToken\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"unpause\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"wethAddress\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"wethGateway\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct CreditManager<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for CreditManager<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for CreditManager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CreditManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> CreditManager<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), CREDITMANAGER_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `_calcClosePayments` (0x996329f8) function"]
        pub fn calc_close_payments(
            &self,
            credit_account: ethers_core::types::Address,
            total_value: ethers_core::types::U256,
            is_liquidated: bool,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [153, 99, 41, 248],
                    (credit_account, total_value, is_liquidated),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_calcClosePaymentsPure` (0xf8dbc6b6) function"]
        pub fn calc_close_payments_pure(
            &self,
            total_value: ethers_core::types::U256,
            is_liquidated: bool,
            borrowed_amount: ethers_core::types::U256,
            cumulative_index_at_credit_account_open_ray: ethers_core::types::U256,
            cumulative_index_now_ray: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [248, 219, 198, 182],
                    (
                        total_value,
                        is_liquidated,
                        borrowed_amount,
                        cumulative_index_at_credit_account_open_ray,
                        cumulative_index_now_ray,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addCollateral` (0x59781034) function"]
        pub fn add_collateral(
            &self,
            on_behalf_of: ethers_core::types::Address,
            token: ethers_core::types::Address,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 120, 16, 52], (on_behalf_of, token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x7e5465ba) function"]
        pub fn approve(
            &self,
            target_contract: ethers_core::types::Address,
            token: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([126, 84, 101, 186], (target_contract, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcRepayAmount` (0x3ce07355) function"]
        pub fn calc_repay_amount(
            &self,
            borrower: ethers_core::types::Address,
            is_liquidated: bool,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([60, 224, 115, 85], (borrower, is_liquidated))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `closeCreditAccount` (0xab114805) function"]
        pub fn close_credit_account(
            &self,
            to: ethers_core::types::Address,
            paths: Vec<(Vec<ethers_core::types::Address>, ethers_core::types::U256)>,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 17, 72, 5], (to, paths))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditAccounts` (0x055ee9b5) function"]
        pub fn credit_accounts(
            &self,
            p0: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([5, 94, 233, 181], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditFilter` (0xf93f515b) function"]
        pub fn credit_filter(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([249, 63, 81, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `defaultSwapContract` (0xe0c011b7) function"]
        pub fn default_swap_contract(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([224, 192, 17, 183], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeOrder` (0x6ce4074a) function"]
        pub fn execute_order(
            &self,
            borrower: ethers_core::types::Address,
            target: ethers_core::types::Address,
            data: Vec<u8>,
        ) -> ethers_contract::builders::ContractCall<M, Vec<u8>> {
            self.0
                .method_hash([108, 228, 7, 74], (borrower, target, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeInterest` (0x5e0b63d3) function"]
        pub fn fee_interest(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([94, 11, 99, 211], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeLiquidation` (0x3915ffaa) function"]
        pub fn fee_liquidation(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([57, 21, 255, 170], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditAccountOrRevert` (0xe958b704) function"]
        pub fn get_credit_account_or_revert(
            &self,
            borrower: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([233, 88, 183, 4], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasOpenedCreditAccount` (0x256ac915) function"]
        pub fn has_opened_credit_account(
            &self,
            borrower: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([37, 106, 201, 21], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseBorrowedAmount` (0x9efc60d0) function"]
        pub fn increase_borrowed_amount(
            &self,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 252, 96, 208], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidateCreditAccount` (0xa69a7dd6) function"]
        pub fn liquidate_credit_account(
            &self,
            borrower: ethers_core::types::Address,
            to: ethers_core::types::Address,
            force: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 154, 125, 214], (borrower, to, force))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidationDiscount` (0x8053fcbe) function"]
        pub fn liquidation_discount(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([128, 83, 252, 190], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxAmount` (0x5f48f393) function"]
        pub fn max_amount(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([95, 72, 243, 147], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxLeverageFactor` (0xb2c53a6c) function"]
        pub fn max_leverage_factor(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([178, 197, 58, 108], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minAmount` (0x9b2cb5d8) function"]
        pub fn min_amount(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([155, 44, 181, 216], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minHealthFactor` (0xe1b4264c) function"]
        pub fn min_health_factor(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([225, 180, 38, 76], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `openCreditAccount` (0x5288ba4b) function"]
        pub fn open_credit_account(
            &self,
            amount: ethers_core::types::U256,
            on_behalf_of: ethers_core::types::Address,
            leverage_factor: ethers_core::types::U256,
            referral_code: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [82, 136, 186, 75],
                    (amount, on_behalf_of, leverage_factor, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pause` (0x8456cb59) function"]
        pub fn pause(&self) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(&self) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `poolService` (0x570a7af2) function"]
        pub fn pool_service(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([87, 10, 122, 242], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `provideCreditAccountAllowance` (0x579122ab) function"]
        pub fn provide_credit_account_allowance(
            &self,
            credit_account: ethers_core::types::Address,
            target_contract: ethers_core::types::Address,
            token: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 145, 34, 171], (credit_account, target_contract, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayCreditAccount` (0xd692ba33) function"]
        pub fn repay_credit_account(
            &self,
            to: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 146, 186, 51], to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayCreditAccountETH` (0xa6eab5c2) function"]
        pub fn repay_credit_account_eth(
            &self,
            borrower: ethers_core::types::Address,
            to: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([166, 234, 181, 194], (borrower, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setParams` (0xebb39512) function"]
        pub fn set_params(
            &self,
            min_amount: ethers_core::types::U256,
            max_amount: ethers_core::types::U256,
            max_leverage_factor: ethers_core::types::U256,
            fee_interest: ethers_core::types::U256,
            fee_liquidation: ethers_core::types::U256,
            liquidation_discount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [235, 179, 149, 18],
                    (
                        min_amount,
                        max_amount,
                        max_leverage_factor,
                        fee_interest,
                        fee_liquidation,
                        liquidation_discount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferAccountOwnership` (0x5019e20a) function"]
        pub fn transfer_account_ownership(
            &self,
            new_owner: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 25, 226, 10], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlyingToken` (0x2495a599) function"]
        pub fn underlying_token(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([36, 149, 165, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpause` (0x3f4ba83a) function"]
        pub fn unpause(&self) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wethAddress` (0x4f0e0ef3) function"]
        pub fn weth_address(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([79, 14, 14, 243], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wethGateway` (0xc5e10eef) function"]
        pub fn weth_gateway(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([197, 225, 14, 239], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AddCollateral` event"]
        pub fn add_collateral_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, AddCollateralFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CloseCreditAccount` event"]
        pub fn close_credit_account_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, CloseCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExecuteOrder` event"]
        pub fn execute_order_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ExecuteOrderFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `IncreaseBorrowedAmount` event"]
        pub fn increase_borrowed_amount_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, IncreaseBorrowedAmountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LiquidateCreditAccount` event"]
        pub fn liquidate_credit_account_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, LiquidateCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewParameters` event"]
        pub fn new_parameters_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, NewParametersFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OpenCreditAccount` event"]
        pub fn open_credit_account_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, OpenCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(&self) -> ethers_contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RepayCreditAccount` event"]
        pub fn repay_credit_account_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, RepayCreditAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferAccount` event"]
        pub fn transfer_account_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, TransferAccountFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(&self) -> ethers_contract::builders::Event<M, UnpausedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, CreditManagerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "AddCollateral", abi = "AddCollateral(address,address,uint256)")]
    pub struct AddCollateralFilter {
        #[ethevent(indexed)]
        pub on_behalf_of: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub token: ethers_core::types::Address,
        pub value: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "CloseCreditAccount",
        abi = "CloseCreditAccount(address,address,uint256)"
    )]
    pub struct CloseCreditAccountFilter {
        #[ethevent(indexed)]
        pub owner: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers_core::types::Address,
        pub remaining_funds: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "ExecuteOrder", abi = "ExecuteOrder(address,address)")]
    pub struct ExecuteOrderFilter {
        #[ethevent(indexed)]
        pub borrower: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub target: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "IncreaseBorrowedAmount",
        abi = "IncreaseBorrowedAmount(address,uint256)"
    )]
    pub struct IncreaseBorrowedAmountFilter {
        #[ethevent(indexed)]
        pub borrower: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "LiquidateCreditAccount",
        abi = "LiquidateCreditAccount(address,address,uint256)"
    )]
    pub struct LiquidateCreditAccountFilter {
        #[ethevent(indexed)]
        pub owner: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub liquidator: ethers_core::types::Address,
        pub remaining_funds: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "NewParameters",
        abi = "NewParameters(uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct NewParametersFilter {
        pub min_amount: ethers_core::types::U256,
        pub max_amount: ethers_core::types::U256,
        pub max_leverage: ethers_core::types::U256,
        pub fee_interest: ethers_core::types::U256,
        pub fee_liquidation: ethers_core::types::U256,
        pub liquidation_discount: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "OpenCreditAccount",
        abi = "OpenCreditAccount(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct OpenCreditAccountFilter {
        #[ethevent(indexed)]
        pub sender: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub credit_account: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
        pub borrow_amount: ethers_core::types::U256,
        pub referral_code: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "RepayCreditAccount",
        abi = "RepayCreditAccount(address,address)"
    )]
    pub struct RepayCreditAccountFilter {
        #[ethevent(indexed)]
        pub owner: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "TransferAccount", abi = "TransferAccount(address,address)")]
    pub struct TransferAccountFilter {
        pub old_owner: ethers_core::types::Address,
        pub new_owner: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ethers_core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum CreditManagerEvents {
        AddCollateralFilter(AddCollateralFilter),
        CloseCreditAccountFilter(CloseCreditAccountFilter),
        ExecuteOrderFilter(ExecuteOrderFilter),
        IncreaseBorrowedAmountFilter(IncreaseBorrowedAmountFilter),
        LiquidateCreditAccountFilter(LiquidateCreditAccountFilter),
        NewParametersFilter(NewParametersFilter),
        OpenCreditAccountFilter(OpenCreditAccountFilter),
        PausedFilter(PausedFilter),
        RepayCreditAccountFilter(RepayCreditAccountFilter),
        TransferAccountFilter(TransferAccountFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers_core::abi::Tokenizable for CreditManagerEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddCollateralFilter::from_token(token.clone()) {
                return Ok(CreditManagerEvents::AddCollateralFilter(decoded));
            }
            if let Ok(decoded) = CloseCreditAccountFilter::from_token(token.clone()) {
                return Ok(CreditManagerEvents::CloseCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = ExecuteOrderFilter::from_token(token.clone()) {
                return Ok(CreditManagerEvents::ExecuteOrderFilter(decoded));
            }
            if let Ok(decoded) = IncreaseBorrowedAmountFilter::from_token(token.clone()) {
                return Ok(CreditManagerEvents::IncreaseBorrowedAmountFilter(decoded));
            }
            if let Ok(decoded) = LiquidateCreditAccountFilter::from_token(token.clone()) {
                return Ok(CreditManagerEvents::LiquidateCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = NewParametersFilter::from_token(token.clone()) {
                return Ok(CreditManagerEvents::NewParametersFilter(decoded));
            }
            if let Ok(decoded) = OpenCreditAccountFilter::from_token(token.clone()) {
                return Ok(CreditManagerEvents::OpenCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::from_token(token.clone()) {
                return Ok(CreditManagerEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = RepayCreditAccountFilter::from_token(token.clone()) {
                return Ok(CreditManagerEvents::RepayCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = TransferAccountFilter::from_token(token.clone()) {
                return Ok(CreditManagerEvents::TransferAccountFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::from_token(token.clone()) {
                return Ok(CreditManagerEvents::UnpausedFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                CreditManagerEvents::AddCollateralFilter(element) => element.into_token(),
                CreditManagerEvents::CloseCreditAccountFilter(element) => element.into_token(),
                CreditManagerEvents::ExecuteOrderFilter(element) => element.into_token(),
                CreditManagerEvents::IncreaseBorrowedAmountFilter(element) => element.into_token(),
                CreditManagerEvents::LiquidateCreditAccountFilter(element) => element.into_token(),
                CreditManagerEvents::NewParametersFilter(element) => element.into_token(),
                CreditManagerEvents::OpenCreditAccountFilter(element) => element.into_token(),
                CreditManagerEvents::PausedFilter(element) => element.into_token(),
                CreditManagerEvents::RepayCreditAccountFilter(element) => element.into_token(),
                CreditManagerEvents::TransferAccountFilter(element) => element.into_token(),
                CreditManagerEvents::UnpausedFilter(element) => element.into_token(),
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for CreditManagerEvents {}
    impl ethers_contract::EthLogDecode for CreditManagerEvents {
        fn decode_log(log: &ethers_core::abi::RawLog) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddCollateralFilter::decode_log(log) {
                return Ok(CreditManagerEvents::AddCollateralFilter(decoded));
            }
            if let Ok(decoded) = CloseCreditAccountFilter::decode_log(log) {
                return Ok(CreditManagerEvents::CloseCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = ExecuteOrderFilter::decode_log(log) {
                return Ok(CreditManagerEvents::ExecuteOrderFilter(decoded));
            }
            if let Ok(decoded) = IncreaseBorrowedAmountFilter::decode_log(log) {
                return Ok(CreditManagerEvents::IncreaseBorrowedAmountFilter(decoded));
            }
            if let Ok(decoded) = LiquidateCreditAccountFilter::decode_log(log) {
                return Ok(CreditManagerEvents::LiquidateCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = NewParametersFilter::decode_log(log) {
                return Ok(CreditManagerEvents::NewParametersFilter(decoded));
            }
            if let Ok(decoded) = OpenCreditAccountFilter::decode_log(log) {
                return Ok(CreditManagerEvents::OpenCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(CreditManagerEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = RepayCreditAccountFilter::decode_log(log) {
                return Ok(CreditManagerEvents::RepayCreditAccountFilter(decoded));
            }
            if let Ok(decoded) = TransferAccountFilter::decode_log(log) {
                return Ok(CreditManagerEvents::TransferAccountFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(CreditManagerEvents::UnpausedFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}

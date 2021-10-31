pub use creditfilter_mod::*;
#[allow(clippy::too_many_arguments)]
mod creditfilter_mod {
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
    #[doc = "CreditFilter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CREDITFILTER_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_addressProvider\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_underlyingToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"protocol\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"adapter\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ContractAllowed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"protocol\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ContractForbidden\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"chiThreshold\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"fastCheckDelay\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"NewFastCheckParameters\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"Paused\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidityThreshold\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"TokenAllowed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"TransferAccountAllowed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"pugin\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"TransferPluginAllowed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"Unpaused\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"_allowedTokensMap\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"addressProvider\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract AddressProvider\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"targetContract\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"adapter\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowContract\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"plugin\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"allowPlugin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidationThreshold\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"allowToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowanceForAccountTransfers\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowedAdapters\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"i\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"allowedContracts\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"allowedContractsCount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowedPlugins\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"allowedTokens\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"allowedTokensCount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"state\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"approveAccountTransfers\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"calcCreditAccountAccruedInterest\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"calcCreditAccountHealthFactor\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"percentage\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"times\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"calcMaxPossibleDrop\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"calcThresholdWeightedValue\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"total\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"calcTotalValue\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"total\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"checkAndEnableToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenIn\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenOut\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountIn\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountOut\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"checkCollateralChange\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amountIn\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amountOut\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"tokenIn\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"tokenOut\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"checkMultiTokenCollateral\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"chiThreshold\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"connectCreditManager\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"contractToAdapter\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"creditManager\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"enabledTokens\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"fastCheckCounter\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"targetContract\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"forbidContract\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"forbidToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"id\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getCreditAccountTokenById\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"balance\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tv\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tvw\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"hfCheckInterval\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"initEnabledTokens\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isTokenAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"liquidationThresholds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"pause\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"paused\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"poolService\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"priceOracle\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"revertIfAccountTransferIsNotAllowed\",\n    \"outputs\": [],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"minHealthFactor\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"revertIfCantIncreaseBorrowing\",\n    \"outputs\": [],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"revertIfTokenNotAllowed\",\n    \"outputs\": [],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_chiThreshold\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_hfCheckInterval\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setFastCheckParameters\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"tokenMasksMap\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"underlyingToken\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"unpause\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"updateUnderlyingTokenLiquidationThreshold\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"wethAddress\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct CreditFilter<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for CreditFilter<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for CreditFilter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CreditFilter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> CreditFilter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), CREDITFILTER_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `_allowedTokensMap` (0xb2d0d86b) function"]
        pub fn allowed_tokens_map(
            &self,
            p0: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([178, 208, 216, 107], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addressProvider` (0x2954018c) function"]
        pub fn address_provider(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([41, 84, 1, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowContract` (0x7bccacee) function"]
        pub fn allow_contract(
            &self,
            target_contract: ethers_core::types::Address,
            adapter: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 204, 172, 238], (target_contract, adapter))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowPlugin` (0x2e2986dd) function"]
        pub fn allow_plugin(
            &self,
            plugin: ethers_core::types::Address,
            state: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 41, 134, 221], (plugin, state))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowToken` (0xa147c6c6) function"]
        pub fn allow_token(
            &self,
            token: ethers_core::types::Address,
            liquidation_threshold: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 71, 198, 198], (token, liquidation_threshold))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowanceForAccountTransfers` (0x5a29be45) function"]
        pub fn allowance_for_account_transfers(
            &self,
            from: ethers_core::types::Address,
            to: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([90, 41, 190, 69], (from, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedAdapters` (0x3bdfe4f5) function"]
        pub fn allowed_adapters(
            &self,
            p0: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([59, 223, 228, 245], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedContracts` (0x5094cb4f) function"]
        pub fn allowed_contracts(
            &self,
            i: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([80, 148, 203, 79], i)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedContractsCount` (0x50e036ff) function"]
        pub fn allowed_contracts_count(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([80, 224, 54, 255], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedPlugins` (0x5f598edd) function"]
        pub fn allowed_plugins(
            &self,
            p0: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([95, 89, 142, 221], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedTokens` (0x5e5f2e26) function"]
        pub fn allowed_tokens(
            &self,
            p0: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([94, 95, 46, 38], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedTokensCount` (0x20a05ff7) function"]
        pub fn allowed_tokens_count(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([32, 160, 95, 247], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approveAccountTransfers` (0x5f27212a) function"]
        pub fn approve_account_transfers(
            &self,
            from: ethers_core::types::Address,
            state: bool,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 39, 33, 42], (from, state))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcCreditAccountAccruedInterest` (0x3192195c) function"]
        pub fn calc_credit_account_accrued_interest(
            &self,
            credit_account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([49, 146, 25, 92], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcCreditAccountHealthFactor` (0xdfd59465) function"]
        pub fn calc_credit_account_health_factor(
            &self,
            credit_account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([223, 213, 148, 101], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcMaxPossibleDrop` (0xb3c61943) function"]
        pub fn calc_max_possible_drop(
            &self,
            percentage: ethers_core::types::U256,
            times: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([179, 198, 25, 67], (percentage, times))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcThresholdWeightedValue` (0x90b1300a) function"]
        pub fn calc_threshold_weighted_value(
            &self,
            credit_account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([144, 177, 48, 10], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcTotalValue` (0xc7de38a6) function"]
        pub fn calc_total_value(
            &self,
            credit_account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([199, 222, 56, 166], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkAndEnableToken` (0x51e3f160) function"]
        pub fn check_and_enable_token(
            &self,
            credit_account: ethers_core::types::Address,
            token: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 227, 241, 96], (credit_account, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkCollateralChange` (0xe1c8ef0d) function"]
        pub fn check_collateral_change(
            &self,
            credit_account: ethers_core::types::Address,
            token_in: ethers_core::types::Address,
            token_out: ethers_core::types::Address,
            amount_in: ethers_core::types::U256,
            amount_out: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [225, 200, 239, 13],
                    (credit_account, token_in, token_out, amount_in, amount_out),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkMultiTokenCollateral` (0x7e4a6863) function"]
        pub fn check_multi_token_collateral(
            &self,
            credit_account: ethers_core::types::Address,
            amount_in: Vec<ethers_core::types::U256>,
            amount_out: Vec<ethers_core::types::U256>,
            token_in: Vec<ethers_core::types::Address>,
            token_out: Vec<ethers_core::types::Address>,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [126, 74, 104, 99],
                    (credit_account, amount_in, amount_out, token_in, token_out),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `chiThreshold` (0x47dedfc9) function"]
        pub fn chi_threshold(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([71, 222, 223, 201], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `connectCreditManager` (0xcf33d955) function"]
        pub fn connect_credit_manager(
            &self,
            credit_manager: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 51, 217, 85], credit_manager)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `contractToAdapter` (0xfdd57645) function"]
        pub fn contract_to_adapter(
            &self,
            p0: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([253, 213, 118, 69], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditManager` (0xc12c21c0) function"]
        pub fn credit_manager(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([193, 44, 33, 192], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enabledTokens` (0xb451cecc) function"]
        pub fn enabled_tokens(
            &self,
            p0: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([180, 81, 206, 204], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fastCheckCounter` (0x4cba294a) function"]
        pub fn fast_check_counter(
            &self,
            p0: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([76, 186, 41, 74], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forbidContract` (0x52438e54) function"]
        pub fn forbid_contract(
            &self,
            target_contract: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([82, 67, 142, 84], target_contract)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forbidToken` (0x24147708) function"]
        pub fn forbid_token(
            &self,
            token: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 20, 119, 8], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditAccountTokenById` (0xaf0a6502) function"]
        pub fn get_credit_account_token_by_id(
            &self,
            credit_account: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash([175, 10, 101, 2], (credit_account, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hfCheckInterval` (0xe6dee2cc) function"]
        pub fn hf_check_interval(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([230, 222, 226, 204], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initEnabledTokens` (0xe54fe9c8) function"]
        pub fn init_enabled_tokens(
            &self,
            credit_account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 79, 233, 200], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isTokenAllowed` (0xf9eaee0d) function"]
        pub fn is_token_allowed(
            &self,
            token: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([249, 234, 238, 13], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidationThresholds` (0x78327438) function"]
        pub fn liquidation_thresholds(
            &self,
            p0: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([120, 50, 116, 56], p0)
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
        #[doc = "Calls the contract's `priceOracle` (0x2630c12f) function"]
        pub fn price_oracle(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([38, 48, 193, 47], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revertIfAccountTransferIsNotAllowed` (0x3b00ae70) function"]
        pub fn revert_if_account_transfer_is_not_allowed(
            &self,
            owner: ethers_core::types::Address,
            new_owner: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 0, 174, 112], (owner, new_owner))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revertIfCantIncreaseBorrowing` (0xa5757517) function"]
        pub fn revert_if_cant_increase_borrowing(
            &self,
            credit_account: ethers_core::types::Address,
            min_health_factor: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 117, 117, 23], (credit_account, min_health_factor))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revertIfTokenNotAllowed` (0x7dd0ba82) function"]
        pub fn revert_if_token_not_allowed(
            &self,
            token: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([125, 208, 186, 130], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFastCheckParameters` (0x62061c6d) function"]
        pub fn set_fast_check_parameters(
            &self,
            chi_threshold: ethers_core::types::U256,
            hf_check_interval: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 6, 28, 109], (chi_threshold, hf_check_interval))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenMasksMap` (0xf67c5bd0) function"]
        pub fn token_masks_map(
            &self,
            p0: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([246, 124, 91, 208], p0)
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
        #[doc = "Calls the contract's `updateUnderlyingTokenLiquidationThreshold` (0x40631828) function"]
        pub fn update_underlying_token_liquidation_threshold(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 99, 24, 40], ())
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
        #[doc = "Gets the contract's `ContractAllowed` event"]
        pub fn contract_allowed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ContractAllowedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ContractForbidden` event"]
        pub fn contract_forbidden_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ContractForbiddenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewFastCheckParameters` event"]
        pub fn new_fast_check_parameters_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, NewFastCheckParametersFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(&self) -> ethers_contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenAllowed` event"]
        pub fn token_allowed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, TokenAllowedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferAccountAllowed` event"]
        pub fn transfer_account_allowed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, TransferAccountAllowedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferPluginAllowed` event"]
        pub fn transfer_plugin_allowed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, TransferPluginAllowedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(&self) -> ethers_contract::builders::Event<M, UnpausedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, CreditFilterEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "ContractAllowed", abi = "ContractAllowed(address,address)")]
    pub struct ContractAllowedFilter {
        #[ethevent(indexed)]
        pub protocol: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub adapter: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "ContractForbidden", abi = "ContractForbidden(address)")]
    pub struct ContractForbiddenFilter {
        #[ethevent(indexed)]
        pub protocol: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "NewFastCheckParameters",
        abi = "NewFastCheckParameters(uint256,uint256)"
    )]
    pub struct NewFastCheckParametersFilter {
        pub chi_threshold: ethers_core::types::U256,
        pub fast_check_delay: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "TokenAllowed", abi = "TokenAllowed(address,uint256)")]
    pub struct TokenAllowedFilter {
        #[ethevent(indexed)]
        pub token: ethers_core::types::Address,
        pub liquidity_threshold: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "TransferAccountAllowed",
        abi = "TransferAccountAllowed(address,address,bool)"
    )]
    pub struct TransferAccountAllowedFilter {
        #[ethevent(indexed)]
        pub from: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers_core::types::Address,
        pub state: bool,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "TransferPluginAllowed",
        abi = "TransferPluginAllowed(address,bool)"
    )]
    pub struct TransferPluginAllowedFilter {
        #[ethevent(indexed)]
        pub pugin: ethers_core::types::Address,
        pub state: bool,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ethers_core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum CreditFilterEvents {
        ContractAllowedFilter(ContractAllowedFilter),
        ContractForbiddenFilter(ContractForbiddenFilter),
        NewFastCheckParametersFilter(NewFastCheckParametersFilter),
        PausedFilter(PausedFilter),
        TokenAllowedFilter(TokenAllowedFilter),
        TransferAccountAllowedFilter(TransferAccountAllowedFilter),
        TransferPluginAllowedFilter(TransferPluginAllowedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers_core::abi::Tokenizable for CreditFilterEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ContractAllowedFilter::from_token(token.clone()) {
                return Ok(CreditFilterEvents::ContractAllowedFilter(decoded));
            }
            if let Ok(decoded) = ContractForbiddenFilter::from_token(token.clone()) {
                return Ok(CreditFilterEvents::ContractForbiddenFilter(decoded));
            }
            if let Ok(decoded) = NewFastCheckParametersFilter::from_token(token.clone()) {
                return Ok(CreditFilterEvents::NewFastCheckParametersFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::from_token(token.clone()) {
                return Ok(CreditFilterEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = TokenAllowedFilter::from_token(token.clone()) {
                return Ok(CreditFilterEvents::TokenAllowedFilter(decoded));
            }
            if let Ok(decoded) = TransferAccountAllowedFilter::from_token(token.clone()) {
                return Ok(CreditFilterEvents::TransferAccountAllowedFilter(decoded));
            }
            if let Ok(decoded) = TransferPluginAllowedFilter::from_token(token.clone()) {
                return Ok(CreditFilterEvents::TransferPluginAllowedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::from_token(token.clone()) {
                return Ok(CreditFilterEvents::UnpausedFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                CreditFilterEvents::ContractAllowedFilter(element) => element.into_token(),
                CreditFilterEvents::ContractForbiddenFilter(element) => element.into_token(),
                CreditFilterEvents::NewFastCheckParametersFilter(element) => element.into_token(),
                CreditFilterEvents::PausedFilter(element) => element.into_token(),
                CreditFilterEvents::TokenAllowedFilter(element) => element.into_token(),
                CreditFilterEvents::TransferAccountAllowedFilter(element) => element.into_token(),
                CreditFilterEvents::TransferPluginAllowedFilter(element) => element.into_token(),
                CreditFilterEvents::UnpausedFilter(element) => element.into_token(),
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for CreditFilterEvents {}
    impl ethers_contract::EthLogDecode for CreditFilterEvents {
        fn decode_log(log: &ethers_core::abi::RawLog) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ContractAllowedFilter::decode_log(log) {
                return Ok(CreditFilterEvents::ContractAllowedFilter(decoded));
            }
            if let Ok(decoded) = ContractForbiddenFilter::decode_log(log) {
                return Ok(CreditFilterEvents::ContractForbiddenFilter(decoded));
            }
            if let Ok(decoded) = NewFastCheckParametersFilter::decode_log(log) {
                return Ok(CreditFilterEvents::NewFastCheckParametersFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(CreditFilterEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = TokenAllowedFilter::decode_log(log) {
                return Ok(CreditFilterEvents::TokenAllowedFilter(decoded));
            }
            if let Ok(decoded) = TransferAccountAllowedFilter::decode_log(log) {
                return Ok(CreditFilterEvents::TransferAccountAllowedFilter(decoded));
            }
            if let Ok(decoded) = TransferPluginAllowedFilter::decode_log(log) {
                return Ok(CreditFilterEvents::TransferPluginAllowedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(CreditFilterEvents::UnpausedFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}

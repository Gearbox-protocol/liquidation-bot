pub use datacompressor_mod::*;
#[allow(clippy::too_many_arguments)]
mod datacompressor_mod {
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
    #[doc = "DataCompressor was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static DATACOMPRESSOR_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_addressProvider\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"WETHToken\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"addressProvider\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract AddressProvider\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"borrowedAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"calcExpectedAtOpenHf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"balances\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"name\": \"calcExpectedHf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"contractsRegister\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ContractsRegister\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_allowedContract\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getAdapter\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getCreditAccountData\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"borrower\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"inUse\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"creditManager\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"underlyingToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowedAmountPlusInterest\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"totalValue\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"healthFactor\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowRate\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"balance\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DataTypes.TokenBalance[]\",\n            \"name\": \"balances\",\n            \"type\": \"tuple[]\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.CreditAccountData\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getCreditAccountDataExtended\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"borrower\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"inUse\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"creditManager\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"underlyingToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowedAmountPlusInterest\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"totalValue\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"healthFactor\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowRate\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"balance\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DataTypes.TokenBalance[]\",\n            \"name\": \"balances\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"repayAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"liquidationAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"canBeClosed\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowedAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"cumulativeIndexAtOpen\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"since\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.CreditAccountDataExtended\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getCreditAccountList\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"borrower\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"inUse\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"creditManager\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"underlyingToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowedAmountPlusInterest\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"totalValue\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"healthFactor\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowRate\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"balance\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct DataTypes.TokenBalance[]\",\n            \"name\": \"balances\",\n            \"type\": \"tuple[]\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.CreditAccountData[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getCreditManagerData\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"hasAccount\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"underlyingToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isWETH\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"canBorrow\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowRate\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"minAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"maxAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"maxLeverageFactor\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"availableLiquidity\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"allowedTokens\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"allowedContract\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"adapter\",\n                \"type\": \"address\"\n              }\n            ],\n            \"internalType\": \"struct DataTypes.ContractAdapter[]\",\n            \"name\": \"adapters\",\n            \"type\": \"tuple[]\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.CreditManagerData\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getCreditManagersList\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"hasAccount\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"underlyingToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isWETH\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"canBorrow\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowRate\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"minAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"maxAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"maxLeverageFactor\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"availableLiquidity\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"allowedTokens\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"allowedContract\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"adapter\",\n                \"type\": \"address\"\n              }\n            ],\n            \"internalType\": \"struct DataTypes.ContractAdapter[]\",\n            \"name\": \"adapters\",\n            \"type\": \"tuple[]\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.CreditManagerData[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_pool\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getPoolData\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isWETH\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"underlyingToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"dieselToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"linearCumulativeIndex\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"availableLiquidity\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"expectedLiquidity\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"expectedLiquidityLimit\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"totalBorrowed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"depositAPY_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowAPY_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"dieselRate_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"withdrawFee\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"cumulativeIndex_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timestampLU\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.PoolData\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getPoolsList\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isWETH\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"underlyingToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"dieselToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"linearCumulativeIndex\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"availableLiquidity\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"expectedLiquidity\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"expectedLiquidityLimit\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"totalBorrowed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"depositAPY_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowAPY_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"dieselRate_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"withdrawFee\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"cumulativeIndex_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timestampLU\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.PoolData[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"addr\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"getTokenData\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"symbol\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"decimals\",\n            \"type\": \"uint8\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.TokenInfo[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"hasOpenedCreditAccount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct DataCompressor<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for DataCompressor<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for DataCompressor<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DataCompressor))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> DataCompressor<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), DATACOMPRESSOR_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `WETHToken` (0x4b2f336d) function"]
        pub fn weth_token(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([75, 47, 51, 109], ())
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
        #[doc = "Calls the contract's `calcExpectedAtOpenHf` (0x39595cf8) function"]
        pub fn calc_expected_at_open_hf(
            &self,
            credit_manager: ethers_core::types::Address,
            token: ethers_core::types::Address,
            amount: ethers_core::types::U256,
            borrowed_amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash(
                    [57, 89, 92, 248],
                    (credit_manager, token, amount, borrowed_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcExpectedHf` (0xba3b7345) function"]
        pub fn calc_expected_hf(
            &self,
            credit_manager: ethers_core::types::Address,
            borrower: ethers_core::types::Address,
            balances: Vec<ethers_core::types::U256>,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([186, 59, 115, 69], (credit_manager, borrower, balances))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `contractsRegister` (0x7a0c7b21) function"]
        pub fn contracts_register(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([122, 12, 123, 33], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAdapter` (0x4c472fc9) function"]
        pub fn get_adapter(
            &self,
            credit_manager: ethers_core::types::Address,
            allowed_contract: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([76, 71, 47, 201], (credit_manager, allowed_contract))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditAccountData` (0x0dbd616d) function"]
        pub fn get_credit_account_data(
            &self,
            credit_manager: ethers_core::types::Address,
            borrower: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::Address,
                ethers_core::types::Address,
                bool,
                ethers_core::types::Address,
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                Vec<(ethers_core::types::Address, ethers_core::types::U256)>,
            ),
        > {
            self.0
                .method_hash([13, 189, 97, 109], (credit_manager, borrower))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditAccountDataExtended` (0x191482d4) function"]
        pub fn get_credit_account_data_extended(
            &self,
            credit_manager: ethers_core::types::Address,
            borrower: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::Address,
                ethers_core::types::Address,
                bool,
                ethers_core::types::Address,
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                Vec<(ethers_core::types::Address, ethers_core::types::U256)>,
                ethers_core::types::U256,
                ethers_core::types::U256,
                bool,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash([25, 20, 130, 212], (credit_manager, borrower))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditAccountList` (0xa80deda3) function"]
        pub fn get_credit_account_list(
            &self,
            borrower: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
                ethers_core::types::Address,
                ethers_core::types::Address,
                bool,
                ethers_core::types::Address,
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                Vec<(ethers_core::types::Address, ethers_core::types::U256)>,
            )>,
        > {
            self.0
                .method_hash([168, 13, 237, 163], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditManagerData` (0xb10b074e) function"]
        pub fn get_credit_manager_data(
            &self,
            credit_manager: ethers_core::types::Address,
            borrower: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::Address,
                bool,
                ethers_core::types::Address,
                bool,
                bool,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                Vec<ethers_core::types::Address>,
                Vec<(ethers_core::types::Address, ethers_core::types::Address)>,
            ),
        > {
            self.0
                .method_hash([177, 11, 7, 78], (credit_manager, borrower))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditManagersList` (0xb8169039) function"]
        pub fn get_credit_managers_list(
            &self,
            borrower: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
                ethers_core::types::Address,
                bool,
                ethers_core::types::Address,
                bool,
                bool,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                Vec<ethers_core::types::Address>,
                Vec<(ethers_core::types::Address, ethers_core::types::Address)>,
            )>,
        > {
            self.0
                .method_hash([184, 22, 144, 57], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolData` (0x13d21cdf) function"]
        pub fn get_pool_data(
            &self,
            pool: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::Address,
                bool,
                ethers_core::types::Address,
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash([19, 210, 28, 223], pool)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolsList` (0x1bcd8fc0) function"]
        pub fn get_pools_list(
            &self,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
                ethers_core::types::Address,
                bool,
                ethers_core::types::Address,
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            )>,
        > {
            self.0
                .method_hash([27, 205, 143, 192], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTokenData` (0xbf2eb19e) function"]
        pub fn get_token_data(
            &self,
            addr: Vec<ethers_core::types::Address>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(ethers_core::types::Address, String, u8)>,
        > {
            self.0
                .method_hash([191, 46, 177, 158], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasOpenedCreditAccount` (0xfc9914cb) function"]
        pub fn has_opened_credit_account(
            &self,
            credit_manager: ethers_core::types::Address,
            borrower: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([252, 153, 20, 203], (credit_manager, borrower))
                .expect("method not found (this should never happen)")
        }
    }
}

pub use i_data_compressor::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_data_compressor {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "IDataCompressor was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n  \n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_allowedContract\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getAdapter\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"adapter\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getCreditAccountData\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"borrower\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"inUse\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"creditManager\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"underlying\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowedAmountPlusInterest\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowedAmountPlusInterestAndFees\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"totalValue\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"healthFactor\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowRate\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"balance\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bool\",\n                \"name\": \"isAllowed\",\n                \"type\": \"bool\"\n              },\n              {\n                \"internalType\": \"bool\",\n                \"name\": \"isEnabled\",\n                \"type\": \"bool\"\n              }\n            ],\n            \"internalType\": \"struct TokenBalance[]\",\n            \"name\": \"balances\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"repayAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"liquidationAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"canBeClosed\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowedAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"cumulativeIndexAtOpen\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"since\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"version\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"enabledTokenMask\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct CreditAccountData\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getCreditAccountList\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"borrower\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"inUse\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"creditManager\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"underlying\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowedAmountPlusInterest\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowedAmountPlusInterestAndFees\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"totalValue\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"healthFactor\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowRate\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"balance\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bool\",\n                \"name\": \"isAllowed\",\n                \"type\": \"bool\"\n              },\n              {\n                \"internalType\": \"bool\",\n                \"name\": \"isEnabled\",\n                \"type\": \"bool\"\n              }\n            ],\n            \"internalType\": \"struct TokenBalance[]\",\n            \"name\": \"balances\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"repayAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"liquidationAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"canBeClosed\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowedAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"cumulativeIndexAtOpen\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"since\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"version\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"enabledTokenMask\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct CreditAccountData[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getCreditManagerData\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"underlying\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"pool\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isWETH\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"canBorrow\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowRate\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"minAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"maxAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"maxLeverageFactor\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"availableLiquidity\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"collateralTokens\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"allowedContract\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"adapter\",\n                \"type\": \"address\"\n              }\n            ],\n            \"internalType\": \"struct ContractAdapter[]\",\n            \"name\": \"adapters\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"internalType\": \"uint256[]\",\n            \"name\": \"liquidationThresholds\",\n            \"type\": \"uint256[]\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"version\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"creditFacade\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"creditConfigurator\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isDegenMode\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"degenNFT\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isIncreaseDebtForbidden\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"forbiddenTokenMask\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"maxEnabledTokensLength\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint16\",\n            \"name\": \"feeInterest\",\n            \"type\": \"uint16\"\n          },\n          {\n            \"internalType\": \"uint16\",\n            \"name\": \"feeLiquidation\",\n            \"type\": \"uint16\"\n          },\n          {\n            \"internalType\": \"uint16\",\n            \"name\": \"liquidationDiscount\",\n            \"type\": \"uint16\"\n          },\n          {\n            \"internalType\": \"uint16\",\n            \"name\": \"feeLiquidationExpired\",\n            \"type\": \"uint16\"\n          },\n          {\n            \"internalType\": \"uint16\",\n            \"name\": \"liquidationDiscountExpired\",\n            \"type\": \"uint16\"\n          }\n        ],\n        \"internalType\": \"struct CreditManagerData\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getCreditManagersList\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"underlying\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"pool\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isWETH\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"canBorrow\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowRate\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"minAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"maxAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"maxLeverageFactor\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"availableLiquidity\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"collateralTokens\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"allowedContract\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"adapter\",\n                \"type\": \"address\"\n              }\n            ],\n            \"internalType\": \"struct ContractAdapter[]\",\n            \"name\": \"adapters\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"internalType\": \"uint256[]\",\n            \"name\": \"liquidationThresholds\",\n            \"type\": \"uint256[]\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"version\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"creditFacade\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"creditConfigurator\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isDegenMode\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"degenNFT\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isIncreaseDebtForbidden\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"forbiddenTokenMask\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"maxEnabledTokensLength\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint16\",\n            \"name\": \"feeInterest\",\n            \"type\": \"uint16\"\n          },\n          {\n            \"internalType\": \"uint16\",\n            \"name\": \"feeLiquidation\",\n            \"type\": \"uint16\"\n          },\n          {\n            \"internalType\": \"uint16\",\n            \"name\": \"liquidationDiscount\",\n            \"type\": \"uint16\"\n          },\n          {\n            \"internalType\": \"uint16\",\n            \"name\": \"feeLiquidationExpired\",\n            \"type\": \"uint16\"\n          },\n          {\n            \"internalType\": \"uint16\",\n            \"name\": \"liquidationDiscountExpired\",\n            \"type\": \"uint16\"\n          }\n        ],\n        \"internalType\": \"struct CreditManagerData[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_pool\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getPoolData\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isWETH\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"underlying\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"dieselToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"linearCumulativeIndex\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"availableLiquidity\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"expectedLiquidity\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"expectedLiquidityLimit\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"totalBorrowed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"depositAPY_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowAPY_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"dieselRate_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"withdrawFee\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"cumulativeIndex_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timestampLU\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"version\",\n            \"type\": \"uint8\"\n          }\n        ],\n        \"internalType\": \"struct PoolData\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getPoolsList\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"addr\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"isWETH\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"underlying\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"dieselToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"linearCumulativeIndex\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"availableLiquidity\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"expectedLiquidity\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"expectedLiquidityLimit\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"totalBorrowed\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"depositAPY_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"borrowAPY_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"dieselRate_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"withdrawFee\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"cumulativeIndex_RAY\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"timestampLU\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"version\",\n            \"type\": \"uint8\"\n          }\n        ],\n        \"internalType\": \"struct PoolData[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"hasOpenedCreditAccount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"version\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IDATACOMPRESSOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IDataCompressor<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IDataCompressor<M> {
        fn clone(&self) -> Self {
            IDataCompressor(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IDataCompressor<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IDataCompressor<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IDataCompressor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IDataCompressor<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IDATACOMPRESSOR_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `getAdapter` (0x4c472fc9) function"]
        pub fn get_adapter(
            &self,
            credit_manager: ethers::core::types::Address,
            allowed_contract: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([76, 71, 47, 201], (credit_manager, allowed_contract))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditAccountData` (0x0dbd616d) function"]
        pub fn get_credit_account_data(
            &self,
            credit_manager: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, CreditAccountData> {
            self.0
                .method_hash([13, 189, 97, 109], (credit_manager, borrower))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditAccountList` (0xa80deda3) function"]
        pub fn get_credit_account_list(
            &self,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<CreditAccountData>>
        {
            self.0
                .method_hash([168, 13, 237, 163], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditManagerData` (0xae093f3f) function"]
        pub fn get_credit_manager_data(
            &self,
            credit_manager: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, CreditManagerData> {
            self.0
                .method_hash([174, 9, 63, 63], credit_manager)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditManagersList` (0x663b8fdb) function"]
        pub fn get_credit_managers_list(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<CreditManagerData>>
        {
            self.0
                .method_hash([102, 59, 143, 219], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolData` (0x13d21cdf) function"]
        pub fn get_pool_data(
            &self,
            pool: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, PoolData> {
            self.0
                .method_hash([19, 210, 28, 223], pool)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolsList` (0x1bcd8fc0) function"]
        pub fn get_pools_list(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<PoolData>> {
            self.0
                .method_hash([27, 205, 143, 192], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasOpenedCreditAccount` (0xfc9914cb) function"]
        pub fn has_opened_credit_account(
            &self,
            credit_manager: ethers::core::types::Address,
            borrower: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([252, 153, 20, 203], (credit_manager, borrower))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IDataCompressor<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `getAdapter` function with signature `getAdapter(address,address)` and selector `[76, 71, 47, 201]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getAdapter", abi = "getAdapter(address,address)")]
    pub struct GetAdapterCall {
        pub credit_manager: ethers::core::types::Address,
        pub allowed_contract: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getCreditAccountData` function with signature `getCreditAccountData(address,address)` and selector `[13, 189, 97, 109]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getCreditAccountData",
        abi = "getCreditAccountData(address,address)"
    )]
    pub struct GetCreditAccountDataCall {
        pub credit_manager: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getCreditAccountList` function with signature `getCreditAccountList(address)` and selector `[168, 13, 237, 163]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCreditAccountList", abi = "getCreditAccountList(address)")]
    pub struct GetCreditAccountListCall {
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getCreditManagerData` function with signature `getCreditManagerData(address)` and selector `[174, 9, 63, 63]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCreditManagerData", abi = "getCreditManagerData(address)")]
    pub struct GetCreditManagerDataCall {
        pub credit_manager: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getCreditManagersList` function with signature `getCreditManagersList()` and selector `[102, 59, 143, 219]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCreditManagersList", abi = "getCreditManagersList()")]
    pub struct GetCreditManagersListCall;
    #[doc = "Container type for all input parameters for the `getPoolData` function with signature `getPoolData(address)` and selector `[19, 210, 28, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPoolData", abi = "getPoolData(address)")]
    pub struct GetPoolDataCall {
        pub pool: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getPoolsList` function with signature `getPoolsList()` and selector `[27, 205, 143, 192]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPoolsList", abi = "getPoolsList()")]
    pub struct GetPoolsListCall;
    #[doc = "Container type for all input parameters for the `hasOpenedCreditAccount` function with signature `hasOpenedCreditAccount(address,address)` and selector `[252, 153, 20, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "hasOpenedCreditAccount",
        abi = "hasOpenedCreditAccount(address,address)"
    )]
    pub struct HasOpenedCreditAccountCall {
        pub credit_manager: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `version` function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IDataCompressorCalls {
        GetAdapter(GetAdapterCall),
        GetCreditAccountData(GetCreditAccountDataCall),
        GetCreditAccountList(GetCreditAccountListCall),
        GetCreditManagerData(GetCreditManagerDataCall),
        GetCreditManagersList(GetCreditManagersListCall),
        GetPoolData(GetPoolDataCall),
        GetPoolsList(GetPoolsListCall),
        HasOpenedCreditAccount(HasOpenedCreditAccountCall),
        Version(VersionCall),
    }
    impl ethers::core::abi::AbiDecode for IDataCompressorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetAdapterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetAdapter(decoded));
            }
            if let Ok(decoded) =
                <GetCreditAccountDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetCreditAccountData(decoded));
            }
            if let Ok(decoded) =
                <GetCreditAccountListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetCreditAccountList(decoded));
            }
            if let Ok(decoded) =
                <GetCreditManagerDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetCreditManagerData(decoded));
            }
            if let Ok(decoded) =
                <GetCreditManagersListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetCreditManagersList(decoded));
            }
            if let Ok(decoded) =
                <GetPoolDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetPoolData(decoded));
            }
            if let Ok(decoded) =
                <GetPoolsListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::GetPoolsList(decoded));
            }
            if let Ok(decoded) =
                <HasOpenedCreditAccountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::HasOpenedCreditAccount(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IDataCompressorCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IDataCompressorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IDataCompressorCalls::GetAdapter(element) => element.encode(),
                IDataCompressorCalls::GetCreditAccountData(element) => element.encode(),
                IDataCompressorCalls::GetCreditAccountList(element) => element.encode(),
                IDataCompressorCalls::GetCreditManagerData(element) => element.encode(),
                IDataCompressorCalls::GetCreditManagersList(element) => element.encode(),
                IDataCompressorCalls::GetPoolData(element) => element.encode(),
                IDataCompressorCalls::GetPoolsList(element) => element.encode(),
                IDataCompressorCalls::HasOpenedCreditAccount(element) => element.encode(),
                IDataCompressorCalls::Version(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IDataCompressorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IDataCompressorCalls::GetAdapter(element) => element.fmt(f),
                IDataCompressorCalls::GetCreditAccountData(element) => element.fmt(f),
                IDataCompressorCalls::GetCreditAccountList(element) => element.fmt(f),
                IDataCompressorCalls::GetCreditManagerData(element) => element.fmt(f),
                IDataCompressorCalls::GetCreditManagersList(element) => element.fmt(f),
                IDataCompressorCalls::GetPoolData(element) => element.fmt(f),
                IDataCompressorCalls::GetPoolsList(element) => element.fmt(f),
                IDataCompressorCalls::HasOpenedCreditAccount(element) => element.fmt(f),
                IDataCompressorCalls::Version(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetAdapterCall> for IDataCompressorCalls {
        fn from(var: GetAdapterCall) -> Self {
            IDataCompressorCalls::GetAdapter(var)
        }
    }
    impl ::std::convert::From<GetCreditAccountDataCall> for IDataCompressorCalls {
        fn from(var: GetCreditAccountDataCall) -> Self {
            IDataCompressorCalls::GetCreditAccountData(var)
        }
    }
    impl ::std::convert::From<GetCreditAccountListCall> for IDataCompressorCalls {
        fn from(var: GetCreditAccountListCall) -> Self {
            IDataCompressorCalls::GetCreditAccountList(var)
        }
    }
    impl ::std::convert::From<GetCreditManagerDataCall> for IDataCompressorCalls {
        fn from(var: GetCreditManagerDataCall) -> Self {
            IDataCompressorCalls::GetCreditManagerData(var)
        }
    }
    impl ::std::convert::From<GetCreditManagersListCall> for IDataCompressorCalls {
        fn from(var: GetCreditManagersListCall) -> Self {
            IDataCompressorCalls::GetCreditManagersList(var)
        }
    }
    impl ::std::convert::From<GetPoolDataCall> for IDataCompressorCalls {
        fn from(var: GetPoolDataCall) -> Self {
            IDataCompressorCalls::GetPoolData(var)
        }
    }
    impl ::std::convert::From<GetPoolsListCall> for IDataCompressorCalls {
        fn from(var: GetPoolsListCall) -> Self {
            IDataCompressorCalls::GetPoolsList(var)
        }
    }
    impl ::std::convert::From<HasOpenedCreditAccountCall> for IDataCompressorCalls {
        fn from(var: HasOpenedCreditAccountCall) -> Self {
            IDataCompressorCalls::HasOpenedCreditAccount(var)
        }
    }
    impl ::std::convert::From<VersionCall> for IDataCompressorCalls {
        fn from(var: VersionCall) -> Self {
            IDataCompressorCalls::Version(var)
        }
    }
    #[doc = "Container type for all return fields from the `getAdapter` function with signature `getAdapter(address,address)` and selector `[76, 71, 47, 201]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetAdapterReturn {
        pub adapter: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `getCreditAccountData` function with signature `getCreditAccountData(address,address)` and selector `[13, 189, 97, 109]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCreditAccountDataReturn(pub CreditAccountData);
    #[doc = "Container type for all return fields from the `getCreditAccountList` function with signature `getCreditAccountList(address)` and selector `[168, 13, 237, 163]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCreditAccountListReturn(pub ::std::vec::Vec<CreditAccountData>);
    #[doc = "Container type for all return fields from the `getCreditManagerData` function with signature `getCreditManagerData(address)` and selector `[174, 9, 63, 63]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCreditManagerDataReturn(pub CreditManagerData);
    #[doc = "Container type for all return fields from the `getCreditManagersList` function with signature `getCreditManagersList()` and selector `[102, 59, 143, 219]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCreditManagersListReturn(pub ::std::vec::Vec<CreditManagerData>);
    #[doc = "Container type for all return fields from the `getPoolData` function with signature `getPoolData(address)` and selector `[19, 210, 28, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPoolDataReturn(pub PoolData);
    #[doc = "Container type for all return fields from the `getPoolsList` function with signature `getPoolsList()` and selector `[27, 205, 143, 192]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPoolsListReturn(pub ::std::vec::Vec<PoolData>);
    #[doc = "Container type for all return fields from the `hasOpenedCreditAccount` function with signature `hasOpenedCreditAccount(address,address)` and selector `[252, 153, 20, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HasOpenedCreditAccountReturn(pub bool);
    #[doc = "Container type for all return fields from the `version` function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VersionReturn(pub ethers::core::types::U256);
    #[doc = "`ContractAdapter(address,address)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ContractAdapter {
        pub allowed_contract: ethers::core::types::Address,
        pub adapter: ethers::core::types::Address,
    }
    #[doc = "`CreditAccountData(address,address,bool,address,address,uint256,uint256,uint256,uint256,uint256,(address,uint256,bool,bool)[],uint256,uint256,bool,uint256,uint256,uint256,uint8,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CreditAccountData {
        pub addr: ethers::core::types::Address,
        pub borrower: ethers::core::types::Address,
        pub in_use: bool,
        pub credit_manager: ethers::core::types::Address,
        pub underlying: ethers::core::types::Address,
        pub borrowed_amount_plus_interest: ethers::core::types::U256,
        pub borrowed_amount_plus_interest_and_fees: ethers::core::types::U256,
        pub total_value: ethers::core::types::U256,
        pub health_factor: ethers::core::types::U256,
        pub borrow_rate: ethers::core::types::U256,
        pub balances: ::std::vec::Vec<TokenBalance>,
        pub repay_amount: ethers::core::types::U256,
        pub liquidation_amount: ethers::core::types::U256,
        pub can_be_closed: bool,
        pub borrowed_amount: ethers::core::types::U256,
        pub cumulative_index_at_open: ethers::core::types::U256,
        pub since: ethers::core::types::U256,
        pub version: u8,
        pub enabled_token_mask: ethers::core::types::U256,
    }
    #[doc = "`CreditManagerData(address,address,address,bool,bool,uint256,uint256,uint256,uint256,uint256,address[],(address,address)[],uint256[],uint8,address,address,bool,address,bool,uint256,uint8,uint16,uint16,uint16,uint16,uint16)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CreditManagerData {
        pub addr: ethers::core::types::Address,
        pub underlying: ethers::core::types::Address,
        pub pool: ethers::core::types::Address,
        pub is_weth: bool,
        pub can_borrow: bool,
        pub borrow_rate: ethers::core::types::U256,
        pub min_amount: ethers::core::types::U256,
        pub max_amount: ethers::core::types::U256,
        pub max_leverage_factor: ethers::core::types::U256,
        pub available_liquidity: ethers::core::types::U256,
        pub collateral_tokens: Vec<ethers::core::types::Address>,
        pub adapters: ::std::vec::Vec<ContractAdapter>,
        pub liquidation_thresholds: Vec<ethers::core::types::U256>,
        pub version: u8,
        pub credit_facade: ethers::core::types::Address,
        pub credit_configurator: ethers::core::types::Address,
        pub is_degen_mode: bool,
        pub degen_nft: ethers::core::types::Address,
        pub is_increase_debt_forbidden: bool,
        pub forbidden_token_mask: ethers::core::types::U256,
        pub max_enabled_tokens_length: u8,
        pub fee_interest: u16,
        pub fee_liquidation: u16,
        pub liquidation_discount: u16,
        pub fee_liquidation_expired: u16,
        pub liquidation_discount_expired: u16,
    }
    #[doc = "`PoolData(address,bool,address,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint8)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct PoolData {
        pub addr: ethers::core::types::Address,
        pub is_weth: bool,
        pub underlying: ethers::core::types::Address,
        pub diesel_token: ethers::core::types::Address,
        pub linear_cumulative_index: ethers::core::types::U256,
        pub available_liquidity: ethers::core::types::U256,
        pub expected_liquidity: ethers::core::types::U256,
        pub expected_liquidity_limit: ethers::core::types::U256,
        pub total_borrowed: ethers::core::types::U256,
        pub deposit_apy_ray: ethers::core::types::U256,
        pub borrow_apy_ray: ethers::core::types::U256,
        pub diesel_rate_ray: ethers::core::types::U256,
        pub withdraw_fee: ethers::core::types::U256,
        pub cumulative_index_ray: ethers::core::types::U256,
        pub timestamp_lu: ethers::core::types::U256,
        pub version: u8,
    }
    #[doc = "`TokenBalance(address,uint256,bool,bool)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TokenBalance {
        pub token: ethers::core::types::Address,
        pub balance: ethers::core::types::U256,
        pub is_allowed: bool,
        pub is_enabled: bool,
    }
}

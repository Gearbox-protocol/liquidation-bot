pub use poolservice_mod::*;
#[allow(clippy::too_many_arguments)]
mod poolservice_mod {
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
    #[doc = "PoolService was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static POOLSERVICE_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_addressProvider\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_underlyingToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_dieselAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_interestRateModelAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_expectedLiquidityLimit\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"referralCode\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"AddLiquidity\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Borrow\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"creditManager\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"BorrowForbidden\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"creditManager\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"NewCreditManagerConnected\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"newLimit\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"NewExpectedLiquidityLimit\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newInterestRateModel\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"NewInterestRateModel\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"fee\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"NewWithdrawFee\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"Paused\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"RemoveLiquidity\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"borrowedAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"profit\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loss\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Repay\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"loss\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"UncoveredLoss\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"Unpaused\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"_cumulativeIndex_RAY\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"_expectedLiquidityLU\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"_timestampLU\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"referralCode\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"addLiquidity\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"addressProvider\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract AddressProvider\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"availableLiquidity\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"borrowAPY_RAY\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"calcLinearCumulative_RAY\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"cumulativeIndex_RAY\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"currentBorrowRate_RAY\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"timeDifference\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"calcLinearIndex_RAY\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"cumulativeIndexAtOpen\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"calcTimeDiscountedAmount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"connectCreditManager\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"creditManagers\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"creditManagersCanBorrow\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"creditManagersCanRepay\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"creditManagersCount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"dieselToken\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"expectedLiquidity\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"expectedLiquidityLimit\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"forbidCreditManagerToBorrow\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"fromDiesel\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getDieselRate_RAY\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"interestRateModel\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IInterestRateModel\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"borrowedAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"lendCreditAccount\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"pause\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"paused\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"removeLiquidity\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"borrowedAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"profit\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"loss\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"repayCreditAccount\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"newLimit\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setExpectedLiquidityLimit\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"fee\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setWithdrawFee\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"toDiesel\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"totalBorrowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"treasuryAddress\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"underlyingToken\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"unpause\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_interestRateModel\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"updateInterestRateModel\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"withdrawFee\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct PoolService<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for PoolService<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for PoolService<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PoolService))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> PoolService<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), POOLSERVICE_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `_cumulativeIndex_RAY` (0xdbcb313b) function"]
        pub fn cumulative_index_ray(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([219, 203, 49, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_expectedLiquidityLU` (0x030dbb04) function"]
        pub fn expected_liquidity_lu(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([3, 13, 187, 4], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_timestampLU` (0x609ae317) function"]
        pub fn timestamp_lu(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([96, 154, 227, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidity` (0x9aa5d462) function"]
        pub fn add_liquidity(
            &self,
            amount: ethers_core::types::U256,
            on_behalf_of: ethers_core::types::Address,
            referral_code: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 165, 212, 98], (amount, on_behalf_of, referral_code))
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
        #[doc = "Calls the contract's `availableLiquidity` (0x74375359) function"]
        pub fn available_liquidity(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([116, 55, 83, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowAPY_RAY` (0x45d31f9d) function"]
        pub fn borrow_apy_ray(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([69, 211, 31, 157], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcLinearCumulative_RAY` (0x0fce70fb) function"]
        pub fn calc_linear_cumulative_ray(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([15, 206, 112, 251], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcLinearIndex_RAY` (0x31d8bc27) function"]
        pub fn calc_linear_index_ray(
            &self,
            cumulative_index_ray: ethers_core::types::U256,
            current_borrow_rate_ray: ethers_core::types::U256,
            time_difference: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash(
                    [49, 216, 188, 39],
                    (
                        cumulative_index_ray,
                        current_borrow_rate_ray,
                        time_difference,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcTimeDiscountedAmount` (0x0edf6c4f) function"]
        pub fn calc_time_discounted_amount(
            &self,
            amount: ethers_core::types::U256,
            cumulative_index_at_open: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([14, 223, 108, 79], (amount, cumulative_index_at_open))
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
        #[doc = "Calls the contract's `creditManagers` (0x1e16e4fc) function"]
        pub fn credit_managers(
            &self,
            p0: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([30, 22, 228, 252], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditManagersCanBorrow` (0x2e97ca21) function"]
        pub fn credit_managers_can_borrow(
            &self,
            p0: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([46, 151, 202, 33], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditManagersCanRepay` (0x3e163df0) function"]
        pub fn credit_managers_can_repay(
            &self,
            p0: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([62, 22, 61, 240], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `creditManagersCount` (0xa4e8273e) function"]
        pub fn credit_managers_count(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([164, 232, 39, 62], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dieselToken` (0x36dda7d5) function"]
        pub fn diesel_token(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([54, 221, 167, 213], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expectedLiquidity` (0xfe14112d) function"]
        pub fn expected_liquidity(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([254, 20, 17, 45], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `expectedLiquidityLimit` (0xef8d9603) function"]
        pub fn expected_liquidity_limit(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([239, 141, 150, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forbidCreditManagerToBorrow` (0x078c4781) function"]
        pub fn forbid_credit_manager_to_borrow(
            &self,
            credit_manager: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 140, 71, 129], credit_manager)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fromDiesel` (0x5427c938) function"]
        pub fn from_diesel(
            &self,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([84, 39, 201, 56], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDieselRate_RAY` (0x788c6bfe) function"]
        pub fn get_diesel_rate_ray(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([120, 140, 107, 254], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `interestRateModel` (0xf3fdb15a) function"]
        pub fn interest_rate_model(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([243, 253, 177, 90], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lendCreditAccount` (0xbf28068b) function"]
        pub fn lend_credit_account(
            &self,
            borrowed_amount: ethers_core::types::U256,
            credit_account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 40, 6, 139], (borrowed_amount, credit_account))
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
        #[doc = "Calls the contract's `removeLiquidity` (0x05fe138b) function"]
        pub fn remove_liquidity(
            &self,
            amount: ethers_core::types::U256,
            to: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([5, 254, 19, 139], (amount, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayCreditAccount` (0xca9505e4) function"]
        pub fn repay_credit_account(
            &self,
            borrowed_amount: ethers_core::types::U256,
            profit: ethers_core::types::U256,
            loss: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 149, 5, 228], (borrowed_amount, profit, loss))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpectedLiquidityLimit` (0xbb04b193) function"]
        pub fn set_expected_liquidity_limit(
            &self,
            new_limit: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 4, 177, 147], new_limit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setWithdrawFee` (0xb6ac642a) function"]
        pub fn set_withdraw_fee(
            &self,
            fee: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 172, 100, 42], fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `toDiesel` (0x4d778ad1) function"]
        pub fn to_diesel(
            &self,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([77, 119, 138, 209], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalBorrowed` (0x4c19386c) function"]
        pub fn total_borrowed(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([76, 25, 56, 108], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `treasuryAddress` (0xc5f956af) function"]
        pub fn treasury_address(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([197, 249, 86, 175], ())
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
        #[doc = "Calls the contract's `updateInterestRateModel` (0x5664cacf) function"]
        pub fn update_interest_rate_model(
            &self,
            interest_rate_model: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 100, 202, 207], interest_rate_model)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFee` (0xe941fa78) function"]
        pub fn withdraw_fee(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([233, 65, 250, 120], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AddLiquidity` event"]
        pub fn add_liquidity_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, AddLiquidityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Borrow` event"]
        pub fn borrow_filter(&self) -> ethers_contract::builders::Event<M, BorrowFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `BorrowForbidden` event"]
        pub fn borrow_forbidden_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, BorrowForbiddenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewCreditManagerConnected` event"]
        pub fn new_credit_manager_connected_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, NewCreditManagerConnectedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewExpectedLiquidityLimit` event"]
        pub fn new_expected_liquidity_limit_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, NewExpectedLiquidityLimitFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewInterestRateModel` event"]
        pub fn new_interest_rate_model_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, NewInterestRateModelFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewWithdrawFee` event"]
        pub fn new_withdraw_fee_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, NewWithdrawFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(&self) -> ethers_contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RemoveLiquidity` event"]
        pub fn remove_liquidity_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, RemoveLiquidityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Repay` event"]
        pub fn repay_filter(&self) -> ethers_contract::builders::Event<M, RepayFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UncoveredLoss` event"]
        pub fn uncovered_loss_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, UncoveredLossFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(&self) -> ethers_contract::builders::Event<M, UnpausedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, PoolServiceEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "AddLiquidity",
        abi = "AddLiquidity(address,address,uint256,uint256)"
    )]
    pub struct AddLiquidityFilter {
        #[ethevent(indexed)]
        pub sender: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
        pub referral_code: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Borrow", abi = "Borrow(address,address,uint256)")]
    pub struct BorrowFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub credit_account: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "BorrowForbidden", abi = "BorrowForbidden(address)")]
    pub struct BorrowForbiddenFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "NewCreditManagerConnected",
        abi = "NewCreditManagerConnected(address)"
    )]
    pub struct NewCreditManagerConnectedFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "NewExpectedLiquidityLimit",
        abi = "NewExpectedLiquidityLimit(uint256)"
    )]
    pub struct NewExpectedLiquidityLimitFilter {
        pub new_limit: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "NewInterestRateModel", abi = "NewInterestRateModel(address)")]
    pub struct NewInterestRateModelFilter {
        #[ethevent(indexed)]
        pub new_interest_rate_model: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "NewWithdrawFee", abi = "NewWithdrawFee(uint256)")]
    pub struct NewWithdrawFeeFilter {
        pub fee: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "RemoveLiquidity",
        abi = "RemoveLiquidity(address,address,uint256)"
    )]
    pub struct RemoveLiquidityFilter {
        #[ethevent(indexed)]
        pub sender: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers_core::types::Address,
        pub amount: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Repay", abi = "Repay(address,uint256,uint256,uint256)")]
    pub struct RepayFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers_core::types::Address,
        pub borrowed_amount: ethers_core::types::U256,
        pub profit: ethers_core::types::U256,
        pub loss: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "UncoveredLoss", abi = "UncoveredLoss(address,uint256)")]
    pub struct UncoveredLossFilter {
        #[ethevent(indexed)]
        pub credit_manager: ethers_core::types::Address,
        pub loss: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ethers_core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum PoolServiceEvents {
        AddLiquidityFilter(AddLiquidityFilter),
        BorrowFilter(BorrowFilter),
        BorrowForbiddenFilter(BorrowForbiddenFilter),
        NewCreditManagerConnectedFilter(NewCreditManagerConnectedFilter),
        NewExpectedLiquidityLimitFilter(NewExpectedLiquidityLimitFilter),
        NewInterestRateModelFilter(NewInterestRateModelFilter),
        NewWithdrawFeeFilter(NewWithdrawFeeFilter),
        PausedFilter(PausedFilter),
        RemoveLiquidityFilter(RemoveLiquidityFilter),
        RepayFilter(RepayFilter),
        UncoveredLossFilter(UncoveredLossFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers_core::abi::Tokenizable for PoolServiceEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddLiquidityFilter::from_token(token.clone()) {
                return Ok(PoolServiceEvents::AddLiquidityFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::from_token(token.clone()) {
                return Ok(PoolServiceEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = BorrowForbiddenFilter::from_token(token.clone()) {
                return Ok(PoolServiceEvents::BorrowForbiddenFilter(decoded));
            }
            if let Ok(decoded) = NewCreditManagerConnectedFilter::from_token(token.clone()) {
                return Ok(PoolServiceEvents::NewCreditManagerConnectedFilter(decoded));
            }
            if let Ok(decoded) = NewExpectedLiquidityLimitFilter::from_token(token.clone()) {
                return Ok(PoolServiceEvents::NewExpectedLiquidityLimitFilter(decoded));
            }
            if let Ok(decoded) = NewInterestRateModelFilter::from_token(token.clone()) {
                return Ok(PoolServiceEvents::NewInterestRateModelFilter(decoded));
            }
            if let Ok(decoded) = NewWithdrawFeeFilter::from_token(token.clone()) {
                return Ok(PoolServiceEvents::NewWithdrawFeeFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::from_token(token.clone()) {
                return Ok(PoolServiceEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = RemoveLiquidityFilter::from_token(token.clone()) {
                return Ok(PoolServiceEvents::RemoveLiquidityFilter(decoded));
            }
            if let Ok(decoded) = RepayFilter::from_token(token.clone()) {
                return Ok(PoolServiceEvents::RepayFilter(decoded));
            }
            if let Ok(decoded) = UncoveredLossFilter::from_token(token.clone()) {
                return Ok(PoolServiceEvents::UncoveredLossFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::from_token(token.clone()) {
                return Ok(PoolServiceEvents::UnpausedFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                PoolServiceEvents::AddLiquidityFilter(element) => element.into_token(),
                PoolServiceEvents::BorrowFilter(element) => element.into_token(),
                PoolServiceEvents::BorrowForbiddenFilter(element) => element.into_token(),
                PoolServiceEvents::NewCreditManagerConnectedFilter(element) => element.into_token(),
                PoolServiceEvents::NewExpectedLiquidityLimitFilter(element) => element.into_token(),
                PoolServiceEvents::NewInterestRateModelFilter(element) => element.into_token(),
                PoolServiceEvents::NewWithdrawFeeFilter(element) => element.into_token(),
                PoolServiceEvents::PausedFilter(element) => element.into_token(),
                PoolServiceEvents::RemoveLiquidityFilter(element) => element.into_token(),
                PoolServiceEvents::RepayFilter(element) => element.into_token(),
                PoolServiceEvents::UncoveredLossFilter(element) => element.into_token(),
                PoolServiceEvents::UnpausedFilter(element) => element.into_token(),
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for PoolServiceEvents {}
    impl ethers_contract::EthLogDecode for PoolServiceEvents {
        fn decode_log(log: &ethers_core::abi::RawLog) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddLiquidityFilter::decode_log(log) {
                return Ok(PoolServiceEvents::AddLiquidityFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(PoolServiceEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = BorrowForbiddenFilter::decode_log(log) {
                return Ok(PoolServiceEvents::BorrowForbiddenFilter(decoded));
            }
            if let Ok(decoded) = NewCreditManagerConnectedFilter::decode_log(log) {
                return Ok(PoolServiceEvents::NewCreditManagerConnectedFilter(decoded));
            }
            if let Ok(decoded) = NewExpectedLiquidityLimitFilter::decode_log(log) {
                return Ok(PoolServiceEvents::NewExpectedLiquidityLimitFilter(decoded));
            }
            if let Ok(decoded) = NewInterestRateModelFilter::decode_log(log) {
                return Ok(PoolServiceEvents::NewInterestRateModelFilter(decoded));
            }
            if let Ok(decoded) = NewWithdrawFeeFilter::decode_log(log) {
                return Ok(PoolServiceEvents::NewWithdrawFeeFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(PoolServiceEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = RemoveLiquidityFilter::decode_log(log) {
                return Ok(PoolServiceEvents::RemoveLiquidityFilter(decoded));
            }
            if let Ok(decoded) = RepayFilter::decode_log(log) {
                return Ok(PoolServiceEvents::RepayFilter(decoded));
            }
            if let Ok(decoded) = UncoveredLossFilter::decode_log(log) {
                return Ok(PoolServiceEvents::UncoveredLossFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(PoolServiceEvents::UnpausedFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}

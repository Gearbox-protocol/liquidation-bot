pub use aggregatorv3interface_mod::*;
#[allow(clippy::too_many_arguments)]
mod aggregatorv3interface_mod {
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
    #[doc = "AggregatorV3Interface was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static AGGREGATORV3INTERFACE_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [],\n    \"name\": \"decimals\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"description\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"_roundId\",\n        \"type\": \"uint80\"\n      }\n    ],\n    \"name\": \"getRoundData\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"roundId\",\n        \"type\": \"uint80\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"answer\",\n        \"type\": \"int256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startedAt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"updatedAt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"answeredInRound\",\n        \"type\": \"uint80\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"latestRoundData\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"roundId\",\n        \"type\": \"uint80\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"answer\",\n        \"type\": \"int256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startedAt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"updatedAt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"answeredInRound\",\n        \"type\": \"uint80\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"version\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct AggregatorV3Interface<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for AggregatorV3Interface<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for AggregatorV3Interface<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AggregatorV3Interface))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> AggregatorV3Interface<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                AGGREGATORV3INTERFACE_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `description` (0x7284e416) function"]
        pub fn description(&self) -> ethers_contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([114, 132, 228, 22], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoundData` (0x9a6fc8f5) function"]
        pub fn get_round_data(
            &self,
            round_id: u128,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([154, 111, 200, 245], round_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `latestRoundData` (0xfeaf968c) function"]
        pub fn latest_round_data(
            &self,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                u128,
                I256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([254, 175, 150, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
    }
}

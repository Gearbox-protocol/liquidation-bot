pub use iterminator_mod::*;
#[allow(clippy::too_many_arguments)]
mod iterminator_mod {
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
    #[doc = "ITerminator was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ITERMINATOR_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_executor\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowExecutor\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"executors\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_executor\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"forbidExecutor\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amountIn\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"path\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amountOutMin\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct ITerminator.UniV2Params[]\",\n        \"name\": \"_routes\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"_yearnTokens\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"liquidate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"creditManagers\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"tokens\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"provideAllowance\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct ITerminator<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for ITerminator<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for ITerminator<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ITerminator))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> ITerminator<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), ITERMINATOR_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `allowExecutor` (0xb1b05f2a) function"]
        pub fn allow_executor(
            &self,
            executor: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([177, 176, 95, 42], executor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executors` (0x9ac2a011) function"]
        pub fn executors(
            &self,
            p0: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([154, 194, 160, 17], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forbidExecutor` (0x6e9d5987) function"]
        pub fn forbid_executor(
            &self,
            executor: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 157, 89, 135], executor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidate` (0x35eb19ad) function"]
        pub fn liquidate(
            &self,
            credit_manager: ethers_core::types::Address,
            borrower: ethers_core::types::Address,
            routes: Vec<(
                ethers_core::types::U256,
                Vec<ethers_core::types::Address>,
                ethers_core::types::U256,
            )>,
            yearn_tokens: Vec<ethers_core::types::Address>,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [53, 235, 25, 173],
                    (credit_manager, borrower, routes, yearn_tokens),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `provideAllowance` (0x35f4c03a) function"]
        pub fn provide_allowance(
            &self,
            credit_managers: Vec<ethers_core::types::Address>,
            tokens: Vec<ethers_core::types::Address>,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 244, 192, 58], (credit_managers, tokens))
                .expect("method not found (this should never happen)")
        }
    }
}

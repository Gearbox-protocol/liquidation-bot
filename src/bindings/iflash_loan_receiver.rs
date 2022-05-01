pub use iflashloanreceiver_mod::*;
#[allow(clippy::too_many_arguments)]
mod iflashloanreceiver_mod {
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
    #[doc = "IFlashLoanReceiver was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IFLASHLOANRECEIVER_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [],\n    \"name\": \"ADDRESSES_PROVIDER\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ILendingPoolAddressesProvider\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"LENDING_POOL\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ILendingPool\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"assets\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amounts\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"premiums\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"initiator\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"params\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"executeOperation\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct IFlashLoanReceiver<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for IFlashLoanReceiver<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for IFlashLoanReceiver<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IFlashLoanReceiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> IFlashLoanReceiver<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                IFLASHLOANRECEIVER_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `ADDRESSES_PROVIDER` (0x0542975c) function"]
        pub fn addresses_provider(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([5, 66, 151, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `LENDING_POOL` (0xb4dcfc77) function"]
        pub fn lending_pool(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([180, 220, 252, 119], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeOperation` (0x920f5c84) function"]
        pub fn execute_operation(
            &self,
            assets: Vec<ethers_core::types::Address>,
            amounts: Vec<ethers_core::types::U256>,
            premiums: Vec<ethers_core::types::U256>,
            initiator: ethers_core::types::Address,
            params: Vec<u8>,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [146, 15, 92, 132],
                    (assets, amounts, premiums, initiator, params),
                )
                .expect("method not found (this should never happen)")
        }
    }
}

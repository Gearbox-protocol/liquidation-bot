pub use pathfinder_mod::*;
#[allow(clippy::too_many_arguments)]
mod pathfinder_mod {
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
    #[doc = "PathFinder was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PATHFINDER_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_addressProvider\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"addressProvider\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract AddressProvider\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"swapInterface\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"router\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"swapType\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"tokens\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"bestUniPath\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"path\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"rate\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"expectedAmount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct PathFinder.TradePath\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"contractsRegister\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract ContractsRegister\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"path\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"swapType\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"convertPathToPathV3\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"result\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"router\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"connectorTokens\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"getClosurePaths\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"path\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"rate\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"expectedAmount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct PathFinder.TradePath[]\",\n        \"name\": \"result\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct PathFinder<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for PathFinder<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for PathFinder<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PathFinder))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> PathFinder<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), PATHFINDER_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `addressProvider` (0x2954018c) function"]
        pub fn address_provider(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([41, 84, 1, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bestUniPath` (0x457b2030) function"]
        pub fn best_uni_path(
            &self,
            swap_interface: ethers_core::types::U256,
            router: ethers_core::types::Address,
            swap_type: ethers_core::types::U256,
            from: ethers_core::types::Address,
            to: ethers_core::types::Address,
            amount: ethers_core::types::U256,
            tokens: Vec<ethers_core::types::Address>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                Vec<ethers_core::types::Address>,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [69, 123, 32, 48],
                    (swap_interface, router, swap_type, from, to, amount, tokens),
                )
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
        #[doc = "Calls the contract's `convertPathToPathV3` (0xe2430f93) function"]
        pub fn convert_path_to_path_v3(
            &self,
            path: Vec<ethers_core::types::Address>,
            swap_type: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, Vec<u8>> {
            self.0
                .method_hash([226, 67, 15, 147], (path, swap_type))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getClosurePaths` (0x8b7ce872) function"]
        pub fn get_closure_paths(
            &self,
            router: ethers_core::types::Address,
            credit_manager: ethers_core::types::Address,
            borrower: ethers_core::types::Address,
            connector_tokens: Vec<ethers_core::types::Address>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            Vec<(
                Vec<ethers_core::types::Address>,
                ethers_core::types::U256,
                ethers_core::types::U256,
            )>,
        > {
            self.0
                .method_hash(
                    [139, 124, 232, 114],
                    (router, credit_manager, borrower, connector_tokens),
                )
                .expect("method not found (this should never happen)")
        }
    }
}

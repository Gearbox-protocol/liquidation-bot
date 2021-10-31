pub use icreditaccount_mod::*;
#[allow(clippy::too_many_arguments)]
mod icreditaccount_mod {
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
    #[doc = "ICreditAccount was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICREDITACCOUNT_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"swapContract\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"approveToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"borrowedAmount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"targetContract\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"cancelAllowance\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_creditManager\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"connectTo\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"creditManager\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"cumulativeIndexAtOpen\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"destination\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"execute\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"factory\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"initialize\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"safeTransfer\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_borrowedAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_cumulativeIndexAtOpen\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"setGenericParameters\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"since\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_borrowedAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"updateBorrowedAmount\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct ICreditAccount<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for ICreditAccount<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for ICreditAccount<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICreditAccount))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> ICreditAccount<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), ICREDITACCOUNT_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `approveToken` (0x03105b04) function"]
        pub fn approve_token(
            &self,
            token: ethers_core::types::Address,
            swap_contract: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 16, 91, 4], (token, swap_contract))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowedAmount` (0x1afbb7a4) function"]
        pub fn borrowed_amount(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([26, 251, 183, 164], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelAllowance` (0x19a16039) function"]
        pub fn cancel_allowance(
            &self,
            token: ethers_core::types::Address,
            target_contract: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 161, 96, 57], (token, target_contract))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `connectTo` (0x4725ac3f) function"]
        pub fn connect_to(
            &self,
            credit_manager: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 37, 172, 63], credit_manager)
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
        #[doc = "Calls the contract's `cumulativeIndexAtOpen` (0x17d11a15) function"]
        pub fn cumulative_index_at_open(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([23, 209, 26, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `execute` (0x1cff79cd) function"]
        pub fn execute(
            &self,
            destination: ethers_core::types::Address,
            data: Vec<u8>,
        ) -> ethers_contract::builders::ContractCall<M, Vec<u8>> {
            self.0
                .method_hash([28, 255, 121, 205], (destination, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x8129fc1c) function"]
        pub fn initialize(&self) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 41, 252, 28], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransfer` (0xd1660f99) function"]
        pub fn safe_transfer(
            &self,
            token: ethers_core::types::Address,
            to: ethers_core::types::Address,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 102, 15, 153], (token, to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGenericParameters` (0xabaf48f5) function"]
        pub fn set_generic_parameters(
            &self,
            borrowed_amount: ethers_core::types::U256,
            cumulative_index_at_open: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [171, 175, 72, 245],
                    (borrowed_amount, cumulative_index_at_open),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `since` (0x3dc54b40) function"]
        pub fn since(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([61, 197, 75, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateBorrowedAmount` (0x77904813) function"]
        pub fn update_borrowed_amount(
            &self,
            borrowed_amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([119, 144, 72, 19], borrowed_amount)
                .expect("method not found (this should never happen)")
        }
    }
}

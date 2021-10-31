pub use iwethgateway_mod::*;
#[allow(clippy::too_many_arguments)]
mod iwethgateway_mod {
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
    #[doc = "IWETHGateway was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IWETHGATEWAY_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"addCollateralETH\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pool\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"referralCode\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"addLiquidityETH\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"leverageFactor\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"referralCode\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"openCreditAccountETH\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pool\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"removeLiquidityETH\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditManager\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"repayCreditAccountETH\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"unwrapWETH\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct IWETHGateway<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for IWETHGateway<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for IWETHGateway<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IWETHGateway))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> IWETHGateway<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), IWETHGATEWAY_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `addCollateralETH` (0x420ac3b2) function"]
        pub fn add_collateral_eth(
            &self,
            credit_manager: ethers_core::types::Address,
            on_behalf_of: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 10, 195, 178], (credit_manager, on_behalf_of))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidityETH` (0xdeecfbc9) function"]
        pub fn add_liquidity_eth(
            &self,
            pool: ethers_core::types::Address,
            on_behalf_of: ethers_core::types::Address,
            referral_code: u16,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 236, 251, 201], (pool, on_behalf_of, referral_code))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `openCreditAccountETH` (0xd8c99bc3) function"]
        pub fn open_credit_account_eth(
            &self,
            credit_manager: ethers_core::types::Address,
            on_behalf_of: ethers_core::types::Address,
            leverage_factor: ethers_core::types::U256,
            referral_code: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [216, 201, 155, 195],
                    (credit_manager, on_behalf_of, leverage_factor, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETH` (0xe79a4089) function"]
        pub fn remove_liquidity_eth(
            &self,
            pool: ethers_core::types::Address,
            amount: ethers_core::types::U256,
            to: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 154, 64, 137], (pool, amount, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayCreditAccountETH` (0xa6eab5c2) function"]
        pub fn repay_credit_account_eth(
            &self,
            credit_manager: ethers_core::types::Address,
            to: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 234, 181, 194], (credit_manager, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapWETH` (0x5869dba8) function"]
        pub fn unwrap_weth(
            &self,
            to: ethers_core::types::Address,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 105, 219, 168], (to, amount))
                .expect("method not found (this should never happen)")
        }
    }
}

pub use ipriceoracle_mod::*;
#[allow(clippy::too_many_arguments)]
mod ipriceoracle_mod {
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
    #[doc = "IPriceOracle was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IPRICEORACLE_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"priceFeed\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"NewPriceFeed\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"priceFeedToken\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"addPriceFeed\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenFrom\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenTo\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"convert\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenFrom\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenTo\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getLastPrice\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct IPriceOracle<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for IPriceOracle<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for IPriceOracle<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IPriceOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> IPriceOracle<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), IPRICEORACLE_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `addPriceFeed` (0xe8a97a3e) function"]
        pub fn add_price_feed(
            &self,
            token: ethers_core::types::Address,
            price_feed_token: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 169, 122, 62], (token, price_feed_token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `convert` (0xb66102df) function"]
        pub fn convert(
            &self,
            amount: ethers_core::types::U256,
            token_from: ethers_core::types::Address,
            token_to: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([182, 97, 2, 223], (amount, token_from, token_to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastPrice` (0x743b9086) function"]
        pub fn get_last_price(
            &self,
            token_from: ethers_core::types::Address,
            token_to: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([116, 59, 144, 134], (token_from, token_to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewPriceFeed` event"]
        pub fn new_price_feed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, NewPriceFeedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, NewPriceFeedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "NewPriceFeed", abi = "NewPriceFeed(address,address)")]
    pub struct NewPriceFeedFilter {
        #[ethevent(indexed)]
        pub token: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub price_feed: ethers_core::types::Address,
    }
}

pub use ilendingpooladdressesprovider_mod::*;
#[allow(clippy::too_many_arguments)]
mod ilendingpooladdressesprovider_mod {
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
    #[doc = "ILendingPoolAddressesProvider was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ILENDINGPOOLADDRESSESPROVIDER_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"hasProxy\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"AddressSet\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ConfigurationAdminUpdated\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"EmergencyAdminUpdated\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"LendingPoolCollateralManagerUpdated\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"LendingPoolConfiguratorUpdated\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"LendingPoolUpdated\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"LendingRateOracleUpdated\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"string\",\n        \"name\": \"newMarketId\",\n        \"type\": \"string\"\n      }\n    ],\n    \"name\": \"MarketIdSet\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"PriceOracleUpdated\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ProxyCreated\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"getAddress\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getEmergencyAdmin\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getLendingPool\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getLendingPoolCollateralManager\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getLendingPoolConfigurator\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getLendingRateOracle\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getMarketId\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getPoolAdmin\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getPriceOracle\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setAddress\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"impl\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setAddressAsProxy\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setEmergencyAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"manager\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setLendingPoolCollateralManager\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"configurator\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setLendingPoolConfiguratorImpl\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pool\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setLendingPoolImpl\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"lendingRateOracle\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setLendingRateOracle\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"marketId\",\n        \"type\": \"string\"\n      }\n    ],\n    \"name\": \"setMarketId\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"admin\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setPoolAdmin\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"priceOracle\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setPriceOracle\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct ILendingPoolAddressesProvider<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for ILendingPoolAddressesProvider<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for ILendingPoolAddressesProvider<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ILendingPoolAddressesProvider))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> ILendingPoolAddressesProvider<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                ILENDINGPOOLADDRESSESPROVIDER_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `getAddress` (0x21f8a721) function"]
        pub fn get_address(
            &self,
            id: [u8; 32],
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([33, 248, 167, 33], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getEmergencyAdmin` (0xddcaa9ea) function"]
        pub fn get_emergency_admin(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([221, 202, 169, 234], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLendingPool` (0x0261bf8b) function"]
        pub fn get_lending_pool(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([2, 97, 191, 139], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLendingPoolCollateralManager` (0x712d9171) function"]
        pub fn get_lending_pool_collateral_manager(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([113, 45, 145, 113], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLendingPoolConfigurator` (0x85c858b1) function"]
        pub fn get_lending_pool_configurator(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([133, 200, 88, 177], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLendingRateOracle` (0x3618abba) function"]
        pub fn get_lending_rate_oracle(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([54, 24, 171, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMarketId` (0x568ef470) function"]
        pub fn get_market_id(&self) -> ethers_contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([86, 142, 244, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPoolAdmin` (0xaecda378) function"]
        pub fn get_pool_admin(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([174, 205, 163, 120], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPriceOracle` (0xfca513a8) function"]
        pub fn get_price_oracle(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([252, 165, 19, 168], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAddress` (0xca446dd9) function"]
        pub fn set_address(
            &self,
            id: [u8; 32],
            new_address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 68, 109, 217], (id, new_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAddressAsProxy` (0x5dcc528c) function"]
        pub fn set_address_as_proxy(
            &self,
            id: [u8; 32],
            impl_: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 204, 82, 140], (id, impl_))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setEmergencyAdmin` (0x35da3394) function"]
        pub fn set_emergency_admin(
            &self,
            admin: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 218, 51, 148], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLendingPoolCollateralManager` (0x398e5553) function"]
        pub fn set_lending_pool_collateral_manager(
            &self,
            manager: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 142, 85, 83], manager)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLendingPoolConfiguratorImpl` (0xc12542df) function"]
        pub fn set_lending_pool_configurator_impl(
            &self,
            configurator: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([193, 37, 66, 223], configurator)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLendingPoolImpl` (0x5aef021f) function"]
        pub fn set_lending_pool_impl(
            &self,
            pool: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 239, 2, 31], pool)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLendingRateOracle` (0x820d1274) function"]
        pub fn set_lending_rate_oracle(
            &self,
            lending_rate_oracle: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 13, 18, 116], lending_rate_oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMarketId` (0xf67b1847) function"]
        pub fn set_market_id(
            &self,
            market_id: String,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 123, 24, 71], market_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPoolAdmin` (0x283d62ad) function"]
        pub fn set_pool_admin(
            &self,
            admin: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 61, 98, 173], admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPriceOracle` (0x530e784f) function"]
        pub fn set_price_oracle(
            &self,
            price_oracle: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 14, 120, 79], price_oracle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AddressSet` event"]
        pub fn address_set_filter(&self) -> ethers_contract::builders::Event<M, AddressSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ConfigurationAdminUpdated` event"]
        pub fn configuration_admin_updated_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ConfigurationAdminUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EmergencyAdminUpdated` event"]
        pub fn emergency_admin_updated_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, EmergencyAdminUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LendingPoolCollateralManagerUpdated` event"]
        pub fn lending_pool_collateral_manager_updated_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, LendingPoolCollateralManagerUpdatedFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `LendingPoolConfiguratorUpdated` event"]
        pub fn lending_pool_configurator_updated_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, LendingPoolConfiguratorUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LendingPoolUpdated` event"]
        pub fn lending_pool_updated_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, LendingPoolUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LendingRateOracleUpdated` event"]
        pub fn lending_rate_oracle_updated_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, LendingRateOracleUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MarketIdSet` event"]
        pub fn market_id_set_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, MarketIdSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PriceOracleUpdated` event"]
        pub fn price_oracle_updated_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, PriceOracleUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProxyCreated` event"]
        pub fn proxy_created_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ProxyCreatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(
            &self,
        ) -> ethers_contract::builders::Event<M, ILendingPoolAddressesProviderEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "AddressSet", abi = "AddressSet(bytes32,address,bool)")]
    pub struct AddressSetFilter {
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub new_address: ethers_core::types::Address,
        pub has_proxy: bool,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "ConfigurationAdminUpdated",
        abi = "ConfigurationAdminUpdated(address)"
    )]
    pub struct ConfigurationAdminUpdatedFilter {
        #[ethevent(indexed)]
        pub new_address: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "EmergencyAdminUpdated", abi = "EmergencyAdminUpdated(address)")]
    pub struct EmergencyAdminUpdatedFilter {
        #[ethevent(indexed)]
        pub new_address: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "LendingPoolCollateralManagerUpdated",
        abi = "LendingPoolCollateralManagerUpdated(address)"
    )]
    pub struct LendingPoolCollateralManagerUpdatedFilter {
        #[ethevent(indexed)]
        pub new_address: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "LendingPoolConfiguratorUpdated",
        abi = "LendingPoolConfiguratorUpdated(address)"
    )]
    pub struct LendingPoolConfiguratorUpdatedFilter {
        #[ethevent(indexed)]
        pub new_address: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "LendingPoolUpdated", abi = "LendingPoolUpdated(address)")]
    pub struct LendingPoolUpdatedFilter {
        #[ethevent(indexed)]
        pub new_address: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "LendingRateOracleUpdated",
        abi = "LendingRateOracleUpdated(address)"
    )]
    pub struct LendingRateOracleUpdatedFilter {
        #[ethevent(indexed)]
        pub new_address: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "MarketIdSet", abi = "MarketIdSet(string)")]
    pub struct MarketIdSetFilter {
        pub new_market_id: String,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "PriceOracleUpdated", abi = "PriceOracleUpdated(address)")]
    pub struct PriceOracleUpdatedFilter {
        #[ethevent(indexed)]
        pub new_address: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "ProxyCreated", abi = "ProxyCreated(bytes32,address)")]
    pub struct ProxyCreatedFilter {
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub new_address: ethers_core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ILendingPoolAddressesProviderEvents {
        AddressSetFilter(AddressSetFilter),
        ConfigurationAdminUpdatedFilter(ConfigurationAdminUpdatedFilter),
        EmergencyAdminUpdatedFilter(EmergencyAdminUpdatedFilter),
        LendingPoolCollateralManagerUpdatedFilter(LendingPoolCollateralManagerUpdatedFilter),
        LendingPoolConfiguratorUpdatedFilter(LendingPoolConfiguratorUpdatedFilter),
        LendingPoolUpdatedFilter(LendingPoolUpdatedFilter),
        LendingRateOracleUpdatedFilter(LendingRateOracleUpdatedFilter),
        MarketIdSetFilter(MarketIdSetFilter),
        PriceOracleUpdatedFilter(PriceOracleUpdatedFilter),
        ProxyCreatedFilter(ProxyCreatedFilter),
    }
    impl ethers_core::abi::Tokenizable for ILendingPoolAddressesProviderEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddressSetFilter::from_token(token.clone()) {
                return Ok(ILendingPoolAddressesProviderEvents::AddressSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ConfigurationAdminUpdatedFilter::from_token(token.clone()) {
                return Ok(
                    ILendingPoolAddressesProviderEvents::ConfigurationAdminUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = EmergencyAdminUpdatedFilter::from_token(token.clone()) {
                return Ok(
                    ILendingPoolAddressesProviderEvents::EmergencyAdminUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) =
                LendingPoolCollateralManagerUpdatedFilter::from_token(token.clone())
            {
                return Ok(
                    ILendingPoolAddressesProviderEvents::LendingPoolCollateralManagerUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = LendingPoolConfiguratorUpdatedFilter::from_token(token.clone()) {
                return Ok(
                    ILendingPoolAddressesProviderEvents::LendingPoolConfiguratorUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = LendingPoolUpdatedFilter::from_token(token.clone()) {
                return Ok(ILendingPoolAddressesProviderEvents::LendingPoolUpdatedFilter(decoded));
            }
            if let Ok(decoded) = LendingRateOracleUpdatedFilter::from_token(token.clone()) {
                return Ok(
                    ILendingPoolAddressesProviderEvents::LendingRateOracleUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MarketIdSetFilter::from_token(token.clone()) {
                return Ok(ILendingPoolAddressesProviderEvents::MarketIdSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PriceOracleUpdatedFilter::from_token(token.clone()) {
                return Ok(ILendingPoolAddressesProviderEvents::PriceOracleUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ProxyCreatedFilter::from_token(token.clone()) {
                return Ok(ILendingPoolAddressesProviderEvents::ProxyCreatedFilter(
                    decoded,
                ));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                ILendingPoolAddressesProviderEvents::AddressSetFilter(element) => {
                    element.into_token()
                }
                ILendingPoolAddressesProviderEvents::ConfigurationAdminUpdatedFilter(element) => {
                    element.into_token()
                }
                ILendingPoolAddressesProviderEvents::EmergencyAdminUpdatedFilter(element) => {
                    element.into_token()
                }
                ILendingPoolAddressesProviderEvents::LendingPoolCollateralManagerUpdatedFilter(
                    element,
                ) => element.into_token(),
                ILendingPoolAddressesProviderEvents::LendingPoolConfiguratorUpdatedFilter(
                    element,
                ) => element.into_token(),
                ILendingPoolAddressesProviderEvents::LendingPoolUpdatedFilter(element) => {
                    element.into_token()
                }
                ILendingPoolAddressesProviderEvents::LendingRateOracleUpdatedFilter(element) => {
                    element.into_token()
                }
                ILendingPoolAddressesProviderEvents::MarketIdSetFilter(element) => {
                    element.into_token()
                }
                ILendingPoolAddressesProviderEvents::PriceOracleUpdatedFilter(element) => {
                    element.into_token()
                }
                ILendingPoolAddressesProviderEvents::ProxyCreatedFilter(element) => {
                    element.into_token()
                }
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for ILendingPoolAddressesProviderEvents {}
    impl ethers_contract::EthLogDecode for ILendingPoolAddressesProviderEvents {
        fn decode_log(log: &ethers_core::abi::RawLog) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddressSetFilter::decode_log(log) {
                return Ok(ILendingPoolAddressesProviderEvents::AddressSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ConfigurationAdminUpdatedFilter::decode_log(log) {
                return Ok(
                    ILendingPoolAddressesProviderEvents::ConfigurationAdminUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = EmergencyAdminUpdatedFilter::decode_log(log) {
                return Ok(
                    ILendingPoolAddressesProviderEvents::EmergencyAdminUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = LendingPoolCollateralManagerUpdatedFilter::decode_log(log) {
                return Ok(
                    ILendingPoolAddressesProviderEvents::LendingPoolCollateralManagerUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = LendingPoolConfiguratorUpdatedFilter::decode_log(log) {
                return Ok(
                    ILendingPoolAddressesProviderEvents::LendingPoolConfiguratorUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = LendingPoolUpdatedFilter::decode_log(log) {
                return Ok(ILendingPoolAddressesProviderEvents::LendingPoolUpdatedFilter(decoded));
            }
            if let Ok(decoded) = LendingRateOracleUpdatedFilter::decode_log(log) {
                return Ok(
                    ILendingPoolAddressesProviderEvents::LendingRateOracleUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = MarketIdSetFilter::decode_log(log) {
                return Ok(ILendingPoolAddressesProviderEvents::MarketIdSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PriceOracleUpdatedFilter::decode_log(log) {
                return Ok(ILendingPoolAddressesProviderEvents::PriceOracleUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ProxyCreatedFilter::decode_log(log) {
                return Ok(ILendingPoolAddressesProviderEvents::ProxyCreatedFilter(
                    decoded,
                ));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}

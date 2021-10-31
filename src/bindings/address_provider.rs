pub use addressprovider_mod::*;
#[allow(clippy::too_many_arguments)]
mod addressprovider_mod {
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
    #[doc = "AddressProvider was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ADDRESSPROVIDER_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"service\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newAddress\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"AddressSet\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"previousOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ACCOUNT_FACTORY\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ACCOUNT_MINER\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ACL\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"CONTRACTS_REGISTER\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"DATA_COMPRESSOR\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"GEAR_TOKEN\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"PRICE_ORACLE\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"TREASURY_CONTRACT\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"WETH_GATEWAY\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"WETH_TOKEN\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"addresses\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getACL\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getAccountFactory\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getAccountMiner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getContractsRegister\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getDataCompressor\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getGearToken\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getPriceOracle\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getTreasuryContract\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getWETHGateway\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getWethToken\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"renounceOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_address\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setACL\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_address\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setAccountFactory\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_address\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setAccountMiner\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_address\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setContractsRegister\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_address\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setDataCompressor\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_address\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setGearToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_address\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setPriceOracle\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_address\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setTreasuryContract\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_address\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setWETHGateway\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_address\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setWethToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct AddressProvider<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for AddressProvider<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for AddressProvider<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AddressProvider))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> AddressProvider<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), ADDRESSPROVIDER_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `ACCOUNT_FACTORY` (0x05197d10) function"]
        pub fn account_factory(&self) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([5, 25, 125, 16], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ACCOUNT_MINER` (0xc0e53d6e) function"]
        pub fn account_miner(&self) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([192, 229, 61, 110], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ACL` (0x7af53532) function"]
        pub fn acl(&self) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([122, 245, 53, 50], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CONTRACTS_REGISTER` (0xf9366f47) function"]
        pub fn contracts_register(&self) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([249, 54, 111, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DATA_COMPRESSOR` (0x72788be7) function"]
        pub fn data_compressor(&self) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([114, 120, 139, 231], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `GEAR_TOKEN` (0x124a6462) function"]
        pub fn gear_token(&self) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([18, 74, 100, 98], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PRICE_ORACLE` (0x0a19399a) function"]
        pub fn price_oracle(&self) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([10, 25, 57, 154], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `TREASURY_CONTRACT` (0x9e9df2b9) function"]
        pub fn treasury_contract(&self) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([158, 157, 242, 185], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WETH_GATEWAY` (0xae5a98ba) function"]
        pub fn weth_gateway(&self) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([174, 90, 152, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WETH_TOKEN` (0x37d277d4) function"]
        pub fn weth_token(&self) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([55, 210, 119, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addresses` (0x699f200f) function"]
        pub fn addresses(
            &self,
            p0: [u8; 32],
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([105, 159, 32, 15], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getACL` (0x08737695) function"]
        pub fn get_acl(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([8, 115, 118, 149], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAccountFactory` (0x9068a868) function"]
        pub fn get_account_factory(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([144, 104, 168, 104], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAccountMiner` (0x0bc285ad) function"]
        pub fn get_account_miner(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([11, 194, 133, 173], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getContractsRegister` (0xc513c9bb) function"]
        pub fn get_contracts_register(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([197, 19, 201, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDataCompressor` (0x060678c2) function"]
        pub fn get_data_compressor(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([6, 6, 120, 194], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getGearToken` (0xaffd9243) function"]
        pub fn get_gear_token(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([175, 253, 146, 67], ())
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
        #[doc = "Calls the contract's `getTreasuryContract` (0x26c74fc3) function"]
        pub fn get_treasury_contract(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([38, 199, 79, 195], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getWETHGateway` (0x77532ed9) function"]
        pub fn get_weth_gateway(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([119, 83, 46, 217], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getWethToken` (0x4c252f91) function"]
        pub fn get_weth_token(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([76, 37, 47, 145], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setACL` (0x76aad605) function"]
        pub fn set_acl(
            &self,
            address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 170, 214, 5], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAccountFactory` (0xaddc1a76) function"]
        pub fn set_account_factory(
            &self,
            address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 220, 26, 118], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAccountMiner` (0x8978d480) function"]
        pub fn set_account_miner(
            &self,
            address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 120, 212, 128], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setContractsRegister` (0xce3c4ae4) function"]
        pub fn set_contracts_register(
            &self,
            address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 60, 74, 228], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDataCompressor` (0xc5120b39) function"]
        pub fn set_data_compressor(
            &self,
            address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([197, 18, 11, 57], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setGearToken` (0xbcaead98) function"]
        pub fn set_gear_token(
            &self,
            address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 174, 173, 152], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPriceOracle` (0x530e784f) function"]
        pub fn set_price_oracle(
            &self,
            address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 14, 120, 79], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTreasuryContract` (0x1ed65110) function"]
        pub fn set_treasury_contract(
            &self,
            address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 214, 81, 16], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setWETHGateway` (0x21da5837) function"]
        pub fn set_weth_gateway(
            &self,
            address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 218, 88, 55], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setWethToken` (0x86e09c08) function"]
        pub fn set_weth_token(
            &self,
            address: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 224, 156, 8], address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AddressSet` event"]
        pub fn address_set_filter(&self) -> ethers_contract::builders::Event<M, AddressSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, AddressProviderEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "AddressSet", abi = "AddressSet(bytes32,address)")]
    pub struct AddressSetFilter {
        #[ethevent(indexed)]
        pub service: [u8; 32],
        #[ethevent(indexed)]
        pub new_address: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers_core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum AddressProviderEvents {
        AddressSetFilter(AddressSetFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ethers_core::abi::Tokenizable for AddressProviderEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddressSetFilter::from_token(token.clone()) {
                return Ok(AddressProviderEvents::AddressSetFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::from_token(token.clone()) {
                return Ok(AddressProviderEvents::OwnershipTransferredFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                AddressProviderEvents::AddressSetFilter(element) => element.into_token(),
                AddressProviderEvents::OwnershipTransferredFilter(element) => element.into_token(),
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for AddressProviderEvents {}
    impl ethers_contract::EthLogDecode for AddressProviderEvents {
        fn decode_log(log: &ethers_core::abi::RawLog) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddressSetFilter::decode_log(log) {
                return Ok(AddressProviderEvents::AddressSetFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AddressProviderEvents::OwnershipTransferredFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}

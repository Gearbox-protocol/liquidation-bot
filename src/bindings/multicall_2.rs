pub use multicall2_mod::*;
#[allow(clippy::too_many_arguments)]
mod multicall2_mod {
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
    #[doc = "Multicall2 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MULTICALL2_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"target\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"callData\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct Multicall2.Call[]\",\n        \"name\": \"calls\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"aggregate\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"blockNumber\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes[]\",\n        \"name\": \"returnData\",\n        \"type\": \"bytes[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"target\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"callData\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct Multicall2.Call[]\",\n        \"name\": \"calls\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"blockAndAggregate\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"blockNumber\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"blockHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"success\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"returnData\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct Multicall2.Result[]\",\n        \"name\": \"returnData\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"blockNumber\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getBlockHash\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"blockHash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getBlockNumber\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"blockNumber\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getCurrentBlockCoinbase\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"coinbase\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getCurrentBlockDifficulty\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"difficulty\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getCurrentBlockGasLimit\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"gaslimit\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getCurrentBlockTimestamp\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"timestamp\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"addr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getEthBalance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"balance\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getLastBlockHash\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"blockHash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"requireSuccess\",\n        \"type\": \"bool\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"target\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"callData\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct Multicall2.Call[]\",\n        \"name\": \"calls\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"tryAggregate\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"success\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"returnData\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct Multicall2.Result[]\",\n        \"name\": \"returnData\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"requireSuccess\",\n        \"type\": \"bool\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"target\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"callData\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct Multicall2.Call[]\",\n        \"name\": \"calls\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"tryBlockAndAggregate\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"blockNumber\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"blockHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"success\",\n            \"type\": \"bool\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"returnData\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct Multicall2.Result[]\",\n        \"name\": \"returnData\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct Multicall2<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for Multicall2<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for Multicall2<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Multicall2))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> Multicall2<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), MULTICALL2_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `aggregate` (0x252dba42) function"]
        pub fn aggregate(
            &self,
            calls: Vec<(ethers_core::types::Address, Vec<u8>)>,
        ) -> ethers_contract::builders::ContractCall<M, (ethers_core::types::U256, Vec<Vec<u8>>)>
        {
            self.0
                .method_hash([37, 45, 186, 66], calls)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `blockAndAggregate` (0xc3077fa9) function"]
        pub fn block_and_aggregate(
            &self,
            calls: Vec<(ethers_core::types::Address, Vec<u8>)>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (ethers_core::types::U256, [u8; 32], Vec<(bool, Vec<u8>)>),
        > {
            self.0
                .method_hash([195, 7, 127, 169], calls)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBlockHash` (0xee82ac5e) function"]
        pub fn get_block_hash(
            &self,
            block_number: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([238, 130, 172, 94], block_number)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBlockNumber` (0x42cbb15c) function"]
        pub fn get_block_number(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([66, 203, 177, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBlockCoinbase` (0xa8b0574e) function"]
        pub fn get_current_block_coinbase(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([168, 176, 87, 78], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBlockDifficulty` (0x72425d9d) function"]
        pub fn get_current_block_difficulty(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([114, 66, 93, 157], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBlockGasLimit` (0x86d516e8) function"]
        pub fn get_current_block_gas_limit(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([134, 213, 22, 232], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBlockTimestamp` (0x0f28c97d) function"]
        pub fn get_current_block_timestamp(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([15, 40, 201, 125], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getEthBalance` (0x4d2301cc) function"]
        pub fn get_eth_balance(
            &self,
            addr: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([77, 35, 1, 204], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastBlockHash` (0x27e86d6e) function"]
        pub fn get_last_block_hash(&self) -> ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([39, 232, 109, 110], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tryAggregate` (0xbce38bd7) function"]
        pub fn try_aggregate(
            &self,
            require_success: bool,
            calls: Vec<(ethers_core::types::Address, Vec<u8>)>,
        ) -> ethers_contract::builders::ContractCall<M, Vec<(bool, Vec<u8>)>> {
            self.0
                .method_hash([188, 227, 139, 215], (require_success, calls))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tryBlockAndAggregate` (0x399542e9) function"]
        pub fn try_block_and_aggregate(
            &self,
            require_success: bool,
            calls: Vec<(ethers_core::types::Address, Vec<u8>)>,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (ethers_core::types::U256, [u8; 32], Vec<(bool, Vec<u8>)>),
        > {
            self.0
                .method_hash([57, 149, 66, 233], (require_success, calls))
                .expect("method not found (this should never happen)")
        }
    }
}

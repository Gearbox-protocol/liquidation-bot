pub use multi_call_2::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod multi_call_2 {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "MultiCall2 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"target\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"callData\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct Multicall2.Call[]\",\n          \"name\": \"calls\",\n          \"type\": \"tuple[]\"\n        }\n      ],\n      \"name\": \"aggregate\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"blockNumber\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"bytes[]\",\n          \"name\": \"returnData\",\n          \"type\": \"bytes[]\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"target\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"callData\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct Multicall2.Call[]\",\n          \"name\": \"calls\",\n          \"type\": \"tuple[]\"\n        }\n      ],\n      \"name\": \"blockAndAggregate\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"blockNumber\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"blockHash\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"success\",\n              \"type\": \"bool\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"returnData\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct Multicall2.Result[]\",\n          \"name\": \"returnData\",\n          \"type\": \"tuple[]\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"blockNumber\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"getBlockHash\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"blockHash\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getBlockNumber\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"blockNumber\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getCurrentBlockCoinbase\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"coinbase\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getCurrentBlockDifficulty\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"difficulty\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getCurrentBlockGasLimit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"gaslimit\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getCurrentBlockTimestamp\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"timestamp\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"addr\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"getEthBalance\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"balance\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getLastBlockHash\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"blockHash\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"requireSuccess\",\n          \"type\": \"bool\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"target\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"callData\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct Multicall2.Call[]\",\n          \"name\": \"calls\",\n          \"type\": \"tuple[]\"\n        }\n      ],\n      \"name\": \"tryAggregate\",\n      \"outputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"success\",\n              \"type\": \"bool\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"returnData\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct Multicall2.Result[]\",\n          \"name\": \"returnData\",\n          \"type\": \"tuple[]\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"requireSuccess\",\n          \"type\": \"bool\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"target\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"callData\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct Multicall2.Call[]\",\n          \"name\": \"calls\",\n          \"type\": \"tuple[]\"\n        }\n      ],\n      \"name\": \"tryBlockAndAggregate\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"blockNumber\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"blockHash\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"success\",\n              \"type\": \"bool\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"returnData\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"internalType\": \"struct Multicall2.Result[]\",\n          \"name\": \"returnData\",\n          \"type\": \"tuple[]\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    }\n  ]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MULTICALL2_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct MultiCall2<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MultiCall2<M> {
        fn clone(&self) -> Self {
            MultiCall2(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MultiCall2<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for MultiCall2<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MultiCall2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MultiCall2<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MULTICALL2_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `aggregate` (0x252dba42) function"]
        pub fn aggregate(
            &self,
            calls: ::std::vec::Vec<Call>,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ::std::vec::Vec<ethers::core::types::Bytes>,
            ),
        > {
            self.0
                .method_hash([37, 45, 186, 66], calls)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `blockAndAggregate` (0xc3077fa9) function"]
        pub fn block_and_aggregate(
            &self,
            calls: ::std::vec::Vec<Call>,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, [u8; 32], ::std::vec::Vec<Result>),
        > {
            self.0
                .method_hash([195, 7, 127, 169], calls)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBlockHash` (0xee82ac5e) function"]
        pub fn get_block_hash(
            &self,
            block_number: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([238, 130, 172, 94], block_number)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getBlockNumber` (0x42cbb15c) function"]
        pub fn get_block_number(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([66, 203, 177, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBlockCoinbase` (0xa8b0574e) function"]
        pub fn get_current_block_coinbase(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([168, 176, 87, 78], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBlockDifficulty` (0x72425d9d) function"]
        pub fn get_current_block_difficulty(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([114, 66, 93, 157], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBlockGasLimit` (0x86d516e8) function"]
        pub fn get_current_block_gas_limit(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([134, 213, 22, 232], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBlockTimestamp` (0x0f28c97d) function"]
        pub fn get_current_block_timestamp(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([15, 40, 201, 125], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getEthBalance` (0x4d2301cc) function"]
        pub fn get_eth_balance(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([77, 35, 1, 204], addr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastBlockHash` (0x27e86d6e) function"]
        pub fn get_last_block_hash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([39, 232, 109, 110], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tryAggregate` (0xbce38bd7) function"]
        pub fn try_aggregate(
            &self,
            require_success: bool,
            calls: ::std::vec::Vec<Call>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Result>> {
            self.0
                .method_hash([188, 227, 139, 215], (require_success, calls))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tryBlockAndAggregate` (0x399542e9) function"]
        pub fn try_block_and_aggregate(
            &self,
            require_success: bool,
            calls: ::std::vec::Vec<Call>,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, [u8; 32], ::std::vec::Vec<Result>),
        > {
            self.0
                .method_hash([57, 149, 66, 233], (require_success, calls))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MultiCall2<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `aggregate` function with signature `aggregate((address,bytes)[])` and selector `[37, 45, 186, 66]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "aggregate", abi = "aggregate((address,bytes)[])")]
    pub struct AggregateCall {
        pub calls: ::std::vec::Vec<Call>,
    }
    #[doc = "Container type for all input parameters for the `blockAndAggregate` function with signature `blockAndAggregate((address,bytes)[])` and selector `[195, 7, 127, 169]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "blockAndAggregate",
        abi = "blockAndAggregate((address,bytes)[])"
    )]
    pub struct BlockAndAggregateCall {
        pub calls: ::std::vec::Vec<Call>,
    }
    #[doc = "Container type for all input parameters for the `getBlockHash` function with signature `getBlockHash(uint256)` and selector `[238, 130, 172, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getBlockHash", abi = "getBlockHash(uint256)")]
    pub struct GetBlockHashCall {
        pub block_number: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getBlockNumber` function with signature `getBlockNumber()` and selector `[66, 203, 177, 92]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getBlockNumber", abi = "getBlockNumber()")]
    pub struct GetBlockNumberCall;
    #[doc = "Container type for all input parameters for the `getCurrentBlockCoinbase` function with signature `getCurrentBlockCoinbase()` and selector `[168, 176, 87, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCurrentBlockCoinbase", abi = "getCurrentBlockCoinbase()")]
    pub struct GetCurrentBlockCoinbaseCall;
    #[doc = "Container type for all input parameters for the `getCurrentBlockDifficulty` function with signature `getCurrentBlockDifficulty()` and selector `[114, 66, 93, 157]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getCurrentBlockDifficulty",
        abi = "getCurrentBlockDifficulty()"
    )]
    pub struct GetCurrentBlockDifficultyCall;
    #[doc = "Container type for all input parameters for the `getCurrentBlockGasLimit` function with signature `getCurrentBlockGasLimit()` and selector `[134, 213, 22, 232]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCurrentBlockGasLimit", abi = "getCurrentBlockGasLimit()")]
    pub struct GetCurrentBlockGasLimitCall;
    #[doc = "Container type for all input parameters for the `getCurrentBlockTimestamp` function with signature `getCurrentBlockTimestamp()` and selector `[15, 40, 201, 125]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCurrentBlockTimestamp", abi = "getCurrentBlockTimestamp()")]
    pub struct GetCurrentBlockTimestampCall;
    #[doc = "Container type for all input parameters for the `getEthBalance` function with signature `getEthBalance(address)` and selector `[77, 35, 1, 204]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getEthBalance", abi = "getEthBalance(address)")]
    pub struct GetEthBalanceCall {
        pub addr: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getLastBlockHash` function with signature `getLastBlockHash()` and selector `[39, 232, 109, 110]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getLastBlockHash", abi = "getLastBlockHash()")]
    pub struct GetLastBlockHashCall;
    #[doc = "Container type for all input parameters for the `tryAggregate` function with signature `tryAggregate(bool,(address,bytes)[])` and selector `[188, 227, 139, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "tryAggregate", abi = "tryAggregate(bool,(address,bytes)[])")]
    pub struct TryAggregateCall {
        pub require_success: bool,
        pub calls: ::std::vec::Vec<Call>,
    }
    #[doc = "Container type for all input parameters for the `tryBlockAndAggregate` function with signature `tryBlockAndAggregate(bool,(address,bytes)[])` and selector `[57, 149, 66, 233]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "tryBlockAndAggregate",
        abi = "tryBlockAndAggregate(bool,(address,bytes)[])"
    )]
    pub struct TryBlockAndAggregateCall {
        pub require_success: bool,
        pub calls: ::std::vec::Vec<Call>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MultiCall2Calls {
        Aggregate(AggregateCall),
        BlockAndAggregate(BlockAndAggregateCall),
        GetBlockHash(GetBlockHashCall),
        GetBlockNumber(GetBlockNumberCall),
        GetCurrentBlockCoinbase(GetCurrentBlockCoinbaseCall),
        GetCurrentBlockDifficulty(GetCurrentBlockDifficultyCall),
        GetCurrentBlockGasLimit(GetCurrentBlockGasLimitCall),
        GetCurrentBlockTimestamp(GetCurrentBlockTimestampCall),
        GetEthBalance(GetEthBalanceCall),
        GetLastBlockHash(GetLastBlockHashCall),
        TryAggregate(TryAggregateCall),
        TryBlockAndAggregate(TryBlockAndAggregateCall),
    }
    impl ethers::core::abi::AbiDecode for MultiCall2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AggregateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiCall2Calls::Aggregate(decoded));
            }
            if let Ok(decoded) =
                <BlockAndAggregateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiCall2Calls::BlockAndAggregate(decoded));
            }
            if let Ok(decoded) =
                <GetBlockHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiCall2Calls::GetBlockHash(decoded));
            }
            if let Ok(decoded) =
                <GetBlockNumberCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiCall2Calls::GetBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentBlockCoinbaseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiCall2Calls::GetCurrentBlockCoinbase(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentBlockDifficultyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MultiCall2Calls::GetCurrentBlockDifficulty(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentBlockGasLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiCall2Calls::GetCurrentBlockGasLimit(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentBlockTimestampCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(MultiCall2Calls::GetCurrentBlockTimestamp(decoded));
            }
            if let Ok(decoded) =
                <GetEthBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiCall2Calls::GetEthBalance(decoded));
            }
            if let Ok(decoded) =
                <GetLastBlockHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiCall2Calls::GetLastBlockHash(decoded));
            }
            if let Ok(decoded) =
                <TryAggregateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiCall2Calls::TryAggregate(decoded));
            }
            if let Ok(decoded) =
                <TryBlockAndAggregateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiCall2Calls::TryBlockAndAggregate(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MultiCall2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                MultiCall2Calls::Aggregate(element) => element.encode(),
                MultiCall2Calls::BlockAndAggregate(element) => element.encode(),
                MultiCall2Calls::GetBlockHash(element) => element.encode(),
                MultiCall2Calls::GetBlockNumber(element) => element.encode(),
                MultiCall2Calls::GetCurrentBlockCoinbase(element) => element.encode(),
                MultiCall2Calls::GetCurrentBlockDifficulty(element) => element.encode(),
                MultiCall2Calls::GetCurrentBlockGasLimit(element) => element.encode(),
                MultiCall2Calls::GetCurrentBlockTimestamp(element) => element.encode(),
                MultiCall2Calls::GetEthBalance(element) => element.encode(),
                MultiCall2Calls::GetLastBlockHash(element) => element.encode(),
                MultiCall2Calls::TryAggregate(element) => element.encode(),
                MultiCall2Calls::TryBlockAndAggregate(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MultiCall2Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MultiCall2Calls::Aggregate(element) => element.fmt(f),
                MultiCall2Calls::BlockAndAggregate(element) => element.fmt(f),
                MultiCall2Calls::GetBlockHash(element) => element.fmt(f),
                MultiCall2Calls::GetBlockNumber(element) => element.fmt(f),
                MultiCall2Calls::GetCurrentBlockCoinbase(element) => element.fmt(f),
                MultiCall2Calls::GetCurrentBlockDifficulty(element) => element.fmt(f),
                MultiCall2Calls::GetCurrentBlockGasLimit(element) => element.fmt(f),
                MultiCall2Calls::GetCurrentBlockTimestamp(element) => element.fmt(f),
                MultiCall2Calls::GetEthBalance(element) => element.fmt(f),
                MultiCall2Calls::GetLastBlockHash(element) => element.fmt(f),
                MultiCall2Calls::TryAggregate(element) => element.fmt(f),
                MultiCall2Calls::TryBlockAndAggregate(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AggregateCall> for MultiCall2Calls {
        fn from(var: AggregateCall) -> Self {
            MultiCall2Calls::Aggregate(var)
        }
    }
    impl ::std::convert::From<BlockAndAggregateCall> for MultiCall2Calls {
        fn from(var: BlockAndAggregateCall) -> Self {
            MultiCall2Calls::BlockAndAggregate(var)
        }
    }
    impl ::std::convert::From<GetBlockHashCall> for MultiCall2Calls {
        fn from(var: GetBlockHashCall) -> Self {
            MultiCall2Calls::GetBlockHash(var)
        }
    }
    impl ::std::convert::From<GetBlockNumberCall> for MultiCall2Calls {
        fn from(var: GetBlockNumberCall) -> Self {
            MultiCall2Calls::GetBlockNumber(var)
        }
    }
    impl ::std::convert::From<GetCurrentBlockCoinbaseCall> for MultiCall2Calls {
        fn from(var: GetCurrentBlockCoinbaseCall) -> Self {
            MultiCall2Calls::GetCurrentBlockCoinbase(var)
        }
    }
    impl ::std::convert::From<GetCurrentBlockDifficultyCall> for MultiCall2Calls {
        fn from(var: GetCurrentBlockDifficultyCall) -> Self {
            MultiCall2Calls::GetCurrentBlockDifficulty(var)
        }
    }
    impl ::std::convert::From<GetCurrentBlockGasLimitCall> for MultiCall2Calls {
        fn from(var: GetCurrentBlockGasLimitCall) -> Self {
            MultiCall2Calls::GetCurrentBlockGasLimit(var)
        }
    }
    impl ::std::convert::From<GetCurrentBlockTimestampCall> for MultiCall2Calls {
        fn from(var: GetCurrentBlockTimestampCall) -> Self {
            MultiCall2Calls::GetCurrentBlockTimestamp(var)
        }
    }
    impl ::std::convert::From<GetEthBalanceCall> for MultiCall2Calls {
        fn from(var: GetEthBalanceCall) -> Self {
            MultiCall2Calls::GetEthBalance(var)
        }
    }
    impl ::std::convert::From<GetLastBlockHashCall> for MultiCall2Calls {
        fn from(var: GetLastBlockHashCall) -> Self {
            MultiCall2Calls::GetLastBlockHash(var)
        }
    }
    impl ::std::convert::From<TryAggregateCall> for MultiCall2Calls {
        fn from(var: TryAggregateCall) -> Self {
            MultiCall2Calls::TryAggregate(var)
        }
    }
    impl ::std::convert::From<TryBlockAndAggregateCall> for MultiCall2Calls {
        fn from(var: TryBlockAndAggregateCall) -> Self {
            MultiCall2Calls::TryBlockAndAggregate(var)
        }
    }
    #[doc = "Container type for all return fields from the `aggregate` function with signature `aggregate((address,bytes)[])` and selector `[37, 45, 186, 66]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AggregateReturn {
        pub block_number: ethers::core::types::U256,
        pub return_data: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[doc = "Container type for all return fields from the `blockAndAggregate` function with signature `blockAndAggregate((address,bytes)[])` and selector `[195, 7, 127, 169]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BlockAndAggregateReturn {
        pub block_number: ethers::core::types::U256,
        pub block_hash: [u8; 32],
        pub return_data: ::std::vec::Vec<Result>,
    }
    #[doc = "Container type for all return fields from the `getBlockHash` function with signature `getBlockHash(uint256)` and selector `[238, 130, 172, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetBlockHashReturn {
        pub block_hash: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `getBlockNumber` function with signature `getBlockNumber()` and selector `[66, 203, 177, 92]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetBlockNumberReturn {
        pub block_number: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getCurrentBlockCoinbase` function with signature `getCurrentBlockCoinbase()` and selector `[168, 176, 87, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCurrentBlockCoinbaseReturn {
        pub coinbase: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `getCurrentBlockDifficulty` function with signature `getCurrentBlockDifficulty()` and selector `[114, 66, 93, 157]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCurrentBlockDifficultyReturn {
        pub difficulty: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getCurrentBlockGasLimit` function with signature `getCurrentBlockGasLimit()` and selector `[134, 213, 22, 232]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCurrentBlockGasLimitReturn {
        pub gaslimit: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getCurrentBlockTimestamp` function with signature `getCurrentBlockTimestamp()` and selector `[15, 40, 201, 125]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCurrentBlockTimestampReturn {
        pub timestamp: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getEthBalance` function with signature `getEthBalance(address)` and selector `[77, 35, 1, 204]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetEthBalanceReturn {
        pub balance: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getLastBlockHash` function with signature `getLastBlockHash()` and selector `[39, 232, 109, 110]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLastBlockHashReturn {
        pub block_hash: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `tryAggregate` function with signature `tryAggregate(bool,(address,bytes)[])` and selector `[188, 227, 139, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TryAggregateReturn {
        pub return_data: ::std::vec::Vec<Result>,
    }
    #[doc = "Container type for all return fields from the `tryBlockAndAggregate` function with signature `tryBlockAndAggregate(bool,(address,bytes)[])` and selector `[57, 149, 66, 233]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TryBlockAndAggregateReturn {
        pub block_number: ethers::core::types::U256,
        pub block_hash: [u8; 32],
        pub return_data: ::std::vec::Vec<Result>,
    }
    #[doc = "`Call(address,bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Call {
        pub target: ethers::core::types::Address,
        pub call_data: ethers::core::types::Bytes,
    }
    #[doc = "`Result(bool,bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Result {
        pub success: bool,
        pub return_data: ethers::core::types::Bytes,
    }
}

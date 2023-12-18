pub use i_contracts_register::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod i_contracts_register {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("creditManagers"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("creditManagers"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("i"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCreditManagers"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCreditManagers"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCreditManagersCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCreditManagersCount",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPools"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPools"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolsCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPoolsCount"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isCreditManager"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isCreditManager"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isPool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pools"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pools"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("i"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("version"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("version"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NewCreditManagerAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewCreditManagerAdded",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("creditManager"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewPoolAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewPoolAdded"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ICONTRACTSREGISTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IContractsRegister<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IContractsRegister<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IContractsRegister<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IContractsRegister<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IContractsRegister<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IContractsRegister))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IContractsRegister<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ICONTRACTSREGISTER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `creditManagers` (0x1e16e4fc) function
        pub fn credit_managers(
            &self,
            i: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([30, 22, 228, 252], i)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCreditManagers` (0x94144856) function
        pub fn get_credit_managers(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([148, 20, 72, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCreditManagersCount` (0xc29277cd) function
        pub fn get_credit_managers_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([194, 146, 119, 205], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPools` (0x673a2a1f) function
        pub fn get_pools(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([103, 58, 42, 31], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolsCount` (0xb4ac6860) function
        pub fn get_pools_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([180, 172, 104, 96], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isCreditManager` (0x6fbc6f6b) function
        pub fn is_credit_manager(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([111, 188, 111, 107], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPool` (0x5b16ebb7) function
        pub fn is_pool(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([91, 22, 235, 183], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pools` (0xac4afa38) function
        pub fn pools(
            &self,
            i: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([172, 74, 250, 56], i)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IContractsRegister<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "NewCreditManagerAdded", abi = "NewCreditManagerAdded(address)")]
    pub struct NewCreditManagerAddedFilter {
        #[ethevent(indexed)]
        pub credit_manager: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "NewPoolAdded", abi = "NewPoolAdded(address)")]
    pub struct NewPoolAddedFilter {
        #[ethevent(indexed)]
        pub pool: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum IContractsRegisterEvents {
        NewCreditManagerAddedFilter(NewCreditManagerAddedFilter),
        NewPoolAddedFilter(NewPoolAddedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IContractsRegisterEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = NewCreditManagerAddedFilter::decode_log(log) {
                return Ok(IContractsRegisterEvents::NewCreditManagerAddedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewPoolAddedFilter::decode_log(log) {
                return Ok(IContractsRegisterEvents::NewPoolAddedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IContractsRegisterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NewCreditManagerAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewPoolAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<NewCreditManagerAddedFilter> for IContractsRegisterEvents {
        fn from(value: NewCreditManagerAddedFilter) -> Self {
            Self::NewCreditManagerAddedFilter(value)
        }
    }
    impl ::core::convert::From<NewPoolAddedFilter> for IContractsRegisterEvents {
        fn from(value: NewPoolAddedFilter) -> Self {
            Self::NewPoolAddedFilter(value)
        }
    }
    ///Container type for all input parameters for the `creditManagers` function with signature `creditManagers(uint256)` and selector `0x1e16e4fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "creditManagers", abi = "creditManagers(uint256)")]
    pub struct CreditManagersCall {
        pub i: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getCreditManagers` function with signature `getCreditManagers()` and selector `0x94144856`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getCreditManagers", abi = "getCreditManagers()")]
    pub struct GetCreditManagersCall;
    ///Container type for all input parameters for the `getCreditManagersCount` function with signature `getCreditManagersCount()` and selector `0xc29277cd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getCreditManagersCount", abi = "getCreditManagersCount()")]
    pub struct GetCreditManagersCountCall;
    ///Container type for all input parameters for the `getPools` function with signature `getPools()` and selector `0x673a2a1f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPools", abi = "getPools()")]
    pub struct GetPoolsCall;
    ///Container type for all input parameters for the `getPoolsCount` function with signature `getPoolsCount()` and selector `0xb4ac6860`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPoolsCount", abi = "getPoolsCount()")]
    pub struct GetPoolsCountCall;
    ///Container type for all input parameters for the `isCreditManager` function with signature `isCreditManager(address)` and selector `0x6fbc6f6b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isCreditManager", abi = "isCreditManager(address)")]
    pub struct IsCreditManagerCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `isPool` function with signature `isPool(address)` and selector `0x5b16ebb7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isPool", abi = "isPool(address)")]
    pub struct IsPoolCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `pools` function with signature `pools(uint256)` and selector `0xac4afa38`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pools", abi = "pools(uint256)")]
    pub struct PoolsCall {
        pub i: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum IContractsRegisterCalls {
        CreditManagers(CreditManagersCall),
        GetCreditManagers(GetCreditManagersCall),
        GetCreditManagersCount(GetCreditManagersCountCall),
        GetPools(GetPoolsCall),
        GetPoolsCount(GetPoolsCountCall),
        IsCreditManager(IsCreditManagerCall),
        IsPool(IsPoolCall),
        Pools(PoolsCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for IContractsRegisterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CreditManagersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreditManagers(decoded));
            }
            if let Ok(decoded) =
                <GetCreditManagersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCreditManagers(decoded));
            }
            if let Ok(decoded) =
                <GetCreditManagersCountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCreditManagersCount(decoded));
            }
            if let Ok(decoded) = <GetPoolsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPools(decoded));
            }
            if let Ok(decoded) = <GetPoolsCountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolsCount(decoded));
            }
            if let Ok(decoded) =
                <IsCreditManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsCreditManager(decoded));
            }
            if let Ok(decoded) = <IsPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsPool(decoded));
            }
            if let Ok(decoded) = <PoolsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pools(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IContractsRegisterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CreditManagers(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCreditManagers(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCreditManagersCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolsCount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsCreditManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IContractsRegisterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CreditManagers(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCreditManagers(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCreditManagersCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPools(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolsCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsCreditManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CreditManagersCall> for IContractsRegisterCalls {
        fn from(value: CreditManagersCall) -> Self {
            Self::CreditManagers(value)
        }
    }
    impl ::core::convert::From<GetCreditManagersCall> for IContractsRegisterCalls {
        fn from(value: GetCreditManagersCall) -> Self {
            Self::GetCreditManagers(value)
        }
    }
    impl ::core::convert::From<GetCreditManagersCountCall> for IContractsRegisterCalls {
        fn from(value: GetCreditManagersCountCall) -> Self {
            Self::GetCreditManagersCount(value)
        }
    }
    impl ::core::convert::From<GetPoolsCall> for IContractsRegisterCalls {
        fn from(value: GetPoolsCall) -> Self {
            Self::GetPools(value)
        }
    }
    impl ::core::convert::From<GetPoolsCountCall> for IContractsRegisterCalls {
        fn from(value: GetPoolsCountCall) -> Self {
            Self::GetPoolsCount(value)
        }
    }
    impl ::core::convert::From<IsCreditManagerCall> for IContractsRegisterCalls {
        fn from(value: IsCreditManagerCall) -> Self {
            Self::IsCreditManager(value)
        }
    }
    impl ::core::convert::From<IsPoolCall> for IContractsRegisterCalls {
        fn from(value: IsPoolCall) -> Self {
            Self::IsPool(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for IContractsRegisterCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<VersionCall> for IContractsRegisterCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `creditManagers` function with signature `creditManagers(uint256)` and selector `0x1e16e4fc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CreditManagersReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getCreditManagers` function with signature `getCreditManagers()` and selector `0x94144856`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCreditManagersReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getCreditManagersCount` function with signature `getCreditManagersCount()` and selector `0xc29277cd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCreditManagersCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getPools` function with signature `getPools()` and selector `0x673a2a1f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPoolsReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getPoolsCount` function with signature `getPoolsCount()` and selector `0xb4ac6860`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPoolsCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isCreditManager` function with signature `isCreditManager(address)` and selector `0x6fbc6f6b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsCreditManagerReturn(pub bool);
    ///Container type for all return fields from the `isPool` function with signature `isPool(address)` and selector `0x5b16ebb7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsPoolReturn(pub bool);
    ///Container type for all return fields from the `pools` function with signature `pools(uint256)` and selector `0xac4afa38`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PoolsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VersionReturn(pub ::ethers::core::types::U256);
}

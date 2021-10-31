pub use icreditfilter_mod::*;
#[allow(clippy::too_many_arguments)]
mod icreditfilter_mod {
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
    #[doc = "ICreditFilter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static ICREDITFILTER_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"protocol\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"adapter\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ContractAllowed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"protocol\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ContractForbidden\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"chiThreshold\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"fastCheckDelay\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"NewFastCheckParameters\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidityThreshold\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"TokenAllowed\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"targetContract\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"adapter\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowContract\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidationThreshold\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"allowToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"id\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"allowedContracts\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"allowedContractsCount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"id\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"allowedTokens\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"allowedTokensCount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"calcCreditAccountAccruedInterest\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"calcCreditAccountHealthFactor\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"calcThresholdWeightedValue\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"total\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"calcTotalValue\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"total\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"checkAndEnableToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenIn\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenOut\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountIn\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountOut\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"checkCollateralChange\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amountIn\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amountOut\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"tokenIn\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"tokenOut\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"checkMultiTokenCollateral\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"poolService\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"connectCreditManager\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"allowedContract\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"contractToAdapter\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"enabledTokens\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"targetContract\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"forbidContract\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"id\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getCreditAccountTokenById\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"balance\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"tv\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"twv\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"creditAccount\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"initEnabledTokens\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isTokenAllowed\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"liquidationThresholds\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"priceOracle\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"revertIfTokenNotAllowed\",\n    \"outputs\": [],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"underlyingToken\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"updateUnderlyingTokenLiquidationThreshold\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct ICreditFilter<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for ICreditFilter<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for ICreditFilter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ICreditFilter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> ICreditFilter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers_contract::Contract::new(address.into(), ICREDITFILTER_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `allowContract` (0x7bccacee) function"]
        pub fn allow_contract(
            &self,
            target_contract: ethers_core::types::Address,
            adapter: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 204, 172, 238], (target_contract, adapter))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowToken` (0xa147c6c6) function"]
        pub fn allow_token(
            &self,
            token: ethers_core::types::Address,
            liquidation_threshold: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 71, 198, 198], (token, liquidation_threshold))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedContracts` (0x5094cb4f) function"]
        pub fn allowed_contracts(
            &self,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([80, 148, 203, 79], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedContractsCount` (0x50e036ff) function"]
        pub fn allowed_contracts_count(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([80, 224, 54, 255], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedTokens` (0x5e5f2e26) function"]
        pub fn allowed_tokens(
            &self,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([94, 95, 46, 38], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowedTokensCount` (0x20a05ff7) function"]
        pub fn allowed_tokens_count(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([32, 160, 95, 247], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcCreditAccountAccruedInterest` (0x3192195c) function"]
        pub fn calc_credit_account_accrued_interest(
            &self,
            credit_account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([49, 146, 25, 92], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcCreditAccountHealthFactor` (0xdfd59465) function"]
        pub fn calc_credit_account_health_factor(
            &self,
            credit_account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([223, 213, 148, 101], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcThresholdWeightedValue` (0x90b1300a) function"]
        pub fn calc_threshold_weighted_value(
            &self,
            credit_account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([144, 177, 48, 10], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcTotalValue` (0xc7de38a6) function"]
        pub fn calc_total_value(
            &self,
            credit_account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([199, 222, 56, 166], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkAndEnableToken` (0x51e3f160) function"]
        pub fn check_and_enable_token(
            &self,
            credit_account: ethers_core::types::Address,
            token: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 227, 241, 96], (credit_account, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkCollateralChange` (0xe1c8ef0d) function"]
        pub fn check_collateral_change(
            &self,
            credit_account: ethers_core::types::Address,
            token_in: ethers_core::types::Address,
            token_out: ethers_core::types::Address,
            amount_in: ethers_core::types::U256,
            amount_out: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [225, 200, 239, 13],
                    (credit_account, token_in, token_out, amount_in, amount_out),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkMultiTokenCollateral` (0x7e4a6863) function"]
        pub fn check_multi_token_collateral(
            &self,
            credit_account: ethers_core::types::Address,
            amount_in: Vec<ethers_core::types::U256>,
            amount_out: Vec<ethers_core::types::U256>,
            token_in: Vec<ethers_core::types::Address>,
            token_out: Vec<ethers_core::types::Address>,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [126, 74, 104, 99],
                    (credit_account, amount_in, amount_out, token_in, token_out),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `connectCreditManager` (0xcf33d955) function"]
        pub fn connect_credit_manager(
            &self,
            pool_service: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 51, 217, 85], pool_service)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `contractToAdapter` (0xfdd57645) function"]
        pub fn contract_to_adapter(
            &self,
            allowed_contract: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([253, 213, 118, 69], allowed_contract)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enabledTokens` (0xb451cecc) function"]
        pub fn enabled_tokens(
            &self,
            credit_account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([180, 81, 206, 204], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forbidContract` (0x52438e54) function"]
        pub fn forbid_contract(
            &self,
            target_contract: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([82, 67, 142, 84], target_contract)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditAccountTokenById` (0xaf0a6502) function"]
        pub fn get_credit_account_token_by_id(
            &self,
            credit_account: ethers_core::types::Address,
            id: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash([175, 10, 101, 2], (credit_account, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initEnabledTokens` (0xe54fe9c8) function"]
        pub fn init_enabled_tokens(
            &self,
            credit_account: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 79, 233, 200], credit_account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isTokenAllowed` (0xf9eaee0d) function"]
        pub fn is_token_allowed(
            &self,
            token: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([249, 234, 238, 13], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidationThresholds` (0x78327438) function"]
        pub fn liquidation_thresholds(
            &self,
            token: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([120, 50, 116, 56], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `priceOracle` (0x2630c12f) function"]
        pub fn price_oracle(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([38, 48, 193, 47], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revertIfTokenNotAllowed` (0x7dd0ba82) function"]
        pub fn revert_if_token_not_allowed(
            &self,
            token: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([125, 208, 186, 130], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `underlyingToken` (0x2495a599) function"]
        pub fn underlying_token(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([36, 149, 165, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateUnderlyingTokenLiquidationThreshold` (0x40631828) function"]
        pub fn update_underlying_token_liquidation_threshold(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 99, 24, 40], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ContractAllowed` event"]
        pub fn contract_allowed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ContractAllowedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ContractForbidden` event"]
        pub fn contract_forbidden_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, ContractForbiddenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewFastCheckParameters` event"]
        pub fn new_fast_check_parameters_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, NewFastCheckParametersFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenAllowed` event"]
        pub fn token_allowed_filter(
            &self,
        ) -> ethers_contract::builders::Event<M, TokenAllowedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers_contract::builders::Event<M, ICreditFilterEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "ContractAllowed", abi = "ContractAllowed(address,address)")]
    pub struct ContractAllowedFilter {
        #[ethevent(indexed)]
        pub protocol: ethers_core::types::Address,
        #[ethevent(indexed)]
        pub adapter: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "ContractForbidden", abi = "ContractForbidden(address)")]
    pub struct ContractForbiddenFilter {
        #[ethevent(indexed)]
        pub protocol: ethers_core::types::Address,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(
        name = "NewFastCheckParameters",
        abi = "NewFastCheckParameters(uint256,uint256)"
    )]
    pub struct NewFastCheckParametersFilter {
        pub chi_threshold: ethers_core::types::U256,
        pub fast_check_delay: ethers_core::types::U256,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq, ethers_contract :: EthEvent)]
    #[ethevent(name = "TokenAllowed", abi = "TokenAllowed(address,uint256)")]
    pub struct TokenAllowedFilter {
        #[ethevent(indexed)]
        pub token: ethers_core::types::Address,
        pub liquidity_threshold: ethers_core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ICreditFilterEvents {
        ContractAllowedFilter(ContractAllowedFilter),
        ContractForbiddenFilter(ContractForbiddenFilter),
        NewFastCheckParametersFilter(NewFastCheckParametersFilter),
        TokenAllowedFilter(TokenAllowedFilter),
    }
    impl ethers_core::abi::Tokenizable for ICreditFilterEvents {
        fn from_token(
            token: ethers_core::abi::Token,
        ) -> Result<Self, ethers_core::abi::InvalidOutputType>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ContractAllowedFilter::from_token(token.clone()) {
                return Ok(ICreditFilterEvents::ContractAllowedFilter(decoded));
            }
            if let Ok(decoded) = ContractForbiddenFilter::from_token(token.clone()) {
                return Ok(ICreditFilterEvents::ContractForbiddenFilter(decoded));
            }
            if let Ok(decoded) = NewFastCheckParametersFilter::from_token(token.clone()) {
                return Ok(ICreditFilterEvents::NewFastCheckParametersFilter(decoded));
            }
            if let Ok(decoded) = TokenAllowedFilter::from_token(token.clone()) {
                return Ok(ICreditFilterEvents::TokenAllowedFilter(decoded));
            }
            Err(ethers_core::abi::InvalidOutputType(
                "Failed to decode all event variants".to_string(),
            ))
        }
        fn into_token(self) -> ethers_core::abi::Token {
            match self {
                ICreditFilterEvents::ContractAllowedFilter(element) => element.into_token(),
                ICreditFilterEvents::ContractForbiddenFilter(element) => element.into_token(),
                ICreditFilterEvents::NewFastCheckParametersFilter(element) => element.into_token(),
                ICreditFilterEvents::TokenAllowedFilter(element) => element.into_token(),
            }
        }
    }
    impl ethers_core::abi::TokenizableItem for ICreditFilterEvents {}
    impl ethers_contract::EthLogDecode for ICreditFilterEvents {
        fn decode_log(log: &ethers_core::abi::RawLog) -> Result<Self, ethers_core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ContractAllowedFilter::decode_log(log) {
                return Ok(ICreditFilterEvents::ContractAllowedFilter(decoded));
            }
            if let Ok(decoded) = ContractForbiddenFilter::decode_log(log) {
                return Ok(ICreditFilterEvents::ContractForbiddenFilter(decoded));
            }
            if let Ok(decoded) = NewFastCheckParametersFilter::decode_log(log) {
                return Ok(ICreditFilterEvents::NewFastCheckParametersFilter(decoded));
            }
            if let Ok(decoded) = TokenAllowedFilter::decode_log(log) {
                return Ok(ICreditFilterEvents::TokenAllowedFilter(decoded));
            }
            Err(ethers_core::abi::Error::InvalidData)
        }
    }
}

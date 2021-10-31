pub use iappcreditmanager_mod::*;
#[allow(clippy::too_many_arguments)]
mod iappcreditmanager_mod {
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
    #[doc = "IAppCreditManager was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IAPPCREDITMANAGER_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"addCollateral\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isLiquidated\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"calcRepayAmount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address[]\",\n            \"name\": \"path\",\n            \"type\": \"address[]\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amountOutMin\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct DataTypes.Exchange[]\",\n        \"name\": \"paths\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"closeCreditAccount\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"defaultSwapContract\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getCreditAccountOrRevert\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"borrower\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"hasOpenedCreditAccount\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"increaseBorrowedAmount\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"onBehalfOf\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"leverageFactor\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"referralCode\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"openCreditAccount\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"repayCreditAccount\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct IAppCreditManager<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for IAppCreditManager<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for IAppCreditManager<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IAppCreditManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> IAppCreditManager<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                IAPPCREDITMANAGER_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `addCollateral` (0x59781034) function"]
        pub fn add_collateral(
            &self,
            on_behalf_of: ethers_core::types::Address,
            token: ethers_core::types::Address,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 120, 16, 52], (on_behalf_of, token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calcRepayAmount` (0x3ce07355) function"]
        pub fn calc_repay_amount(
            &self,
            borrower: ethers_core::types::Address,
            is_liquidated: bool,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([60, 224, 115, 85], (borrower, is_liquidated))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `closeCreditAccount` (0xab114805) function"]
        pub fn close_credit_account(
            &self,
            to: ethers_core::types::Address,
            paths: Vec<(Vec<ethers_core::types::Address>, ethers_core::types::U256)>,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 17, 72, 5], (to, paths))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `defaultSwapContract` (0xe0c011b7) function"]
        pub fn default_swap_contract(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([224, 192, 17, 183], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditAccountOrRevert` (0xe958b704) function"]
        pub fn get_credit_account_or_revert(
            &self,
            borrower: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([233, 88, 183, 4], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasOpenedCreditAccount` (0x256ac915) function"]
        pub fn has_opened_credit_account(
            &self,
            borrower: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([37, 106, 201, 21], borrower)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseBorrowedAmount` (0x9efc60d0) function"]
        pub fn increase_borrowed_amount(
            &self,
            amount: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 252, 96, 208], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `openCreditAccount` (0x5288ba4b) function"]
        pub fn open_credit_account(
            &self,
            amount: ethers_core::types::U256,
            on_behalf_of: ethers_core::types::Address,
            leverage_factor: ethers_core::types::U256,
            referral_code: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [82, 136, 186, 75],
                    (amount, on_behalf_of, leverage_factor, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayCreditAccount` (0xd692ba33) function"]
        pub fn repay_credit_account(
            &self,
            to: ethers_core::types::Address,
        ) -> ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 146, 186, 51], to)
                .expect("method not found (this should never happen)")
        }
    }
}

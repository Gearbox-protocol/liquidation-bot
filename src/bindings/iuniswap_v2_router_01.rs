pub use iuniswapv2router01_mod::*;
#[allow(clippy::too_many_arguments)]
mod iuniswapv2router01_mod {
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
    #[doc = "IUniswapV2Router01 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IUNISWAPV2ROUTER01_ABI: ethers_contract::Lazy<ethers_core::abi::Abi> =
        ethers_contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [],\n    \"name\": \"WETH\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenA\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenB\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountADesired\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountBDesired\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountAMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountBMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"addLiquidity\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountA\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountB\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountTokenDesired\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountTokenMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountETHMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"addLiquidityETH\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountToken\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountETH\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"factory\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountOut\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"reserveIn\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"reserveOut\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getAmountIn\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountIn\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountIn\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"reserveIn\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"reserveOut\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getAmountOut\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountOut\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountOut\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"path\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"getAmountsIn\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amounts\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountIn\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"path\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"getAmountsOut\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amounts\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountA\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"reserveA\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"reserveB\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"quote\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountB\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenA\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenB\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountAMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountBMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"removeLiquidity\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountA\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountB\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountTokenMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountETHMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"removeLiquidityETH\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountToken\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountETH\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountTokenMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountETHMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"approveMax\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"v\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"r\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"s\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"removeLiquidityETHWithPermit\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountToken\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountETH\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenA\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"tokenB\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountAMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountBMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"approveMax\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"v\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"r\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"s\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"removeLiquidityWithPermit\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountA\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountB\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountOut\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"path\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"swapETHForExactTokens\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amounts\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountOutMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"path\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"swapExactETHForTokens\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amounts\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountIn\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountOutMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"path\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"swapExactTokensForETH\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amounts\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountIn\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountOutMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"path\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"swapExactTokensForTokens\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amounts\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountOut\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountInMax\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"path\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"swapTokensForExactETH\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amounts\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountOut\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountInMax\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"path\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"swapTokensForExactTokens\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amounts\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct IUniswapV2Router01<M>(ethers_contract::Contract<M>);
    impl<M> std::ops::Deref for IUniswapV2Router01<M> {
        type Target = ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers_providers::Middleware> std::fmt::Debug for IUniswapV2Router01<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV2Router01))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers_providers::Middleware> IUniswapV2Router01<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers_contract::Contract::new(
                address.into(),
                IUNISWAPV2ROUTER01_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `WETH` (0xad5c4648) function"]
        pub fn weth(
            &self,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::Address> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidity` (0xe8e33700) function"]
        pub fn add_liquidity(
            &self,
            token_a: ethers_core::types::Address,
            token_b: ethers_core::types::Address,
            amount_a_desired: ethers_core::types::U256,
            amount_b_desired: ethers_core::types::U256,
            amount_a_min: ethers_core::types::U256,
            amount_b_min: ethers_core::types::U256,
            to: ethers_core::types::Address,
            deadline: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [232, 227, 55, 0],
                    (
                        token_a,
                        token_b,
                        amount_a_desired,
                        amount_b_desired,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidityETH` (0xf305d719) function"]
        pub fn add_liquidity_eth(
            &self,
            token: ethers_core::types::Address,
            amount_token_desired: ethers_core::types::U256,
            amount_token_min: ethers_core::types::U256,
            amount_eth_min: ethers_core::types::U256,
            to: ethers_core::types::Address,
            deadline: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [243, 5, 215, 25],
                    (
                        token,
                        amount_token_desired,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
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
        #[doc = "Calls the contract's `getAmountIn` (0x85f8c259) function"]
        pub fn get_amount_in(
            &self,
            amount_out: ethers_core::types::U256,
            reserve_in: ethers_core::types::U256,
            reserve_out: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([133, 248, 194, 89], (amount_out, reserve_in, reserve_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountOut` (0x054d50d4) function"]
        pub fn get_amount_out(
            &self,
            amount_in: ethers_core::types::U256,
            reserve_in: ethers_core::types::U256,
            reserve_out: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([5, 77, 80, 212], (amount_in, reserve_in, reserve_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountsIn` (0x1f00ca74) function"]
        pub fn get_amounts_in(
            &self,
            amount_out: ethers_core::types::U256,
            path: Vec<ethers_core::types::Address>,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash([31, 0, 202, 116], (amount_out, path))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountsOut` (0xd06ca61f) function"]
        pub fn get_amounts_out(
            &self,
            amount_in: ethers_core::types::U256,
            path: Vec<ethers_core::types::Address>,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash([208, 108, 166, 31], (amount_in, path))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `quote` (0xad615dec) function"]
        pub fn quote(
            &self,
            amount_a: ethers_core::types::U256,
            reserve_a: ethers_core::types::U256,
            reserve_b: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, ethers_core::types::U256> {
            self.0
                .method_hash([173, 97, 93, 236], (amount_a, reserve_a, reserve_b))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidity` (0xbaa2abde) function"]
        pub fn remove_liquidity(
            &self,
            token_a: ethers_core::types::Address,
            token_b: ethers_core::types::Address,
            liquidity: ethers_core::types::U256,
            amount_a_min: ethers_core::types::U256,
            amount_b_min: ethers_core::types::U256,
            to: ethers_core::types::Address,
            deadline: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (ethers_core::types::U256, ethers_core::types::U256),
        > {
            self.0
                .method_hash(
                    [186, 162, 171, 222],
                    (
                        token_a,
                        token_b,
                        liquidity,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETH` (0x02751cec) function"]
        pub fn remove_liquidity_eth(
            &self,
            token: ethers_core::types::Address,
            liquidity: ethers_core::types::U256,
            amount_token_min: ethers_core::types::U256,
            amount_eth_min: ethers_core::types::U256,
            to: ethers_core::types::Address,
            deadline: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<
            M,
            (ethers_core::types::U256, ethers_core::types::U256),
        > {
            self.0
                .method_hash(
                    [2, 117, 28, 236],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETHWithPermit` (0xded9382a) function"]
        pub fn remove_liquidity_eth_with_permit(
            &self,
            token: ethers_core::types::Address,
            liquidity: ethers_core::types::U256,
            amount_token_min: ethers_core::types::U256,
            amount_eth_min: ethers_core::types::U256,
            to: ethers_core::types::Address,
            deadline: ethers_core::types::U256,
            approve_max: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers_contract::builders::ContractCall<
            M,
            (ethers_core::types::U256, ethers_core::types::U256),
        > {
            self.0
                .method_hash(
                    [222, 217, 56, 42],
                    (
                        token,
                        liquidity,
                        amount_token_min,
                        amount_eth_min,
                        to,
                        deadline,
                        approve_max,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityWithPermit` (0x2195995c) function"]
        pub fn remove_liquidity_with_permit(
            &self,
            token_a: ethers_core::types::Address,
            token_b: ethers_core::types::Address,
            liquidity: ethers_core::types::U256,
            amount_a_min: ethers_core::types::U256,
            amount_b_min: ethers_core::types::U256,
            to: ethers_core::types::Address,
            deadline: ethers_core::types::U256,
            approve_max: bool,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers_contract::builders::ContractCall<
            M,
            (ethers_core::types::U256, ethers_core::types::U256),
        > {
            self.0
                .method_hash(
                    [33, 149, 153, 92],
                    (
                        token_a,
                        token_b,
                        liquidity,
                        amount_a_min,
                        amount_b_min,
                        to,
                        deadline,
                        approve_max,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapETHForExactTokens` (0xfb3bdb41) function"]
        pub fn swap_eth_for_exact_tokens(
            &self,
            amount_out: ethers_core::types::U256,
            path: Vec<ethers_core::types::Address>,
            to: ethers_core::types::Address,
            deadline: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash([251, 59, 219, 65], (amount_out, path, to, deadline))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactETHForTokens` (0x7ff36ab5) function"]
        pub fn swap_exact_eth_for_tokens(
            &self,
            amount_out_min: ethers_core::types::U256,
            path: Vec<ethers_core::types::Address>,
            to: ethers_core::types::Address,
            deadline: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash([127, 243, 106, 181], (amount_out_min, path, to, deadline))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForETH` (0x18cbafe5) function"]
        pub fn swap_exact_tokens_for_eth(
            &self,
            amount_in: ethers_core::types::U256,
            amount_out_min: ethers_core::types::U256,
            path: Vec<ethers_core::types::Address>,
            to: ethers_core::types::Address,
            deadline: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash(
                    [24, 203, 175, 229],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactTokensForTokens` (0x38ed1739) function"]
        pub fn swap_exact_tokens_for_tokens(
            &self,
            amount_in: ethers_core::types::U256,
            amount_out_min: ethers_core::types::U256,
            path: Vec<ethers_core::types::Address>,
            to: ethers_core::types::Address,
            deadline: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash(
                    [56, 237, 23, 57],
                    (amount_in, amount_out_min, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapTokensForExactETH` (0x4a25d94a) function"]
        pub fn swap_tokens_for_exact_eth(
            &self,
            amount_out: ethers_core::types::U256,
            amount_in_max: ethers_core::types::U256,
            path: Vec<ethers_core::types::Address>,
            to: ethers_core::types::Address,
            deadline: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash(
                    [74, 37, 217, 74],
                    (amount_out, amount_in_max, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapTokensForExactTokens` (0x8803dbee) function"]
        pub fn swap_tokens_for_exact_tokens(
            &self,
            amount_out: ethers_core::types::U256,
            amount_in_max: ethers_core::types::U256,
            path: Vec<ethers_core::types::Address>,
            to: ethers_core::types::Address,
            deadline: ethers_core::types::U256,
        ) -> ethers_contract::builders::ContractCall<M, Vec<ethers_core::types::U256>> {
            self.0
                .method_hash(
                    [136, 3, 219, 238],
                    (amount_out, amount_in_max, path, to, deadline),
                )
                .expect("method not found (this should never happen)")
        }
    }
}

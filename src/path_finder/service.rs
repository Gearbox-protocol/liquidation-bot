use std::sync::Arc;

use ethers::abi::ethereum_types::{Address, U256};
use ethers::prelude::{Middleware, StreamExt};

use crate::bindings::path_finder::PathFinder as PathFinderContract;
use crate::config::Config;
use crate::errors::LiquidationError;
use crate::errors::LiquidationError::NetError;

const UNISWAP_ADDRESS: &str = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
const EXACT_INPUT: u64 = 1;

#[derive(Debug)]
pub struct TradePath {
    pub(crate) path: Vec<Address>,
    pub(crate) amount_out_min: U256,
}

pub struct PathFinder<M: Middleware> {
    contract: PathFinderContract<M>,
    tokens_middle: Vec<Address>,
    swap_interface: U256,
    pub(crate) router: Address,
}

impl<M: Middleware> PathFinder<M> {
    pub fn new(config: &Config, client: Arc<M>) -> Self {
        let contract = PathFinderContract::new(config.path_finder, client.clone());

        let tokens_middle = vec![
            // DAI
            "0x9DC7B33C3B63fc00ed5472fBD7813eDDa6a64752",
            // USDC
            "0x31EeB2d0F9B6fD8642914aB10F4dD473677D80df",
            // WBTC
            "0xE36bC5d8b689AD6d80e78c3e736670e80d4b329D",
            // ETH
            "0xd0a1e359811322d97991e03f863a0c30c2cf029c",
        ];

        let tokens_middle = tokens_middle
            .iter()
            .map(|t| {
                Address::from_slice(
                    hex::decode(t.strip_prefix("0x").unwrap())
                        .unwrap()
                        .as_slice(),
                )
            })
            .collect::<Vec<Address>>();

        let swap_interface = U256::from(1);

        let router = hex::decode(UNISWAP_ADDRESS.strip_prefix("0x").unwrap())
            .expect("Decoding of uniswap address failed");

        PathFinder {
            contract,
            tokens_middle,
            swap_interface,
            router: Address::from_slice(router.as_slice()),
        }
    }

    pub async fn get_best_rate(
        &self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<TradePath, LiquidationError> {
        if amount == U256::from(0) {
            return Ok(TradePath {
                path: vec![],
                amount_out_min: Default::default(),
            });
        };

        let result = self
            .contract
            .best_uni_path(
                self.swap_interface,
                self.router,
                U256::from(EXACT_INPUT),
                from,
                to,
                amount,
                self.tokens_middle.clone(),
            )
            .call()
            .await
            .map_err(|err| NetError("cant get best uni price".to_string()))?;

        Ok(TradePath {
            path: result.0,
            amount_out_min: result.2 * 99 / 100,
        })
    }
}

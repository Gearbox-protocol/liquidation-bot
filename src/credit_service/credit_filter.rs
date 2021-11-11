use std::collections::HashMap;

use ethers::abi::Address;
use ethers::core::abi::ethereum_types::U64;
use ethers::prelude::{Middleware, U256};

use crate::bindings::credit_filter::CreditFilter as CreditFilterContract;

pub struct CreditFilter<M: Middleware> {
    contract: CreditFilterContract<M>,
    pub liquidation_thresholds: HashMap<Address, U256>,
}

impl<M: Middleware> CreditFilter<M> {
    pub fn new(address: Address, client: std::sync::Arc<M>) -> Self {
        let contract = CreditFilterContract::new(address, client.clone());
        CreditFilter {
            contract,
            liquidation_thresholds: HashMap::new(),
        }
    }

    pub async fn update(&mut self, from_block: &U64, to_block: &U64) {
        let events = self
            .contract
            .token_allowed_filter()
            .from_block(from_block)
            .to_block(to_block)
            .query()
            .await
            .unwrap();

        for event in events {
            if (self.liquidation_thresholds.contains_key(&event.token)) {
                *self.liquidation_thresholds.get_mut(&event.token).unwrap() =
                    event.liquidity_threshold;
            } else {
                self.liquidation_thresholds
                    .insert(event.token, event.liquidity_threshold);
            }
        }

        println!(
            "Got liquidataion thresolds {:?}",
            &self.liquidation_thresholds
        );
    }
}

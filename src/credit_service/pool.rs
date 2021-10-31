use ethers::abi::Address;
use ethers::prelude::{Middleware, U256};

use crate::bindings::pool_service::PoolService as PoolContract;

pub struct PoolService<M: Middleware> {
    contract: PoolContract<M>,
    pub cumulative_index: U256,
}

impl <M: Middleware> PoolService<M> {
    pub fn new(address: Address, client: std::sync::Arc<M>) -> Self {
        let contract = PoolContract::new(address, client.clone());
        PoolService { contract, cumulative_index: 0.into() }
    }

    pub async fn get_new_ci(&mut self) -> U256 {
        self.contract.cumulative_index_ray().call().await.unwrap()
    }
}
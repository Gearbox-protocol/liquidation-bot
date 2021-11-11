use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use ethers::prelude::*;

use crate::credit_service::credit_filter::CreditFilter;
use crate::errors::LiquidationError;
use crate::price_oracle::oracle::PriceOracle;

pub struct CreditAccount {
    pub contract: Address,
    pub borrower: Address,
    pub borrowed_amount: U256,
    pub cumulative_index_at_open: U256,
    pub balances: HashMap<Address, U256>,
    pub health_factor: u64,
}

impl Debug for CreditAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Borrower: {:?}\nBorrowed amount: {}\nAddress:{:?}\ncum_index: {}\nhealth_factor: {}\nbalances: {:?}",
            self.borrower,  self.borrowed_amount, self.contract, self.cumulative_index_at_open, self.health_factor, self.balances
        )
    }
}

impl CreditAccount {
    pub fn compute_hf<M: Middleware, S: Signer>(
        &mut self,
        underlying_token: Address,
        cumulative_index_now: &U256,
        price_oracle: &PriceOracle<M, S>,
        credit_filter: &CreditFilter<SignerMiddleware<M, S>>,
    ) -> Result<u64, LiquidationError> {
        let mut total: U256 = 0.into();
        for asset in self.balances.clone() {
            total += price_oracle.convert(asset.1, asset.0, underlying_token)?
                * credit_filter.liquidation_thresholds.get(&asset.0).unwrap();
        }

        let borrowed_amount_plus_interest =
            self.borrowed_amount * cumulative_index_now / self.cumulative_index_at_open;
        self.health_factor = ((total / borrowed_amount_plus_interest).as_u64()) ;

        Ok(self.health_factor)
    }
}

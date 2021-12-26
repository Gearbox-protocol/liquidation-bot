use crate::ampq_service::AmpqService;
use async_recursion::async_recursion;
use ethers::prelude::*;
use futures::{stream, StreamExt};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::rc::Rc;
use std::vec::Vec;

use crate::bindings::credit_manager::CreditManager as CM;
use crate::bindings::{
    CloseCreditAccountFilter, CreditManagerEvents, DataCompressor, LiquidateCreditAccountFilter,
    OpenCreditAccountFilter, RepayCreditAccountFilter,
};
use crate::config::config::str_to_address;
use crate::credit_service::credit_account::CreditAccount;
use crate::credit_service::credit_filter::CreditFilter;
use crate::credit_service::pool::PoolService;
use crate::errors::LiquidationError;
use crate::errors::LiquidationError::NetError;
use crate::path_finder::service::TradePath;
use crate::path_finder::PathFinder;
use crate::price_oracle::oracle::PriceOracle;
use crate::terminator_service::terminator::TerminatorJob;

pub struct CreditManager<M: Middleware, S: Signer> {
    credit_accounts: HashMap<Address, CreditAccount>,
    added_to_job: HashMap<Address, u8>,

    address: ethers_core::types::Address,
    underlying_token: ethers_core::types::Address,
    is_weth: bool,
    // can_borrow: bool,
    // borrow_rate: ethers_core::types::U256,
    // min_amount: ethers_core::types::U256,
    // max_amount: ethers_core::types::U256,
    // max_leverage_factor: ethers_core::types::U256,
    // available_liquidity: ethers_core::types::U256,
    pub allowed_tokens: Vec<ethers_core::types::Address>,
    // adapters: Vec<(ethers_core::types::Address, ethers_core::types::Address)>,
    contract: CM<SignerMiddleware<M, S>>,
    data_compressor: DataCompressor<SignerMiddleware<M, S>>,
    pool_service: PoolService<SignerMiddleware<M, S>>,
    credit_filter: CreditFilter<SignerMiddleware<M, S>>,
    yearn_tokens: HashMap<Address, Address>,
    ampq_service: AmpqService,
}

impl<M: Middleware, S: Signer> CreditManager<M, S> {
    pub async fn new(
        client: std::sync::Arc<SignerMiddleware<M, S>>,
        payload: &(
            ethers_core::types::Address,
            bool,
            ethers_core::types::Address,
            bool,
            bool,
            ethers_core::types::U256,
            ethers_core::types::U256,
            ethers_core::types::U256,
            ethers_core::types::U256,
            ethers_core::types::U256,
            Vec<ethers_core::types::Address>,
            Vec<(ethers_core::types::Address, ethers_core::types::Address)>,
        ),
        data_compressor: DataCompressor<SignerMiddleware<M, S>>,
        chain_id: u64,
        ampq_service: AmpqService,
    ) -> Self {
        let contract = CM::new(payload.0, client.clone());
        let pool_service_address = contract.pool_service().call().await.unwrap();
        let pool_service = PoolService::new(pool_service_address, client.clone());

        let credit_filter_address = contract.credit_filter().call().await.unwrap();
        let credit_filter = CreditFilter::new(credit_filter_address, client.clone());

        let mut yearn_tokens: HashMap<Address, Address> = HashMap::new();

        if chain_id == 42 {
            // KOVAN YEARN TOKENS
            // DAI
            yearn_tokens.insert(
                str_to_address(String::from("0x67A022C14E1e6517F45E92BF7C76249c0967569d")),
                str_to_address(String::from("0x9DC7B33C3B63fc00ed5472fBD7813eDDa6a64752")),
            );

            yearn_tokens.insert(
                str_to_address(String::from("0xe5267045739E4d6FcA15BB4a79190012F146893b")),
                str_to_address(String::from("0x9DC7B33C3B63fc00ed5472fBD7813eDDa6a64752")),
            );

            // USDC
            yearn_tokens.insert(
                str_to_address(String::from("0x3B55a47d6ffE0b7bb1762109faFa5B84180c1111")),
                str_to_address(String::from("0x31EeB2d0F9B6fD8642914aB10F4dD473677D80df")),
            );

            yearn_tokens.insert(
                str_to_address(String::from("0x980E4d8A22105c2a2fA2252B7685F32fc7564512")),
                str_to_address(String::from("0x31EeB2d0F9B6fD8642914aB10F4dD473677D80df")),
            );
        } else {
            // MAINNET YEARN TOKENS
            // DAI
            yearn_tokens.insert(
                str_to_address(String::from("0xdA816459F1AB5631232FE5e97a05BBBb94970c95")),
                str_to_address(String::from("0x6B175474E89094C44Da98b954EedeAC495271d0F")),
            );

            // USDC
            yearn_tokens.insert(
                str_to_address(String::from("0xa354f35829ae975e850e23e9615b11da1b3dc4de")),
                str_to_address(String::from("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")),
            );
        }

        CreditManager {
            credit_accounts: HashMap::new(),
            added_to_job: HashMap::new(),
            contract,
            address: payload.0,
            underlying_token: payload.2,
            is_weth: payload.3,
            // can_borrow: payload.4,
            // borrow_rate: payload.5,
            // min_amount: payload.6,
            // max_amount: payload.7,
            // max_leverage_factor: payload.8,
            // available_liquidity: payload.9,
            allowed_tokens: payload.10.clone(),
            // adapters: payload.11.clone(),
            data_compressor,
            pool_service,
            credit_filter,
            yearn_tokens,
            ampq_service,
        }
    }

    pub async fn update(
        &mut self,
        from_block: &U64,
        to_block: &U64,
        price_oracle: &PriceOracle<M, S>,
        path_finder: &PathFinder<SignerMiddleware<M, S>>,
        jobs: &mut Vec<TerminatorJob>,
    ) -> Result<(), LiquidationError> {
        self.credit_filter.update(from_block, to_block).await;
        self.update_accounts(from_block, to_block).await;
        let new_ci = self.pool_service.get_new_ci().await;

        println!("Credit manager: {:?}", &self.address);

        let mut accs_to_liquidate: HashSet<Address> = HashSet::new();
        for ca in self.credit_accounts.iter_mut() {
            let hf = ca.1.compute_hf(
                self.underlying_token,
                &new_ci,
                price_oracle,
                &self.credit_filter,
            )?;

            println!("{:?} : {:?}", &ca.1.borrower, &hf);

            if hf < 10000 {
                if self.added_to_job.contains_key(&ca.1.borrower) {
                    let bad_debt_blocks = self.added_to_job[&ca.1.borrower] + 1;
                    *self.added_to_job.get_mut(&ca.1.borrower).unwrap() = bad_debt_blocks;

                    if bad_debt_blocks > 5 && bad_debt_blocks % 50 == 0{
                        self.ampq_service
                            .send(format!(
                                "BAD DEBT!: Credit manager: {:}\nborrower: {:?}",
                                &self.address, &ca.1.borrower
                            ))
                            .await;
                    }
                } else {
                    self.added_to_job.insert(*&ca.1.borrower, 0u8);
                    accs_to_liquidate.insert(ca.1.borrower);
                }
            }
        }

        dbg!(&accs_to_liquidate);

        println!("Starting liquidation process:");

        for acc in accs_to_liquidate {
            jobs.push(self.liquidate(&acc, path_finder).await?);
        }
        Ok(())
    }

    async fn liquidate(
        &mut self,
        address: &Address,
        path_finder: &PathFinder<SignerMiddleware<M, S>>,
    ) -> Result<TerminatorJob, LiquidationError> {
        let account = &self.credit_accounts[&address];
        println!("Preparing to liquidate {:?}", &address);

        let mut paths: Vec<(
            ethers_core::types::U256,
            Vec<ethers_core::types::Address>,
            ethers_core::types::U256,
        )> = Vec::new();

        let mut balances = account.balances.clone();

        for y_token in self.yearn_tokens.iter() {
            *balances.get_mut(&y_token.1).unwrap() = balances[&y_token.0] + balances[&y_token.1];
            *balances.get_mut(&y_token.0).unwrap() = U256::from(0);
        }

        for asset in self.allowed_tokens.iter() {
            let trade_path = path_finder
                .get_best_rate(*asset, self.underlying_token, balances[&asset])
                .await?;
            paths.push((balances[&asset], trade_path.path, trade_path.amount_out_min));
        }

        dbg!(&account);
        dbg!(&paths);

        let repay_amount = self
            .contract
            .calc_repay_amount(*address, true)
            .call()
            .await
            .map_err(|e| NetError("cant get repay amount".to_string()))?;

        Ok(TerminatorJob {
            credit_manager: self.address,
            borrower: *address,
            router: path_finder.router,
            paths,
            underlying_token: self.underlying_token,
            repay_amount,
        })
    }

    #[async_recursion]
    async fn load_events(
        &mut self,
        from_block: &U64,
        to_block: &U64,
    ) -> Vec<(CreditManagerEvents, LogMeta)> {
        let events = self
            .contract
            .events()
            .from_block(from_block)
            .to_block(to_block)
            .query_with_meta()
            .await;

        match events {
            Ok(result) => result,
            Err(err) => {
                println!("Query err: {:?}", err);

                let mid_block = (from_block + to_block) / 2u64;
                if mid_block == *from_block || mid_block == *to_block {
                    panic!("range is already narrow");
                }

                let mut left_part = self.load_events(from_block, &mid_block).await;
                let mut right_part = self.load_events(&(mid_block + 1u64), to_block).await;
                left_part.append(&mut right_part);
                left_part
            }
        }
    }

    async fn update_accounts(&mut self, from_block: &U64, to_block: &U64) {
        let mut updated: HashSet<Address> = HashSet::new();

        let events = self.load_events(from_block, to_block).await;

        let mut counter: u64 = 0;
        let mut oper_by_user: HashMap<Address, u64> = HashMap::new();

        let selected = str_to_address("0xEB2902acd8021Fb93b92a9CFaa5F3cf3758b4318".to_string());

        println!("Credit account: {}", self.address);

        for event in events {
            match &event.0 {
                CreditManagerEvents::OpenCreditAccountFilter(data) => {
                    // println!("OPEN, {:?}  {:?} ", &event.0, data);
                    if data.on_behalf_of == selected || data.sender == selected {
                        println!("[{}]: OPEN: {:?}", &event.1.block_number, data);
                    }
                    updated.insert(data.on_behalf_of);
                }
                CreditManagerEvents::CloseCreditAccountFilter(data) => {
                    // println!("Close credit account, {:?} ", &event.0);
                    if data.owner == selected {
                        println!("[{}]: CLOSE: {:?} ", &event.1.block_number, data);
                    }

                    self.added_to_job.remove(&data.owner);
                    self.credit_accounts.remove(&data.owner);
                    updated.remove(&data.owner);
                }
                CreditManagerEvents::RepayCreditAccountFilter(data) => {
                    if data.owner == selected {
                        println!("[{}]: REPAY: {:?} ", &event.1.block_number, data);
                    }
                    self.added_to_job.remove(&data.owner);
                    self.credit_accounts.remove(&data.owner);
                    updated.remove(&data.owner);
                }
                CreditManagerEvents::LiquidateCreditAccountFilter(data) => {
                    if data.owner == selected {
                        println!("[{}]: LIQUIDATE: {:?} ", &event.1.block_number, data);
                    }
                    self.added_to_job.remove(&data.owner);
                    self.credit_accounts.remove(&data.owner);
                    updated.remove(&data.owner);
                }
                CreditManagerEvents::IncreaseBorrowedAmountFilter(data) => {
                    // println!("Incresae borrowing, {:?} ", &event.0);
                    if data.borrower == selected {
                        println!(
                            "[{}]: INCREASE BORROWING: {:?}",
                            &event.1.block_number, data
                        );
                    }
                    updated.insert(data.borrower);
                }
                CreditManagerEvents::AddCollateralFilter(data) => {
                    if data.on_behalf_of == selected {
                        println!("[{}]: ADD COLLATERAL:  {:?} ", &event.1.block_number, data);
                    }
                    self.added_to_job.remove(&data.owner);
                    updated.insert(data.on_behalf_of);
                }
                CreditManagerEvents::TransferAccountFilter(data) => {
                    // println!("Transfer, {:?} ", &event.0);

                    if data.new_owner == selected {
                        println!("[{}]: TRANSFER, {:?}", &event.1.block_number, data);
                    }
                    self.credit_accounts.remove(&data.old_owner);
                    self.added_to_job.remove(&data.old_owner);
                    updated.remove(&data.old_owner);
                    updated.insert(data.new_owner);
                }
                CreditManagerEvents::NewParametersFilter(data) => {
                    println!("New params, {:?} ", &event.0)
                }
                CreditManagerEvents::ExecuteOrderFilter(data) => {
                    if data.borrower == selected {
                        println!("[{}]: EXECUTE, {:?} ", &event.1.block_number, data);
                    }

                    counter = counter + 1;
                    if oper_by_user.contains_key(&data.borrower) {
                        *oper_by_user.get_mut(&data.borrower).unwrap() =
                            oper_by_user[&data.borrower] + 1;
                    } else {
                        oper_by_user.insert(data.borrower, 1);
                    }
                }

                _ => {}
            }
        }
        // println!("Got operations: {}", &counter);
        // println!("Got operations: {:?}", &oper_by_user.keys().len());
        println!("\n\nUnderlying token: {:?}", &self.underlying_token);
        println!("\n\nCredit manager address: {:?}", &self.address);
        println!("Credit acc data is loaded");

        let function = &self
            .data_compressor
            .abi()
            .functions
            .get("getCreditAccountDataExtended")
            .unwrap()[0];

        dbg!(&updated);

        // let tx =self.data_compressor
        //     .get_credit_account_data_extended(self.address, *updated.iter().next().unwrap())
        //     .tx.clone();
        //
        // let jobs = stream::iter(updated.clone().iter()).map(|b| {
        //     async move {
        //
        //
        //
        //         self
        //             .data_compressor
        //             .client()
        //             .call(&tx, BlockId::from(to_block.as_u64()).into())
        //             .await
        //             .unwrap()
        //     }
        // }).buffer_unordered(3);
        //
        // jobs.for_each(|f| async {
        //     dbg!(f);
        // }).await;

        for borrower in updated.clone().iter() {
            print!(". {}", borrower);
            // let payload =
            //     self
            //     .data_compressor
            //     .get_credit_account_data_extended(self.address, *borrower)
            //     .call()
            //     .await
            //     .unwrap();

            let tx = self
                .data_compressor
                .get_credit_account_data_extended(self.address, *borrower)
                .tx;

            let response = self
                .data_compressor
                .client()
                .call(&tx, BlockId::from(to_block.as_u64()).into())
                .await
                .unwrap();

            let payload: (
                ethers_core::types::Address,
                ethers_core::types::Address,
                bool,
                ethers_core::types::Address,
                ethers_core::types::Address,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
                Vec<(ethers_core::types::Address, ethers_core::types::U256, bool)>,
                ethers_core::types::U256,
                ethers_core::types::U256,
                bool,
                ethers_core::types::U256,
                ethers_core::types::U256,
                ethers_core::types::U256,
            ) = decode_function_data(function, response, false).unwrap();

            let health_factor = payload.7.as_u64();

            let ca = CreditAccount {
                contract: payload.0,
                borrower: *borrower,
                borrowed_amount: payload.13,
                cumulative_index_at_open: payload.14,
                balances: HashMap::from_iter(payload.9.into_iter().map(|elm| (elm.0, elm.1))),
                health_factor,
            };

            // if health_factor > 100_000 {
            //     dbg!(&ca);
            // }

            if self.credit_accounts.contains_key(&borrower) {
                // dbg!(data.unwrap().0);
                *self.credit_accounts.get_mut(&borrower).unwrap() = ca;
            } else {
                self.credit_accounts.insert(*borrower, ca);
            }
        }

        println!("\nTotal accs: {}", &self.credit_accounts.len());

        println!("Credit acc data is updated");
    }

    pub fn print_accounts(&self) {
        for acc in self.credit_accounts.values() {
            dbg!(acc);
        }
    }
}

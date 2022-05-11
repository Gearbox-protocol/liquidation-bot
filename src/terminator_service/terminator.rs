use std::process::exit;
use crate::bindings::flash_loan_terminator::FlashLoanTerminator;
use crate::bindings::iterminator::ITerminator;
use crate::errors::LiquidationError;
use crate::errors::LiquidationError::NetError;
use crate::path_finder::service::TradePath;
use ethers::abi::ethereum_types::H256;
use ethers::abi::Address;
use ethers::prelude::{Middleware, Signer, SignerMiddleware, TransactionReceipt, U256};

#[derive(Debug)]
pub struct TerminatorJob {
    pub(crate) credit_manager: Address,
    pub(crate) borrower: Address,
    pub(crate) router: Address,
    pub(crate) paths: Vec<(
        ethers_core::types::U256,
        Vec<ethers_core::types::Address>,
        ethers_core::types::U256,
    )>,
    pub(crate) yearn_tokens: Vec<Address>,
    pub repay_amount: U256,
    pub underlying_token: Address,
}

pub struct TerminatorService<M: Middleware, S: Signer> {
    terminator: ITerminator<SignerMiddleware<M, S>>,
    terminator_flash: FlashLoanTerminator<SignerMiddleware<M, S>>,
    client: std::sync::Arc<SignerMiddleware<M, S>>,
}

impl<M: Middleware, S: Signer> TerminatorService<M, S> {
    pub async fn new(
        terminator_address: &Address,
        terminator_flash_address: &Address,
        client: std::sync::Arc<SignerMiddleware<M, S>>,
        liquidator_enabled: bool,
    ) -> Self {
        let terminator = ITerminator::new(*terminator_address, client.clone());
        let terminator_flash = FlashLoanTerminator::new(*terminator_flash_address, client.clone());

        // if liquidator_enabled {
        //     enable_executors(&terminator, client.address()).await;
        //     enable_executors(&terminator_flash, client.address()).await;
        // }

        TerminatorService {
            terminator,
            terminator_flash,
            client: client.clone(),
        }
    }

    pub async fn liquidate(
        &mut self,
        job: &TerminatorJob,
        terminator_type: i32,
    ) -> Result<TransactionReceipt, LiquidationError> {
        dbg!(&job);
        println!("Length: {}", &job.paths.len());

        // let terminator = if terminator_type == 1 {
        //     &self.terminator
        // } else {
        //     &self.terminator_flash
        // };

        let price = self.client.get_gas_price().await.map_err(|err| {
            dbg!(err);
            NetError(format!("Cant execute liquidation {:?}", &job).into())
        })?;

        println!("Current price: {}", price);

        let result = self
            .terminator_flash
            .liquidate(
                job.credit_manager,
                job.borrower,
                job.paths.clone(),
                job.yearn_tokens.clone(),
            )
            .gas(3_500_000)
            .gas_price(price * 125 / 100)
            .send()
            .await
            .map_err(|err| {
                dbg!(err);
                dbg!(&job);
                NetError(format!("Contract Error: Cant execute liquidation {:?}", &job).into())

            })?
            .await
            .map_err(|err| {
                dbg!(err);
                NetError(format!("Provider error: Cant execute liquidation {:?}", &job).into())
            })?
            .ok_or(NetError(
                format!("Cant execute liquidation {:?}", &job).into(),
            ))?;
        Ok(result)
    }

    // async fn enable_executors(terminator: &ITerminator<SignerMiddleware<M,S>>, address: Address) {
    //     let is_executor = terminator.executors(client.address()).call().await.unwrap();
    //
    //     if !is_executor {
    //         let tx = terminator
    //             .allow_executor(address.clone())
    //             .send()
    //             .await
    //             .unwrap()
    //             .await
    //             .unwrap()
    //             .unwrap();
    //
    //         println!("Allow executor {}", tx.transaction_hash);
    //     } else {
    //         println!("Executor {} is already allowed", &address)
    //     }
    // }
}

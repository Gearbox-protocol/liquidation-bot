//
extern crate hex;

use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
//
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::sync::Arc;

use anyhow::Result;
use ethers::prelude::*;
use ethers_core::*;

//
use crate::bindings::address_provider::AddressProvider;
use crate::config::Config;
use crate::credit_service::service::CreditService;
use crate::credit_service::CreditManager;
use crate::price_oracle::oracle::PriceOracle;
use crate::token_service::service::TokenService;

//
mod ampq_service;
mod bindings;
mod config;
mod credit_service;
mod errors;
mod path_finder;
mod price_oracle;
mod terminator_service;
mod token_service;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Gearbox liquidation bot started!");

    let config = Config::default();
    println!("Address provider: {:?} ", &config.address_provider);

    dbg!(&config);

    let provider = Provider::<Http>::try_from(config.eth_provider_rpc.clone())?;

    // create a wallet and connect it to the provider
    let wallet = config.private_key.parse::<LocalWallet>()?;
    let w2 = wallet.with_chain_id(config.chain_id);

    println!("Signer address: {:?}", &w2.address());

    let client: ethers::prelude::SignerMiddleware<
        ethers::prelude::Provider<ethers::prelude::Http>,
        ethers::prelude::Wallet<ethers_core::k256::ecdsa::SigningKey>,
    > = SignerMiddleware::new(provider.clone(), w2);

    let client = Arc::new(client);

    let address_provider = AddressProvider::new(config.address_provider, client.clone());

    let data_compressor_addr = config.data_compressor.clone();

    let token_service = TokenService::new(client.clone());

    let price_oracle = PriceOracle::new(client.clone(), config.price_oracle.clone());

    let mut credit_service = CreditService::new(
        &config,
        data_compressor_addr,
        client.clone(),
        token_service,
        price_oracle,
        provider,
    )
    .await;

    credit_service.launch().await;

    Ok(())
}

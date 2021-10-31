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
use crate::credit_service::CreditManager;
use crate::credit_service::service::CreditService;
use crate::price_oracle::oracle::PriceOracle;
use crate::token_service::service::TokenService;

//
mod ampq_service;
mod bindings;
mod config;
mod credit_service;
mod path_finder;
mod price_oracle;
mod terminator_service;
mod token_service;


#[tokio::main]
async fn main() -> Result<()> {
    println!("Gearbox liquidation node!");

    let config = Config::default();
    println!("Address provider: {:?} ", &config.address_provider);

    dbg!(&config);

    let provider = Provider::<Http>::try_from(config.eth_provider_rpc.clone())?;

    // create a wallet and connect it to the provider
    let wallet = config.private_key.parse::<LocalWallet>()?;
    let kovan: u64 = 42;
    let w2 = wallet.with_chain_id(kovan);
    let client: ethers::prelude::SignerMiddleware<
        ethers::prelude::Provider<ethers::prelude::Http>,
        ethers::prelude::Wallet<ethers_core::k256::ecdsa::SigningKey>,
    > = SignerMiddleware::new(provider.clone(), w2);



    let client = Arc::new(client);

    let address_provider = AddressProvider::new(config.address_provider, client.clone());

    let data_compressor_addr = address_provider.get_data_compressor().call().await.unwrap();

    let token_service = TokenService::new(client.clone());

    let price_oracle = PriceOracle::new(
        client.clone(),
        address_provider.get_price_oracle().call().await.unwrap(),
    );

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

    // let mut t = token_service.borrow_mut();
    // t.add_token(tokens).await;
    //
    // drop(t);
    //
    // price_oracle.load_price_feeds(&tokens).await;
    // price_oracle.update_prices().await;
    // price_oracle.print_prices();
    //
    // credit_service.get_events().await;
    // credit_service.ts();
    // credit_service.ts();

    Ok(())
}

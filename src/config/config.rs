use anyhow::Result;
extern crate dotenv;
use ethers::prelude::Address;
use std::fmt::Debug;
use std::{env, fmt};

#[derive(Debug)]
pub struct Config {
    pub private_key: String,
    pub eth_provider_rpc: String,
    pub address_provider: Address,
    pub path_finder: Address,
    pub bot_address: Address,
    pub ampq_addr: String,
}

impl Default for Config {
    fn default() -> Self {
        dotenv::from_filename(".env").ok();
        dotenv::from_filename(".env.local").ok();
        let address_provider =
            str_to_address(env::var("ADDRESS_PROVIDER").expect("No ADDRESS_PROVIDER"));
        let eth_provider_rpc = env::var("ETH_PROVIDER").expect("No ETH_PROVIDER");
        let private_key = env::var("PRIVATE_KEY").expect("No PRIVATE_KEY");
        let path_finder = str_to_address(env::var("PATH_FINDER").expect("No PATH_FINDER"));
        let ampq_addr = env::var("CLOUDAMQP_URL").expect("No CLOUDAMQP_URL");
        let bot_address = str_to_address(env::var("BOT_ADDRESS").expect("No BOT_ADDRESS"));
        Config {
            address_provider,
            private_key,
            eth_provider_rpc,
            path_finder,
            ampq_addr,
            bot_address,
        }
    }
}

pub fn str_to_address(address: String) -> Address {
    let addr = hex::decode(address.as_str().strip_prefix("0x").unwrap())
        .expect(format!("Decoding of {} address failed", address).as_str());
    Address::from_slice(addr.as_slice())
}

use anyhow::Result;
extern crate dotenv;
use ethers::prelude::Address;
use std::fmt::Debug;
use std::{env, fmt};

#[derive(Debug)]
pub struct Config {
    pub chain_id: u64,
    pub chain_id_name: String,
    pub private_key: String,
    pub eth_provider_rpc: String,
    pub address_provider: Address,
    pub path_finder: Address,
    pub bot_address: Address,
    pub ampq_addr: String,
    pub etherscan: String,
    pub charts_url: String,
    pub liquidator_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        dotenv::from_filename(".env").ok();
        dotenv::from_filename(".env.local").ok();
        let chain_id = get_env_or_throw("REACT_APP_CHAIN_ID")
            .parse::<u64>()
            .expect("REACT_APP_CHAIN_ID is not number");
        let address_provider = str_to_address(get_env_or_throw("REACT_APP_ADDRESS_PROVIDER"));

        let private_key = get_env_or_throw("PRIVATE_KEY");
        let path_finder = str_to_address(get_env_or_throw("REACT_APP_PATHFINDER"));
        let ampq_addr = env::var("CLOUDAMQP_URL").unwrap_or("".into());
        let bot_address = str_to_address(get_env_or_throw("BOT_ADDRESS"));

        let (chain_id_name, eth_provider_rpc, etherscan, charts_url) = match chain_id {
            1 => (
                "MAINNET",
                get_env_or_throw("ETH_MAINNET_PROVIDER"),
                "https://etherscan.io",
                "https://charts.gearbox.fi/",
            ),
            42 => (
                "KOVAN",
                get_env_or_throw("ETH_KOVAN_PROVIDER"),
                "https://kovan.etherscan.io",
                "https://charts.kovan.gearbox.fi/",
            ),
            1337 => (
                "FORK",
                get_env_or_throw("ETH_FORK_PROVIDER"),
                "https://etherscan.io",
                "http://localhost:3002/",
            ),

            _ => {
                panic!("Unknown network!")
            }
        };

        let liquidator_enabled = if env::var("LIQUIDATOR_ENABLED").unwrap_or("".into()) == "true" {
            true
        } else {
            false
        };

        Config {
            chain_id,
            chain_id_name: chain_id_name.into(),
            address_provider,
            private_key,
            eth_provider_rpc,
            path_finder,
            ampq_addr,
            bot_address,
            etherscan: etherscan.into(),
            liquidator_enabled,
            charts_url: charts_url.into(),
        }
    }
}

pub fn str_to_address(address: String) -> Address {
    let addr = hex::decode(address.as_str().strip_prefix("0x").unwrap())
        .expect(format!("Decoding of {} address failed", address).as_str());
    Address::from_slice(addr.as_slice())
}

fn get_env_or_throw(env: &str) -> String {
    env::var(env).expect(format!("No {}", env).as_str())
}

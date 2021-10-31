use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::iter::FromIterator;
use std::ops::Deref;

use ethers::abi::ethereum_types::{Address, U256};
use ethers::prelude::Middleware;

use crate::bindings::ERC20;

#[derive(Clone)]
pub struct Token<M: Middleware> {
    pub symbol: String,
    pub decimals: u8,
    pub contract: ERC20<M>,
}

impl<M: Middleware> Token<M> {
    pub async fn balance_of(&self, address: &Address) -> U256 {
        self.contract.balance_of(*address).call().await.unwrap()
    }

    pub fn format_bn(&self, amount: &U256) -> String {
        let amount = amount / U256::from(10).pow(U256::from(self.decimals - 4));
        let amount: f64 = amount.as_u64() as f64;
        format!("{}", amount / 10000f64)
    }
}

impl<M: Middleware> Debug for Token<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Symbol: {:}, decimals: {:}",
            self.symbol.as_str(),
            self.decimals
        )
    }
}

impl<M: Middleware> Display for Token<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Symbol: {:}, decimals: {:}",
            self.symbol.as_str(),
            self.decimals
        )
    }
}

#[derive(Clone)]
pub struct TokenService<M: Middleware> {
    tokens: HashMap<Address, Token<M>>,
    client: std::sync::Arc<M>,
}

impl<M: Middleware> TokenService<M> {
    pub fn new(client: std::sync::Arc<M>) -> Self {
        TokenService {
            client,
            tokens: HashMap::new(),
        }
    }

    pub async fn add_token(&mut self, addresses: &HashSet<Address>) {
        for address in addresses {
            if !self.tokens.contains_key(&address) {
                let token_contract: ERC20<M> = ERC20::new(*address, self.client.clone());
                let symbol = token_contract.symbol().call().await.unwrap();
                let decimals = token_contract.decimals().call().await.unwrap();
                self.tokens.insert(
                    *address,
                    Token {
                        symbol,
                        decimals,
                        contract: token_contract,
                    },
                );
            }
        }
    }

    // pub fn get_token_addresses(&self) -> Vec<&Address> {
    //     Vec::from_iter(self.tokens.keys())
    // }

    pub async fn get_balance(&self, token: &Address, address: &Address) -> U256 {
        self.tokens
            .get(&token)
            .expect(format!("Unknown token {}", &token).as_str())
            .balance_of(address)
            .await
    }

    pub fn format_bn(&self, token: &Address, amount: &U256) -> String {
        self.tokens
            .get(&token)
            .expect(format!("Unknown token {}", &token).as_str())
            .format_bn(amount)
    }

    pub fn symbol(&self, token: &Address) -> &String {
        &self
            .tokens
            .get(&token)
            .expect(format!("Unknown token {}", &token).as_str())
            .symbol
    }
}

impl<M: Middleware> Debug for TokenService<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.tokens.values())
    }
}

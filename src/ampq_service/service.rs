use crate::config::Config;
use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties};
use tokio_amqp::*;

#[derive(Clone)]
pub struct AmpqService {
    channel: Option<Channel>,
    network_name: String,
    router_key: String
}

impl AmpqService {
    pub async fn new(config: &Config) -> Self {
        let channel = if config.ampq_addr == String::from("") {
            None
        } else {
            let conn = Connection::connect(
                &config.ampq_addr,
                ConnectionProperties::default().with_tokio(),
            )
            .await
            .unwrap(); // Note the `with_tokio()` here
            Some(
                conn.create_channel()
                    .await
                    .expect("Cant connect to channel"),
            )
        };

        let router_key = config.ampq_router_key.clone();

        AmpqService {
            channel,
            network_name: config.chain_id_name.clone(),
            router_key
        }
    }

    pub async fn send(&self, msg: String) {
        let message = format!("[{}]: Liquidation bot:\n {}", &self.network_name, msg);

        match &self.channel {
            Some(ch) => {
                ch.basic_publish(
                    "TelegramBot",
                    &self.router_key,
                    BasicPublishOptions::default(),
                    message.into(),
                    BasicProperties::default(),
                )
                .await
                .unwrap()
                .await
                .unwrap();
            }
            _ => {
                println!("{}", message);
            }
        }
    }
}

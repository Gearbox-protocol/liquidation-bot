use crate::config::Config;
use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties};
use tokio_amqp::*;

pub struct AmpqService {
    channel: Option<Channel>,
    network_name: String,
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

        AmpqService {
            channel,
            network_name: config.chain_id_name.clone(),
        }
    }

    pub async fn send(&self, msg: String) {
        let message = format!("[{}]: Liquidation bot:\n {}", &self.network_name, msg);

        match &self.channel {
            Some(ch) => {
                ch.basic_publish(
                    "TelegramBot",
                    "",
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

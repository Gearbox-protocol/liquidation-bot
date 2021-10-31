use crate::config::Config;
use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties};
use tokio_amqp::*;

pub struct AmpqService {
    channel: Channel,
}

impl AmpqService {
    pub async fn new(config: &Config) -> Self {
        let conn = Connection::connect(
            &config.ampq_addr,
            ConnectionProperties::default().with_tokio(),
        )
        .await
        .unwrap(); // Note the `with_tokio()` here
        let channel = conn
            .create_channel()
            .await
            .expect("Cant connect to channel");

        AmpqService { channel }
    }

    pub async fn send(&self, msg: String) {
        self.channel
            .basic_publish(
                "TelegramBot",
                "",
                BasicPublishOptions::default(),
                format!("Liquidation bot:\n {}", msg).into(),
                BasicProperties::default(),
            )
            .await
            .unwrap()
            .await
            .unwrap();
    }
}

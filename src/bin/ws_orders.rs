use log::info;

use std::str::FromStr;

use ethers::types::H160;
use hyperliquid_rust_sdk_custom::{BaseUrl, InfoClient, Message, Subscription};
use tokio::{
    spawn,
    sync::mpsc::unbounded_channel,
    time::{sleep, Duration},
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut info_client = InfoClient::new(None, Some(BaseUrl::Testnet)).await.unwrap();
    let user = H160::from_str("0xc64cc00b46101bd40aa1c3121195e85c0b0918d8").unwrap();

    let (sender, mut receiver) = unbounded_channel();
    let subscription_id = info_client
        .subscribe(Subscription::OrderUpdates { user }, sender)
        .await
        .unwrap();

    spawn(async move {
        sleep(Duration::from_secs(30)).await;
        info!("Unsubscribing from order updates data");
        info_client.unsubscribe(subscription_id).await.unwrap()
    });

    // this loop ends when we unsubscribe
    while let Some(Message::OrderUpdates(order_updates)) = receiver.recv().await {
        info!("Received order update data: {order_updates:?}");
    }
}

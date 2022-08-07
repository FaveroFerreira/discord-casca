use async_tungstenite::tungstenite::Message;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use tokio::time::timeout;

use discord_bot::new;
use discord_bot::gateway::event;
use discord_bot::gateway::event::{GatewayEvent, Hello, Operation};

#[tokio::main]
async fn main() {
    let mut gateway_con = new().await;
    // let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    // tokio::spawn(async move {
    //     loop {
    //         tokio::time::sleep(Duration::from_secs(3)).await;
    //         match &mut rx.try_recv() {
    //             Ok(val) => println!("recv val: {val}"),
    //             Err(e) => eprintln!("failed recv {e:?}")
    //         }
    //     }
    // });

    loop {
        tokio::time::sleep(Duration::from_secs(3)).await;
        match timeout(Duration::from_millis(500), &mut gateway_con.next()).await {
            Ok(Some(Ok(incomming_message))) => {
                println!("identity = {:?}", Operation::Identify);

                println!("discord_msg = {incomming_message:?}");

                let msg = deserialize(incomming_message);

                match msg.op {
                    Operation::Hello => {
                        let data: Hello = serde_json::from_value(msg.d.unwrap()).unwrap();

                        println!("hello = {data:?}");
                    }
                    Operation::Identify => {}
                    Operation::Heartbeat => {}
                    Operation::HeartbeatAck => {}
                    Operation::Resume => {}
                    Operation::Resumed => {}
                    Operation::SelectProtocol => {}
                }

                ()
            }
            Ok(Some(Err(e))) => {
                println!("{e:?}")
            }
            Ok(None) => {
                println!("none")
            }
            Err(e) => {
                println!("errouu = {e:?}")
            }
        }
    }
}

pub fn deserialize(message: Message) -> GatewayEvent {
    match message {
        Message::Text(content) => serde_json::from_str(&content).unwrap(),
        _ => panic!("failed")
    }
}

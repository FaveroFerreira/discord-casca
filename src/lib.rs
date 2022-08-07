use async_tungstenite::tokio::ConnectStream;
use async_tungstenite::WebSocketStream;

pub const DISCORD_URL: &str = "wss://gateway.discord.gg/";
pub const BOT_TOKEN: &str = "";

pub mod gateway;

pub async fn new() -> WebSocketStream<ConnectStream> {
    let url = format!("{DISCORD_URL}?v=10&encoding=json");

    let (ws, response) = async_tungstenite::tokio::connect_async(url).await.unwrap();

    ws
}

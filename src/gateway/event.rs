use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayEvent {
    pub op: Operation,
    pub d: Option<Value>,
    pub s: Option<u8>,
    pub t: Option<String>,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Operation {
    Hello = 10,
    Identify = 2,
    Heartbeat = 1,
    HeartbeatAck = 11,
    Resume,
    Resumed,
    SelectProtocol,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hello {
    pub heartbeat_interval: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Identify {
    pub token: String,
    pub intents: u32,
    pub properties: IdentifyProperties
}

#[derive(Serialize, Deserialize)]
pub struct IdentifyProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}
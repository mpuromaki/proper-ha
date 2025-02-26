use serde::{Deserialize, Serialize};

/// Top-level frame for all Proper Home Automation messages.
/// This frame is serializable to either MessagePack or JSON.
#[derive(Serialize, Deserialize)]
pub struct ProperFrame {
    pub src: PrprNodeUid,   // Source node
    pub dst: PrprNodeUid,   // Destination node
    pub ver: PrprVersion,   // Protocol version
    pub mid: u64,           // Random message identifier
    pub msg: ProperMessage, // Message payload
}

/// Protocol version tuple.
/// Major and minor version numbers.
pub type PrprVersion = (u8, u8);

/// Unique identifier for a Proper Home Automation node.
/// 0 is reserved for Proper Automation Servers.
/// Value should be random for Nodes.
pub type PrprNodeUid = u128;

/// Signal identifier for a Proper Home Automation signal.
/// Defined by the node. Node specific.
/// Serialized as ID for MessagePack.
/// Serialized as Name for JSON.
#[derive(Serialize, Deserialize)]
pub enum PrprSignalId {
    Id(u8),       // 0-255
    Name(String), // Not allowed to start with a number
}

#[repr(u8)]
#[derive(Serialize, Deserialize)]
pub enum ProperMessage {
    NodePush(PrprNodePush) = 10,
}

/// Message for Node to push measurement values to Server.
/// Contains a list of timestamped signals with values.
#[derive(Serialize, Deserialize)]
pub struct PrprNodePush {
    pub data: Vec<PrprNodeSignal>,
}

#[derive(Serialize, Deserialize)]
pub struct PrprNodeSignal {
    pub id: PrprSignalId,
    pub ts: PrprTimestamp,
    pub sig: PrprSignal,
}

/// Timestamp in milliseconds since Unix epoch, TAI.
/// Does not account for leap seconds. Always incrementing.
pub type PrprTimestamp = u64;

/// Signal type and value container for Proper measurements.
/// Defined here, standard for all nodes.
#[repr(u8)]
#[derive(Serialize, Deserialize)]
#[serde(tag = "typ", content = "val")]
pub enum PrprSignal {
    Temperature(f32) = 1,      // Celcius, °C
    Humidity(f32) = 2,         // Humidity, relative, %rh
    Pressure(f32) = 3,         // Pascal, absolute, Pa
    Light(f32) = 4,            // Lux, lx
    Motion(bool) = 5,          // 1 = detected, 0 = not detected
    OnOff(bool) = 6,           // 1 = on, 0 = off
    State(u8) = 253,           // State, unspecified
    Text(u16, Vec<u8>) = 254,  // Length in Bytes, Text as UTF-8
    Bytes(u16, Vec<u8>) = 255, // Length in Bytes, Raw bytes
}

//! Proper Home Automation messages standard.
//! This module defines the standard messages for Proper Home Automation.
//! The messages are serializable to either MessagePack or JSON.
//! For JSON the fields are serialized as strings.
//! For MessagePack the fields are serialized as integers, where defined.
//! The messages are always encapsulated in a ProperFrame.
//!
//! Ack field in ProperFrame allows client to acknowledge messages asynchronously,
//! when Server requires an acknowledgement.
//! Server responds to Node messages with AckStatus message as needed.
//!
//! The underlying idea is that Nodes drive the communication and Servers respond.
//! Servers need to send commands to Nodes, which is handled with outbox and Poll.
//! To delete a message from outbox, Node needs to acknowledge the message.
//! If not acknowledged, the message will be retransmit on next Poll.

use serde::{Deserialize, Serialize};

/// Top-level frame for all Proper Home Automation messages.
/// This frame is serializable to either MessagePack or JSON.
#[derive(Serialize, Deserialize)]
pub struct ProperFrame {
    pub src: PrprNodeUid,   // Source node
    pub dst: PrprNodeUid,   // Destination node
    pub ver: PrprVersion,   // Protocol version
    pub mid: PrprMessageId, // Random message identifier
    pub pnd: bool,          // Sender has one or more pending messages in outbox
    pub ack: Vec<u64>,      // Acknowledged message identifiers, to allow asynchronous simple acks
    pub msg: ProperMessage, // Message payload
}

#[repr(u8)]
#[derive(Serialize, Deserialize)]
pub enum ProperMessage {
    AckStatus(AckStatus) = 1,                 // Server -> Node
    RegisterAllowed(PrprRegisterAllowed) = 2, // Server -> Node
    RegisterDenied(PrprRegisterDenied) = 3,   // Server -> Node
    RequestDetails(PrprRequestDetails) = 4,   // Server -> Node
    //ServerPush(PrprServerPush) = 5,         // Server -> Node
    Register(PrprNodeRegister) = 100, // Node -> Server
    Details(PrprDetails) = 101,       // Node -> Server
    NodePush(PrprNodePush) = 110,     // Node -> Server
    Poll(PrprPoll) = 111,             // Node -> Server
}

/// Message for acknowledgement of previous message, with status.
/// Transport layer status codes should follow message contents.
/// - HTTP: Good -> 2xx, Bad or Uncertain -> 4xx, System error -> 5xx
/// - CoAP: Good -> 2.xx, Bad or Uncertain -> 4.xx, System error -> 5.xx
/// Acknowledge message never requires an response.
#[derive(Serialize, Deserialize)]
pub struct AckStatus {
    pub rmid: PrprMessageId, // Message identifier to acknowledge
    pub code: PrprStatus,    // Status Code
}

/// Message for Node to register with Server.
/// This is the first message from a new or factory reset node.
/// Contains basic information for user to identify the node.
/// Always encrypted with Proper Master Secret.
/// Node = Proper specific information, Device = physical device information
/// Company = Manufacturer information.
#[derive(Serialize, Deserialize)]
pub struct PrprNodeRegister {
    pub nuid: u128,           // Node unique identifier
    pub ncat: PrprDeviceType, // Node category
    pub nnam: String,         // Node name
    pub dmod: String,         // Device model
    pub dser: String,         // Device serial number
    pub cnam: String,         // Company name
}

/// Message for Server to allow Node registration.
/// Sent as asynchronous response to Node Register, after user acceptance.
/// From this point on, Node has private key to push data to Server.
#[derive(Serialize, Deserialize)]
pub struct PrprRegisterAllowed {
    pub nuid: u128, // Node unique identifier
    pub npsk: u128, // Node pre-shared key
}

/// Message for Server to deny Node registration.
/// Sent as asynchronous response to Node Register, after user rejection.
/// Use can factory reset the Node to try again.
#[derive(Serialize, Deserialize)]
pub struct PrprRegisterDenied {
    pub nuid: u128, // Node unique identifier
}

/// Message for Server to request detailed information from Node.
/// Sent after Node Register Allowed, to get more information.
#[derive(Serialize, Deserialize)]
pub struct PrprRequestDetails {
    pub nuid: u128, // Node unique identifier
}

/// Message for Node to push measurement values to Server.
#[derive(Serialize, Deserialize)]
pub struct PrprNodePush {
    pub data: Vec<PrprNodeValue>,
}

/// Message for Node to request pending message from Server.
/// Server will respond with first-in-line pending message.
/// If no pending messages, Server will respond with an Acknowledge.
/// Node is expected to Acknowledge the received message.
/// Server will remove the message from outbox after Acknowledge.
/// Server will retransmit the message if no Acknowledge is received.
#[derive(Serialize, Deserialize)]
pub struct PrprPoll {}

/// Message for Node to provide detailed information to Server.
/// Sent as a response to RequestDetails.
#[derive(Serialize, Deserialize)]
pub struct PrprDetails {
    pub nuid: u128,                // Node unique identifier
    pub ndev: PrprDeviceType,      // Node device type
    pub nnam: String,              // Node name
    pub dmod: String,              // Device model
    pub dser: String,              // Device serial number
    pub durl: String,              // Device URL
    pub cnam: String,              // Company name
    pub curl: String,              // Company URL
    pub sign: Vec<PrprSignalConf>, // Node signals configuration
}

#[derive(Serialize, Deserialize)]
pub struct PrprSignalConf {
    pub sid: PrprSignalId,    // Node specific ID for signal
    pub snam: String,         // Node specific Name for signal
    pub styp: PrprSignalType, // Signal type and value
    pub smin: String,         // Minimum value, serialized as string
    pub smax: String,         // Maximum value, serialized as string
    pub supd: u32,            // Expected update interval in seconds
}

#[derive(Serialize, Deserialize)]
pub struct PrprNodeValue {
    pub sid: PrprSignalId,  // Node specific ID for signal
    pub sts: PrprTimestamp, // Timestamp
    pub sst: PrprStatus,    // Status Code
    pub sig: PrprSignal,    // Signal type and value
}

/// Protocol version tuple.
/// Major and minor version numbers.
pub type PrprVersion = (u8, u8);

/// Unique identifier for a Proper Home Automation node.
/// 0 is reserved for Proper Automation Servers.
/// Value should be random for Nodes.
pub type PrprNodeUid = u128;

/// Unique identifier for a Proper Home Automation message.
/// Usually random number, unique for each message.
pub type PrprMessageId = u64;

/// Timestamp in milliseconds since Unix epoch, TAI.
/// Does not account for leap seconds. Monotonic.
pub type PrprTimestamp = u64;

/// Status code for a Proper Home Automation signal.
/// https://github.com/OPCFoundation/UA-.NETStandard/blob/master/Stack/Opc.Ua.Core/Schema/Opc.Ua.StatusCodes.csv
/// Top 16 bits from OPC UA status codes. 16#0xxx Good, 16#4xxx Uncertain, 16#8xxx Bad
pub type PrprStatus = u16;

/// Node standard device types
/// JSON Serialize / Deserialize as string.
/// MessagePack Serialize / Deserialize as u8.
/// These device types have predefined signals, for ease of integration.
/// Non-standard devices can use Custom device types.
#[repr(u8)]
#[derive(Serialize, Deserialize)]
pub enum PrprDeviceType {
    // Common sensors
    SensorTemperature = 1, // Temperature sensor
    SensorHumidity = 2,    // Humidity sensor
    SensorPressure = 3,    // Pressure sensor
    SensorLight = 4,       // Light sensor
    SensorMotion = 5,      // Motion sensor
    SensorVibration = 6,   // Vibration sensor
    SensorOccupancy = 7,   // Occupancy sensor
    SensorSmoke = 8,       // Smoke sensor
    // Generic sensors
    SensorOnOff = 50,
    SensorAnalog = 51,
    // Common actuators
    ActuatorRelay = 100,  // Relay actuator
    ActuatorDimmer = 101, // Dimmer actuator
    ActuatorShade = 102,  // Window shade actuator
    ActuatorValve = 103,  // Valve actuator
    ActuatorLock = 104,   // Lock actuator
    ActuatorFan = 105,    // Fan actuator
    ActuatorHeater = 106, // Heater actuator
    ActuatorLight = 107,  // Light actuator
    // Generic actuators
    ActuatorOnOff = 150,
    ActuatorAnalog = 151,
    // Combined sensors and actuators
    // Custom sensors and actuators
    CustomSensor = 253,   // Undefined sensor
    CustomActuator = 254, // Undefined actuator
    CustomCombined = 255, // Undefined combined sensor and actuator
}

/// Signal identifier for a Proper Home Automation signal.
/// Defined by the node. Node specific. Untagged
/// Serialized as ID for MessagePack.
/// Serialized as Name for JSON.
/// Name can not start with a number, so that's how to differentiate.
#[derive(Serialize, Deserialize)]
pub enum PrprSignalId {
    Id(u8),       // 0-255
    Name(String), // Not allowed to start with a number
}

/// Signal standard types.
/// JSON Serialize / Deserialize as string.
/// MessagePack Serialize / Deserialize as u8.
/// NOTE: Must be kept in sync with PrprSignal.
#[repr(u8)]
#[derive(Serialize, Deserialize)]
pub enum PrprSignalType {
    Temperature = 1,
    Humidity = 2,
    Pressure = 3,
    Light = 4,
    Motion = 5,
    OnOff = 6,
    State = 253,
    Text = 254,
    Bytes = 255,
}

/// Signal standard types and units.
/// JSON Serialize / Deserialize type as string.
/// MessagePack Serialize / Deserialize type as u8.
/// NOTE: Must be kept in sync with PrprSignalType.
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

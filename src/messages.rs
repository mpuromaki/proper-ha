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
    Acknowledge(PrprAcknowledge) = 1,
    Register(PrprRegister) = 2,
    NodePush(PrprNodePush) = 10,
}

/// Message for general acknowledgement of received message.
/// https://github.com/OPCFoundation/UA-.NETStandard/blob/master/Stack/Opc.Ua.Core/Schema/Opc.Ua.StatusCodes.csv
/// Transport layer status codes should follow message contents.
/// - HTTP: Good -> 2xx, Bad or Uncertain -> 4xx, System error -> 5xx
/// - CoAP: Good -> 2.xx, Bad or Uncertain -> 4.xx, System error -> 5.xx
#[derive(Serialize, Deserialize)]
pub struct PrprAcknowledge {
    pub mid: u64,  // Message identifier to acknowledge
    pub code: u16, // Status Code, top 16 bits from OPC UA status codes. 16#0xxx Good, 16#4xxx Uncertain, 16#8xxx Bad
}

/// Message for Node to register with Server.
/// This is the first message from a new or factory reset node.
/// Contains basic information for user to identify the node.
/// Node = Proper specific information, Device = physical device information
/// Company = Manufacturer information.
#[derive(Serialize, Deserialize)]
pub struct PrprRegister {
    pub ncat: PrprCategory, // Node category
    pub nuid: u128,         // Node unique identifier
    pub nname: String,      // Node name
    pub dmod: String,       // Device model
    pub dser: String,       // Device serial number
    pub cname: String,      // Company name
}

/// Node standard categories
/// JSON Serialize / Deserialize as string.
/// MessagePack Serialize / Deserialize as u8.
/// These categories have predefined signals, for ease of integration.
/// Non-standard devices can use Custom categories.
#[repr(u8)]
#[derive(Serialize, Deserialize)]
pub enum PrprCategory {
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

/// Signal standard types and units.
/// JSON Serialize / Deserialize type as string.
/// MessagePack Serialize / Deserialize type as u8.
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

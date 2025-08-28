use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub block: String,
    pub base_address: Option<u32>,
    pub fifo: FifoConfig,
    #[serde(default)]
    pub reg_bit_size: RegBitSize,
    pub endpoints: Vec<EndpointConfig>,
    #[serde(default = "Vec::new")]
    pub patches: Vec<Patch>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointConfig {
    #[serde(rename = "type")]
    pub ep_direction: EndpointDirection,
    pub max_packet_size: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EndpointDirection {
    TX,
    RX,
    RXTX,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FifoConfig {
    #[serde(rename = "dynamic")]
    Dynamic(DynamicFifoConfig),
    #[serde(rename = "fixed")]
    Fixed(FixedFifoConfig),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicFifoConfig {
    pub total_size: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FixedFifoConfig {
    pub shared: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegBitSize {
    #[serde(default = "default_8")]
    pub fifo: u8,
    #[serde(default = "default_16")]
    pub intr: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Patch {
    pub fieldset: String,
    pub version: String,
}

impl Default for RegBitSize {
    fn default() -> Self {
        RegBitSize { fifo: 8, intr: 16 }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    // This field specifies the parent block to inherit from.
    // It is deserialized from the source YAML but will be removed before final output.
    // `skip_serializing_if` ensures that if `inherits` is `None`, it's omitted from the output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherits: Option<String>,
    pub description: Option<String>,
    pub items: Vec<BlockItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockItem {
    pub name: String,
    pub description: Option<String>,
    pub byte_offset: Option<String>,
    pub bit_size: Option<String>,
    pub fieldset: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<ArrayConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArrayConfig {
    pub len: Value,
    pub stride: Value,
}

fn default_16() -> u8 {
    16
}

fn default_8() -> u8 {
    8
}

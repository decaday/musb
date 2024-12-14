use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub block: String,
    pub endpoint_count: u8,
    pub base_address: Option<u32>,
    pub fifo: FifoConfig,
    #[serde(default)]
    pub reg_bit_size: RegBitSize,
    #[serde(default = "Vec::new")]
    pub patches: Vec<Patch>,
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
    pub btye_size_total: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FixedFifoConfig {
    pub shared: bool,
    pub equal_size: bool,
    #[serde(default = "Vec::new")]
    pub btye_size_endpoints: Vec<u8>, // when equal_size is false
    pub btye_size: Option<u8>,  // when equal_size is true
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegBitSize {
    #[serde(default = "default_16")]
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
        RegBitSize {
            fifo: 16,
            intr: 16,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub description: Option<String>,
    pub items: Vec<BlockItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockItem {
    pub name: String,
    pub description: Option<String>,
    pub byte_offset: Option<String>,
    pub bit_size: Option<String>,
    pub fieldset: String,
}

fn default_16() -> u8 {
    16
}

// fn default_true() -> bool {
//     true
// }

// fn default_false() -> bool {
//     false
// }
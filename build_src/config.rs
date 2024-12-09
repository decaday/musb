use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde_yaml;

use anyhow::Result;
use crate::Config;

pub fn read_configs() -> Result<Config> {
    // Read the YAML file
    let mut file = File::open("registers/configs/py32f07x.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the YAML
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

impl Config {
    pub fn get_replacements(&self) -> HashMap<&str, String> {
        let mut replacements = HashMap::new();
        replacements.insert("FIFO_REG_BIT_SIZE", self.reg_bit_size.fifo.to_string());
        replacements.insert("INTR_REG_BIT_SIZE", self.reg_bit_size.intr.to_string());
        replacements.insert("ENDPOINT_COUNT", self.endpoint_count.to_string());
        replacements
    }
}

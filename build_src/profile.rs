use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde_yaml;

use crate::{Features, Profile};

pub fn read_profile(features: &Features) -> Profile {
    let builtin = features.builtin.clone();

    // Read the YAML file
    println!("registers/profiles/{builtin}.yaml");
    let mut file = File::open(format!("registers/profiles/{builtin}.yaml")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse the YAML
    serde_yaml::from_str(&contents).unwrap()
}

impl Profile {
    pub fn get_replacements(&self) -> HashMap<&str, String> {
        let mut replacements = HashMap::new();
        replacements.insert("FIFO_REG_BIT_SIZE", self.reg_bit_size.fifo.to_string());
        replacements.insert("INTR_REG_BIT_SIZE", self.reg_bit_size.intr.to_string());
        replacements.insert("ENDPOINTS_NUM", self.endpoints.len().to_string());
        replacements
    }
}

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde_yaml;

use crate::{Features, Profile};

pub fn read_profile(features: &Features) -> Profile {
    let builtin = &features.builtin;

    // Read the YAML file
    let path = Path::new("registers").join("profiles").join(format!("{}.yaml", builtin));
    println!("{}", path.display());
    
    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read profile at {}", path.display()));

    // Parse the YAML
    serde_yaml::from_str(&contents).expect("Failed to parse profile YAML")
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

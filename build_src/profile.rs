use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::env;

use serde_yaml;

use crate::{Profile, Bulitin};

pub fn read_profiles(builtin: &Bulitin) -> Profile {
    let builtin = builtin.0.clone();
    
    // Read the YAML file
    let mut file = File::open(format!("registers/profiles/{builtin}.yaml")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse the YAML
    let mut profile: Profile = serde_yaml::from_str(&contents).unwrap();

    if let Some(_) = profile.base_address {
        if let Ok(_) = env::var("MUSB_BASE_ADDRESS") {
            panic!("The base_address field exists and the environment variable `MUSB_BASE_ADDRESS` is set");
        }
    } else {
        let bass_address = env::var("MUSB_BASE_ADDRESS")
            .expect("The base_address field does not exist and the environment variable `MUSB_BASE_ADDRESS` is not set");
        
        let bass_address = if bass_address.starts_with("0x") {
            u32::from_str_radix(&bass_address[2..], 16).expect(format!("Invalid `MUSB_BASE_ADDRESS` hexadecimal number: {}", bass_address).as_str())
        } else {
            bass_address.parse().expect(format!("Invalid `MUSB_BASE_ADDRESS` number: {}", bass_address).as_str())
        };
        profile.base_address = Some(bass_address);
    }
    profile
}

impl Profile {
    pub fn get_replacements(&self) -> HashMap<&str, String> {
        let mut replacements = HashMap::new();
        replacements.insert("FIFO_REG_BIT_SIZE", self.reg_bit_size.fifo.to_string());
        replacements.insert("INTR_REG_BIT_SIZE", self.reg_bit_size.intr.to_string());
        replacements.insert("ENDPOINT_COUNT", self.endpoint_count.to_string());
        replacements
    }
}
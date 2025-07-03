use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde_yaml;

use crate::{Features, Profile, EndpointDirection, EndpointConfig};

pub fn read_profiles(features: &Features) -> Profile {
    let builtin = features.builtin.clone();

    // Read the YAML file
    let mut file = File::open(format!("registers/profiles/{builtin}.yaml")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse the YAML
    let mut profile: Profile = serde_yaml::from_str(&contents).unwrap();

    if profile.endpoints.len() != 0 {
        if let Some(_) = features.endpoints_num {
            panic!("The endpoints_num field in profie exists and the endpoints_num_x feature is enabled.");
        }
    } else {
        if None == features.endpoints_num {
            panic!("The endpoints_num field in profie does not exist and the endpoints_num_x feature is not enabled.");
        }
        if let Some(num) = features.endpoints_num {
            profile.endpoints = vec![EndpointConfig {
                ep_direction: EndpointDirection::RXTX,
                max_packet_size_dword: num,
            }];
        }
    }

    profile
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

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::env;

use serde_yaml;

use crate::{Profile, FifoConfig};

pub fn read_profiles() -> Profile {
    let builtin = match env::vars()
        .map(|(a, _)| a)
        .filter(|x| x.starts_with("CARGO_FEATURE_BUILTIN"))
        .get_one()
    {
        Ok(x) => Some({
            x.strip_prefix("CARGO_FEATURE_BUILTIN_")
            .unwrap()
            .to_ascii_lowercase()
        }),
        Err(GetOneError::None) => None,
        Err(GetOneError::Multiple) => panic!("Multiple builtin-xxx Cargo features enabled"),
    };

    eprintln!("builtin: {builtin:?}");

    let builtin = if let Some(builtin) = builtin {
        builtin
    } else { // TODO
        panic!("No builtin-xxx Cargo features enabled");
    };

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

    pub fn get_features(&self) -> Vec<String> {
        let mut features = Vec::new();
        match &self.fifo {
            FifoConfig::Fixed(fifo) => {
                features.push("_fixed-fifo-size".to_string());
                if fifo.equal_size {
                    features.push("_equal-fifo-size".to_string());
                }
                if fifo.shared {
                    features.push("_ep-shared-fifo".to_string());
                }
            },
            FifoConfig::Dynamic(_) => (),
        }
        features
    }
}

enum GetOneError {
    None,
    Multiple,
}

trait IteratorExt: Iterator {
    fn get_one(self) -> Result<Self::Item, GetOneError>;
}

impl<T: Iterator> IteratorExt for T {
    fn get_one(mut self) -> Result<Self::Item, GetOneError> {
        match self.next() {
            None => Err(GetOneError::None),
            Some(res) => match self.next() {
                Some(_) => Err(GetOneError::Multiple),
                None => Ok(res),
            },
        }
    }
}
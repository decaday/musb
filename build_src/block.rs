use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::Block;
use anyhow::{anyhow, Result};

pub fn extract_fieldsets_from_block(block_name: &str) -> Result<Vec<String>> {
    let path = format!("registers/blocks/{block_name}.yaml");
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let parsed_data: HashMap<String, Block> = serde_yaml::from_str(&content)?;
    
    parsed_data.get(&format!("block/USB")).map_or_else(
        || Err(anyhow!("block/USB not found")),
        |block| Ok(block.items.iter().map(|item| item.fieldset.clone()).collect()),
    )
}
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::Block;

pub fn extract_fieldsets_from_block(block_name: &str) -> Vec<String> {
    let path = format!("registers/blocks/{block_name}.yaml");
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    let parsed_data: HashMap<String, Block> = serde_yaml::from_str(&content).unwrap();
    
    parsed_data.get(&format!("block/USB")).map_or_else(
        || panic!("block/USB not found"),
        |block| block.items.iter().map(|item| item.fieldset.clone()).collect(),
    )
}
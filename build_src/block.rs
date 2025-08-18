use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

use crate::Block;

fn read_block_file(block_name: &str) -> Block {
    let path = format!("registers/blocks/{block_name}.yaml");
    let mut file =
        File::open(&path).unwrap_or_else(|_| panic!("Failed to open block file: {}", path));
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut parsed_data: HashMap<String, Block> = serde_yaml::from_str(&content).unwrap();
    parsed_data
        .remove(&format!("block/USB"))
        .unwrap_or_else(|| panic!("block/USB not found in {}", block_name))
}

// Public function to load a block and process its inheritance chain.
// Returns the final, merged block.
pub fn load_and_merge_block(block_name: &str) -> Block {
    // Load the initial child block.
    let mut block = read_block_file(block_name);

    // If the block inherits from a parent, process the inheritance.
    if let Some(parent_name) = &block.inherits {
        // Recursively load the parent block, which will also be fully merged.
        let parent_block = load_and_merge_block(parent_name);

        // Start with the parent's items as the base.
        let mut final_items = parent_block.items;
        // Create a map for quick lookup of parent items by name to handle overrides.
        let parent_item_map: HashMap<_, _> = final_items
            .iter()
            .enumerate()
            .map(|(i, item)| (item.name.clone(), i))
            .collect();

        // Iterate through the child's items to merge them.
        for child_item in block.items {
            if let Some(&index) = parent_item_map.get(&child_item.name) {
                // If an item with the same name exists in the parent, override it.
                final_items[index] = child_item;
            } else {
                // If it's a new item, add it to the list.
                final_items.push(child_item);
            }
        }

        // Update the block's items with the merged list.
        block.items = final_items;

        // The child's description takes precedence. If the child has no description,
        // use the parent's.
        if block.description.is_none() {
            block.description = parent_block.description;
        }
    }

    // Clear the inherits field so it won't be in the final serialized YAML.
    block.inherits = None;

    // Return the fully merged block.
    block
}

// This function now uses the merged block to extract unique fieldsets.
// It takes a reference to an already merged block.
pub fn extract_fieldsets_from_block(merged_block: &Block) -> Vec<String> {
    merged_block
        .items
        .iter()
        .map(|item| item.fieldset.clone())
        .collect::<HashSet<_>>() // Use HashSet to automatically handle duplicates.
        .into_iter()
        .collect() // Convert back to a Vec.
}

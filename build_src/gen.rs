use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::env;

use anyhow::Result;

pub fn gen_regs_yaml(files: &Vec<String>, replacements: &HashMap<&str, String>) -> Result<String> {
    // Get the OUT_DIR environment variable, defaulting to "out" directory if not set
    let out_dir = env::var("OUT_DIR").unwrap();

    // Create the output path
    let output_path = Path::new(&out_dir).join("regs.yaml");

    // Open the output file for writing
    let mut output_file = File::create(&output_path)?;

    // Iterate over the list of files and perform replacements
    for file_path in files {
        // Read the content of the file
        let content = fs::read_to_string(file_path)?;

        // Perform the replacement operations
        let mut modified_content = content.clone();
        for (key, value) in replacements {
            modified_content = modified_content.replace(*key, value);
        }

        // Write the modified content into the output file
        write!(output_file, "{modified_content}\n")?;
    }

    // Return the path to the output file
    Ok(output_path.to_str().unwrap().to_string())
}
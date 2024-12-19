use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::env;
use std::process::Command;

pub fn gen_regs_yaml(files: &Vec<String>, replacements: &HashMap<&str, String>) {
    // Get the OUT_DIR environment variable, defaulting to "out" directory if not set
    let out_dir = env::var("OUT_DIR").unwrap();

    // Create the output path
    let output_path = Path::new(&out_dir).join("usb_regs.yaml");

    // Open the output file for writing
    let mut output_file = File::create(&output_path).unwrap();

    // Iterate over the list of files and perform replacements
    for file_path in files {
        // Read the content of the file
        let content = fs::read_to_string(file_path).unwrap();

        // Perform the replacement operations
        let mut modified_content = content.clone();
        for (key, value) in replacements {
            modified_content = modified_content.replace(*key, value);
        }

        // Write the modified content into the output file
        write!(output_file, "{modified_content}\n").unwrap();
    }
}

/// Generate USB register Rust code from YAML using yaml2pac and format with rustfmt
///
/// This function:
/// 1. Retrieves the OUT_DIR environment variable
/// 2. Constructs input and output file paths
/// 3. Executes yaml2pac to generate Rust code from YAML
/// 4. Uses rustfmt to format the generated Rust file
pub fn gen_usb_pac(base_address: Option<u32>) {
    // Retrieve OUT_DIR environment variable
    let out_dir = env::var("OUT_DIR")
        .expect("OUT_DIR environment variable not set");

    // Construct full paths for input and output files
    let input_path = Path::new(&out_dir).join("usb_regs.yaml");
    let output_path = Path::new(&out_dir).join("usb_regs.tmp");

    // Execute yaml2pac command to generate Rust code from YAML
    let yaml2pac_status = Command::new("yaml2pac")
        .arg("-i")
        .arg(input_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .arg("--common")
        .status()
        .unwrap();

    // Check yaml2pac command execution status
    if !yaml2pac_status.success() {
        panic!("yaml2pac command failed: {}", yaml2pac_status.code().unwrap());
    }

    // Execute rustfmt to format the generated Rust file
    let rustfmt_status = Command::new("rustfmt")
        .arg(output_path.to_str().unwrap())
        .status()
        .expect("Failed to execute rustfmt");

    // Check rustfmt command execution status
    if !rustfmt_status.success() {
        panic!("rustfmt command failed: {}", yaml2pac_status.code().unwrap());
    }

    let file = File::open(output_path).unwrap();
    let reader = BufReader::new(file);

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(&out_dir).join("usb_regs.rs")).unwrap();

    if let Some(base_address) = base_address {
        // Insert content
        let insert_content = format!(
r#"pub struct UsbInstance;
impl crate::MusbInstance for UsbInstance {{
    fn regs() -> crate::regs::Usb {{
        unsafe {{ Usb::from_ptr(({base_address:#x}) as _ ) }}
    }}
}}
"#); 
            output_file.write_all(insert_content.as_bytes()).unwrap();
    }

    // Ignore #![allow(clippy::xxxxx)]
    // (Moved to src/regs.rs)
    let lines = reader.lines().skip(4);

    for line in lines {
        output_file.write_all(line.unwrap().as_bytes()).unwrap();
        output_file.write_all(b"\n").unwrap();
    }
}
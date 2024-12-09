use std::fs::File;
use std::io::Read;

use serde_yaml;

mod build_src;
use build_src::build_serde::*;
use build_src::fieldset::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=registers");
    println!("cargo:rerun-if-changed=build_src");

    // Read the YAML file
    let mut file = File::open("registers/configs/py32f07x.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the YAML
    let config: Config = serde_yaml::from_str(&contents)?;
    println!("{:#?}", config);

    let fieldset_db = FieldsetDatabase::new_from_file()?;

    for fieldset in fieldset_db.fieldsets {
        println!("{:#?}", fieldset);
    }
    panic!("stop");
    Ok(())
}
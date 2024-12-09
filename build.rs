use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

use serde_yaml;

mod build_src;
use build_src::build_serde::*;
use build_src::fieldset::*;
use build_src::block::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=registers");
    println!("cargo:rerun-if-changed=build_src");

    // Read the YAML file
    let mut file = File::open("registers/configs/py32f07x.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the YAML
    let config: Config = serde_yaml::from_str(&contents)?;
    // println!("{:#?}", config);

    let fieldsets = extract_fieldsets_from_block(&config.block).unwrap();
    let fieldset_db = FieldsetDatabase::new_from_file().unwrap();

    // for fieldset in &fieldset_db.fieldsets {
    //     println!("{:?}", fieldset);
    // }

    // println!("{:#?}", fieldsets);
    
    for fieldset in &fieldsets {
        let version = if let Some(patch) = config.patches.iter()
            .find(|p| p.fieldset == *fieldset) 
        {
            patch.version.clone()
        } else {
            "std".to_string()
        };

        let mode = "peri".to_string();

        let path = fieldset_db.find_files(fieldset, 
            &Some(HashSet::from([version.clone()])), 
            &Some(HashSet::from(["host".to_string()])), 
            &Some(HashSet::from([mode.clone()])),
        ).unwrap();

        println!("{} {} {}", fieldset, version, path);
    }

    // panic!("stop");
    Ok(())
}
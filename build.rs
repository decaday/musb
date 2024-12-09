use std::collections::HashSet;

mod build_src;
use build_src::gen;
use build_src::build_serde::*;
use build_src::fieldset::*;
use build_src::block::*;
use build_src::config::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=registers");
    println!("cargo:rerun-if-changed=build_src");

    let config = read_configs("std_full").unwrap();
    // println!("{:#?}", config);

    let fieldsets = extract_fieldsets_from_block(&config.block).unwrap();
    let fieldset_db = FieldsetDatabase::new_from_file().unwrap();

    // for fieldset in &fieldset_db.fieldsets {
    //     println!("{:?}", fieldset);
    // }

    // println!("{:#?}", fieldsets);

    let mut regs_yaml_files = Vec::new();

    regs_yaml_files.push(format!("registers/blocks/{}.yaml", &config.block).to_string());
    
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
        
        println!("{} {} {}", fieldset, version, &path);
        regs_yaml_files.push(path);
    }

    gen::gen_regs_yaml(&regs_yaml_files, &config.get_replacements()).unwrap();
    gen::gen_usb_pac().unwrap();

    panic!("stop");
    Ok(())
}
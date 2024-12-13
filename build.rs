#[cfg(not(feature = "prebuild"))]
use std::collections::HashSet;

#[cfg(not(feature = "prebuild"))]
mod build_src;

#[cfg(not(feature = "prebuild"))]
use build_src::{gen, 
    build_serde::*, 
    fieldset::*,
    block::*,
    profile::*
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=registers");
    println!("cargo:rerun-if-changed=build_src");
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(not(feature = "prebuild"))]
    build();

    // panic!("stop");
    Ok(())
}

#[cfg(not(feature = "prebuild"))]
fn build() {
    let profile = read_profiles();
    // println!("{:#?}", profile);

    let fieldsets = extract_fieldsets_from_block(&profile.block);
    let fieldset_db = FieldsetDatabase::new_from_file();

    // for fieldset in &fieldset_db.fieldsets {
    //     println!("{:?}", fieldset);
    // }

    // println!("{:#?}", fieldsets);

    let mut regs_yaml_files = Vec::new();

    regs_yaml_files.push(format!("registers/blocks/{}.yaml", &profile.block).to_string());
    
    for fieldset in &fieldsets {
        let version = if let Some(patch) = profile.patches.iter()
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
        );
        
        println!("{} {} {}", fieldset, version, &path);
        regs_yaml_files.push(path);
    }

    gen::gen_regs_yaml(&regs_yaml_files, &profile.get_replacements());
    gen::gen_usb_pac(profile.base_address.unwrap());
}
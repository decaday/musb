#[cfg(not(feature = "prebuild"))]
use std::collections::{HashMap, HashSet};
#[cfg(not(feature = "prebuild"))]
use std::{env, fs, path::Path};
#[cfg(not(feature = "prebuild"))]
use serde_yaml;

mod build_src;
use build_src::feature::*;

#[cfg(not(feature = "prebuild"))]
use build_src::{block::*, build_serde::*, fieldset::*, gen, profile::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=registers");
    println!("cargo:rerun-if-changed=build_src");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=FEATURE_LIST");

    #[cfg(not(feature = "prebuild"))]
    build();

    #[cfg(feature = "prebuild")]
    prebuild();

    // panic!("stop");
    Ok(())
}

#[cfg(feature = "prebuild")]
fn prebuild() {
    let feature = Features::get();
    let features = FeatureGenerator::get_from_prebuild(&feature);
    features.gen();
}

#[cfg(not(feature = "prebuild"))]
fn build() {
    let features = Features::get();

    let profile = read_profile(&features);

    // 1. Load the block and process any inheritance to get the final, merged block.
    //    (This part remains unchanged from the previous modification)
    let final_block = load_and_merge_block(&profile.block);

    // 2. Serialize the MODIFIED `Value` object. Numerical values will be unquoted.
    let yaml_content = serialize_block_to_yaml_string(&final_block);

    // The rest of the function continues as before, writing the modified YAML content to a file.
    let out_dir = env::var("OUT_DIR").unwrap();
    let merged_block_path = Path::new(&out_dir).join(format!("{}_merged.yaml", &profile.block));
    fs::write(&merged_block_path, yaml_content).unwrap();

    // Extract fieldsets from the merged block object we already have in memory.
    let fieldsets = extract_fieldsets_from_block(&final_block);

    let fieldset_db = FieldsetDatabase::new_from_file();

    let mut regs_yaml_files = Vec::new();

    // Add the path to our NEWLY CREATED merged block file to the list of files to concatenate.
    regs_yaml_files.push(merged_block_path.to_str().unwrap().to_string());

    for fieldset in &fieldsets {
        let version = if let Some(patch) = profile.patches.iter().find(|p| p.fieldset == *fieldset)
        {
            patch.version.clone()
        } else {
            "std".to_string()
        };

        let mode = "peri".to_string();

        let path = fieldset_db.find_files(
            fieldset,
            &Some(HashSet::from([version.clone()])),
            &Some(HashSet::from(["host".to_string()])),
            &Some(HashSet::from([mode.clone()])),
        );

        println!("{} {} {}", fieldset, version, &path);
        regs_yaml_files.push(path);
    }

    let features = FeatureGenerator::get_from_profile(&profile);
    features.gen();
    features.gen_file();

    gen::gen_regs_yaml(&regs_yaml_files, &profile.get_replacements());
    gen::gen_usb_pac();
    gen::gen_info(&profile);
}

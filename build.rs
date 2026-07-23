#[cfg(not(feature = "prebuild"))]
use std::collections::HashSet;
#[cfg(not(feature = "prebuild"))]
use std::{env, fs, path::Path};

mod build_src;
use build_src::feature::*;

#[cfg(not(feature = "prebuild"))]
use build_src::{block::*, build_serde::*, gen, profile::*};

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
    let mut final_block = load_and_merge_block(&profile.block);

    let mut added_paths = HashSet::new();
    let mut regs_yaml_files = Vec::new();
    let out_dir = env::var("OUT_DIR").unwrap();
    let merged_block_path = Path::new(&out_dir).join(format!("{}_merged.yaml", &profile.block));

    // Add the path to our NEWLY CREATED merged block file to the list of files to concatenate.
    regs_yaml_files.push(merged_block_path.to_str().unwrap().to_string());

    for item in &mut final_block.items {
        let target_path = if let Some(patch) = profile.patches.iter().find(|p| p.item == item.name)
        {
            patch.path.clone()
        } else {
            item.fieldset.clone()
        };

        let full_path = Path::new("registers/fieldsets").join(&target_path);

        if added_paths.insert(target_path.clone()) {
            println!("{} -> {}", item.name, target_path);
            regs_yaml_files.push(full_path.to_str().unwrap().to_string());
        }

        let content = fs::read_to_string(&full_path).unwrap();
        let fieldset_line = content.lines().find(|l| l.starts_with("fieldset/")).unwrap();
        let logical_name = fieldset_line.trim_start_matches("fieldset/").trim_end_matches(':').to_string();
        item.fieldset = logical_name;
    }

    // 2. Serialize the MODIFIED `Value` object. Numerical values will be unquoted.
    let yaml_content = serialize_block_to_yaml_string(&final_block);
    fs::write(&merged_block_path, yaml_content).unwrap();

    let features = FeatureGenerator::get_from_profile(&profile);
    features.gen();
    features.gen_file();

    gen::gen_regs_yaml(&regs_yaml_files, &profile.get_replacements());
    gen::gen_usb_pac();
    gen::gen_info(&profile);
}

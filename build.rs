#[cfg(not(feature = "prebuild"))]
use std::collections::HashSet;

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

    let profile = read_profiles(&features);
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

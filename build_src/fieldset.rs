use std::{collections::{HashMap, HashSet}, fs, path::Path};

use anyhow::{Context, Result, anyhow};

#[derive(Clone, Debug)]
pub struct Fieldset {
    name: String,
    tags: HashSet<String>,
    file_path: String,
}

impl Fieldset {
    fn new(name: &str, tags: HashSet<String>, file_path: &str) -> Self {
        Self {
            name: name.to_string(),
            tags,
            file_path: file_path.to_string(),
        }
    }
}

pub struct FieldsetDatabase {
    pub fieldsets: Vec<Fieldset>,
}

impl FieldsetDatabase {
    pub fn new() -> Self {
        Self { fieldsets: Vec::new() }
    }

    /// Process the directory and build the database
    pub fn new_from_file() -> Result<Self> {
        let mut db = FieldsetDatabase::new();
        let root_path = "registers\\fieldsets";
        let initial_tags = HashSet::new();
        process_directory(root_path, initial_tags, &mut db)?;
        Ok(db)
    }

    fn add_fieldset(&mut self, fieldset: Fieldset) {
        self.fieldsets.push(fieldset);
    }

    pub fn find_files(
        &self,
        name: &str,
        must_have_tags: &Option<HashSet<String>>,
        must_not_have_tags: &Option<HashSet<String>>,
        best_have_tags: &Option<HashSet<String>>,
    ) -> Result<String> {
        let mut matching_files = Vec::new();

        for fieldset in &self.fieldsets {
            // Check if the name matches
            if fieldset.name != name {
                continue;
            }

            // Check if the Fieldset contains all must-have tags
            if let Some(tags) = must_have_tags {
                if !tags.is_subset(&fieldset.tags) {
                    continue;
                }
            }

            // Check if the Fieldset contains none of the must-not-have tags
            if let Some(tags) = must_not_have_tags {
                if !tags.is_disjoint(&fieldset.tags) {
                    continue;
                }
            }

            // Check if the Fieldset contains any of the best-have tags
            if let Some(tags) = best_have_tags {
                if !tags.is_disjoint(&fieldset.tags) {
                    matching_files.push((fieldset.file_path.clone(), true));
                }
            } else {
                matching_files.push((fieldset.file_path.clone(), false));
            }
        }

        // If there are multiple matching results, return an error
        if matching_files.len() > 1 {
            let best_files: Vec<String> = matching_files
            .iter()
            .filter(|(_, is_true)| *is_true)
            .map(|(file, _)| file.clone())
            .collect();
    
            if best_files.len() == 1 {
                Ok(best_files.into_iter().next().unwrap())
            } else {
                Err(anyhow!("Invalid list: {matching_files:?}"))
                    .context("Expected exactly one file with true value in the list")
            }

        } else if matching_files.is_empty() {
            Err(anyhow!("No matching file found. must_have_tags: {must_have_tags:?},
                must_not_have_tags: {must_not_have_tags:?},
                best_have_tags: {best_have_tags:?}"))
        } else {
            Ok(matching_files[0].0.clone()) // Return the single file path
        }
    }
}


/// Recursively walks through the directory and processes files
fn process_directory<P: AsRef<Path>>(path: P, parent_tags: HashSet<String>, db: &mut FieldsetDatabase) -> Result<()> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            // For directories, split the directory name and apply tags to its files
            let folder_name = entry_path.file_name().unwrap().to_string_lossy().to_string();
            let mut folder_tags = parent_tags.clone();
            add_tags_from_name(&folder_name, &mut folder_tags);

            // Process the contents of the directory
            process_directory(entry_path, folder_tags, db)?;
        } else if entry_path.is_file() {
            // For files, process the file
            let file_name = entry_path.file_name().unwrap().to_string_lossy().to_string();
            let mut file_tags = parent_tags.clone();
            add_tags_from_name(&file_name, &mut file_tags);
            db.add_fieldset(Fieldset::new(&file_name, file_tags, &entry_path.to_string_lossy()));
        }
    }
    Ok(())
}

/// Extract tags from the file or folder name (separated by `_`)
fn add_tags_from_name(name: &str, tags: &mut HashSet<String>) {
    let parts: Vec<&str> = name.split('_').collect();
    if parts.len() > 1 {
        // Skip the first part (which is the name itself)
        tags.extend(parts[1..].iter().map(|&s| s.to_string()));
    }
}


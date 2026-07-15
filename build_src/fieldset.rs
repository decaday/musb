use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Fieldset {
    pub name: String,
    pub tags: HashSet<String>,
    pub file_path: PathBuf,
}

impl Fieldset {
    fn new(name: &str, tags: HashSet<String>, file_path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            tags,
            file_path,
        }
    }
}

pub struct FieldsetDatabase {
    pub fieldsets: Vec<Fieldset>,
}

impl FieldsetDatabase {
    pub fn new() -> Self {
        Self {
            fieldsets: Vec::new(),
        }
    }

    /// Process the directory and build the database
    pub fn new_from_file() -> Self {
        let mut db = FieldsetDatabase::new();
        let root_path = Path::new("registers").join("fieldsets");
        let initial_tags = HashSet::new();
        process_directory(&root_path, initial_tags, &mut db);
        db
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
    ) -> String {
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
                } else {
                    matching_files.push((fieldset.file_path.clone(), false));
                }
            } else {
                matching_files.push((fieldset.file_path.clone(), false));
            }
        }

        // If there are multiple matching results, return an error
        if matching_files.len() > 1 {
            let best_files: Vec<PathBuf> = matching_files
                .into_iter()
                .filter(|(_, is_true)| *is_true)
                .map(|(file, _)| file)
                .collect();

            if best_files.len() == 1 {
                best_files.into_iter().next().unwrap().to_str().unwrap().to_string()
            } else {
                panic!("Invalid list: {best_files:?}\nExpected exactly one file with true value in the list")
            }
        } else if matching_files.is_empty() {
            panic!(
                "No matching file found for {name}. must_have_tags: {must_have_tags:?},
                must_not_have_tags: {must_not_have_tags:?},
                best_have_tags: {best_have_tags:?}"
            )
        } else {
            matching_files[0].0.to_str().unwrap().to_string() // Return the single file path
        }
    }
}

/// Recursively walks through the directory and processes files
fn process_directory<P: AsRef<Path>>(
    path: P,
    parent_tags: HashSet<String>,
    db: &mut FieldsetDatabase,
) {
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            // For directories, split the directory name and apply tags to its files
            let folder_name = entry_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let mut folder_tags = parent_tags.clone();
            let _ = add_tags_from_name(&folder_name, &mut folder_tags);

            // Process the contents of the directory
            process_directory(&entry_path, folder_tags, db);
        } else if entry_path.is_file() {
            // For files, process the file
            let file_name = entry_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let mut file_tags = parent_tags.clone();
            let name = add_tags_from_name(&file_name, &mut file_tags);
            db.add_fieldset(Fieldset::new(
                name,
                file_tags,
                entry_path,
            ));
        }
    }
}

/// Extract tags from the file or folder name (separated by `_`)
fn add_tags_from_name<'a>(name: &'a str, tags: &mut HashSet<String>) -> &'a str {
    let split_index = name.as_bytes().windows(2).enumerate()
        .find(|(_, window)| {
            // Check if the window matches the pattern: `_` followed by a lowercase ASCII character.
            window[0] == b'_' && window[1].is_ascii_lowercase()
        })
        .map(|(i, _)| i);

    match split_index {
        Some(index) => {
            // The name is the part of the string before the underscore.
            let name_part = &name[..index];
            // The tags start from the character after the underscore.
            let tags_part = &name[index + 1..];
            
            // Split the tags part by underscores and add them to the provided HashSet.
            tags.extend(tags_part.split('_').filter(|s| !s.is_empty()).map(|s| s.to_string()));
            name_part
        }
        None => name
    }
}

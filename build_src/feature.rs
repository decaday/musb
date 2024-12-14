use std::env;

pub fn get_builtin() -> String {
    let builtin = match env::vars()
        .map(|(a, _)| a)
        .filter(|x| x.starts_with("CARGO_FEATURE_BUILTIN"))
        .get_one()
    {
        Ok(x) => Some({
            x.strip_prefix("CARGO_FEATURE_BUILTIN_")
            .unwrap()
            .to_ascii_lowercase()
        }),
        Err(GetOneError::None) => None,
        Err(GetOneError::Multiple) => panic!("Multiple builtin-xxx Cargo features enabled"),
    };

    eprintln!("builtin: {builtin:?}");

    if let Some(builtin) = builtin {
        builtin
    } else { // TODO
        panic!("No builtin-xxx Cargo features enabled");
    }
}

pub fn gen_features(features: &Vec<String>) {
    for feature in features {
        println!("cargo:rustc-cfg=feature=\"{}\"", feature);
    }
}

#[cfg(feature = "prebuild")]
pub fn get_features_from_prebuild(builtin: &str) -> Vec<String> {
    use std::path::Path;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufRead;

    let file_path = format!("src/prebuilds/{}/features.txt", builtin);
    let path = Path::new(&file_path);
    
    // Open the file in read-only mode.
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read lines, filter out empty ones, and collect into a Vec<String>.
    let features: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok()) // Handle potential errors for individual lines
        .map(|line| line.trim().to_string()) // Trim whitespace
        .filter(|line| !line.is_empty()) // Remove empty lines
        .collect();

    features
}

enum GetOneError {
    None,
    Multiple,
}

trait IteratorExt: Iterator {
    fn get_one(self) -> Result<Self::Item, GetOneError>;
}

impl<T: Iterator> IteratorExt for T {
    fn get_one(mut self) -> Result<Self::Item, GetOneError> {
        match self.next() {
            None => Err(GetOneError::None),
            Some(res) => match self.next() {
                Some(_) => Err(GetOneError::Multiple),
                None => Ok(res),
            },
        }
    }
}
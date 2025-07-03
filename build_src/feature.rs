use std::env;

#[allow(dead_code)]
pub struct Features {
    pub builtin: String,
}

impl Features {
    fn get_one_feature(name: &str) -> Option<String> {
        let name_upper = name.to_ascii_uppercase();

        match env::vars()
            .map(|(a, _)| a)
            .filter(|x| x.starts_with(format!("CARGO_FEATURE_{}", &name_upper).as_str()))
            .get_one()
        {
            Ok(x) => Some({
                x.strip_prefix(&format!("CARGO_FEATURE_{}_", &name_upper))
                    .unwrap()
                    .to_ascii_lowercase()
            }),
            Err(GetOneError::None) => None,
            Err(GetOneError::Multiple) => panic!("Multiple {}-xxx Cargo features enabled", name),
        }
    }

    pub fn get() -> Self {
        let builtin =
            Self::get_one_feature("builtin")
                .expect("No builtin-xxx Cargo features enabled")
                .replace('_', "-"); // Replace underscores with dashes for consistency
        Self { builtin }
    }
}

pub struct FeatureGenerator(pub Vec<String>);

impl FeatureGenerator {
    pub fn gen(&self) {
        for feature in self.0.iter() {
            println!("cargo:rustc-cfg=feature=\"{}\"", feature);
        }
    }

    #[cfg(not(feature = "prebuild"))]
    pub fn get_from_profile(profile: &crate::Profile) -> Self {
        use crate::FifoConfig;

        let mut features = Vec::new();
        match &profile.fifo {
            FifoConfig::Fixed(fifo) => {
                features.push("_fixed-fifo-size".to_string());
                if fifo.shared {
                    features.push("_ep-shared-fifo".to_string());
                }
            }
            FifoConfig::Dynamic(_) => (),
        }

        if let Some(_) = profile.base_address {
            features.push("_gen-usb-instance".to_string());
        }
        Self(features)
    }

    #[cfg(feature = "prebuild")]
    pub fn get_from_prebuild(features: &Features) -> Self {
        use std::fs::File;
        use std::io::BufRead;
        use std::io::BufReader;
        use std::path::Path;

        let file_path = format!("src/prebuilds/{}/features.txt", features.builtin);
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

        Self(features)
    }

    #[cfg(not(feature = "prebuild"))]
    pub fn gen_file(&self) {
        use std::env;
        use std::fs;
        use std::path::Path;

        let out_dir = env::var("OUT_DIR").unwrap();
        let file_path = Path::new(&out_dir).join("features.txt");

        let mut content = String::new();
        for feature in self.0.iter() {
            content.push_str(&format!("{}\n", feature));
        }

        fs::write(&file_path, content).unwrap();
    }
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

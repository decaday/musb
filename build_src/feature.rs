use std::env;

pub struct Bulitin(pub String);

impl Bulitin {
    pub fn get() -> Self {
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
            Self(builtin)
        } else { // TODO
            panic!("No builtin-xxx Cargo features enabled");
        }
    }
}

pub struct Features(pub Vec<String>);

impl Features {
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
                if fifo.equal_size {
                    features.push("_equal-fifo-size".to_string());
                }
                if fifo.shared {
                    features.push("_ep-shared-fifo".to_string());
                }
            },
            FifoConfig::Dynamic(_) => (),
        }
        Self(features)
    }
    
    #[cfg(feature = "prebuild")]
    pub fn get_from_prebuild(builtin: &Bulitin) -> Self {
        use std::path::Path;
        use std::fs::File;
        use std::io::BufReader;
        use std::io::BufRead;
    
        let file_path = format!("src/prebuilds/{}/features.txt", builtin.0);
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
        use std::path::Path;
        use std::fs;
        use std::env;

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
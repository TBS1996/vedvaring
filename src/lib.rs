use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::{Read, Write};
use std::path::PathBuf;

pub trait Persist: for<'a> Deserialize<'a> + Serialize {
    fn name(&self) -> String;
    fn dir() -> PathBuf;

    fn path(&self) -> PathBuf {
        let mut x = Self::dir().join(self.name());
        x.set_extension("toml");
        x
    }

    fn load_all() -> Vec<Self> {
        let mut instances = Vec::new();
        let paths = std::fs::read_dir(Self::dir()).expect("Failed to read directory");

        for path in paths {
            let path = path.expect("Failed to read path").path();
            if path.extension().map(|ext| ext == "toml").unwrap_or(false) {
                let mut file = File::open(&path).expect("Failed to open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("Failed to read file");

                if let Ok(instance) = toml::from_str(&contents) {
                    instances.push(instance);
                }
            }
        }

        instances
    }

    fn save(&self) {
        let serialized = toml::to_string(self).expect("Failed to serialize");
        let mut file = File::create(self.path()).expect("Failed to create file");
        file.write_all(serialized.as_bytes())
            .expect("Failed to write to file");
    }
}

/// For singleton stuff, like config files, they need to implement default so
/// a new one is created at first load.
pub trait SingletonPersist: for<'a> Deserialize<'a> + Serialize + Default {
    fn name() -> String;
    fn dir() -> PathBuf;

    fn path() -> PathBuf {
        let mut x = Self::dir().join(Self::name());
        x.set_extension("toml");
        x
    }

    fn load() -> Self {
        let path = Self::path();

        if !path.exists() {
            Self::default().save();
        }

        let content = read_to_string(path).unwrap();

        toml::from_str(content.as_str()).unwrap()
    }
    fn save(&self) {
        let s = toml::to_string_pretty(self).unwrap();
        std::fs::create_dir_all(Self::dir()).unwrap();
        let p = Self::path();
        std::fs::write(p.as_path(), s).unwrap();
    }
}

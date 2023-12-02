use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::Read;
use std::path::PathBuf;

pub trait Persist: for<'a> Deserialize<'a> + Serialize {
    fn name(&self) -> String;
    fn dir_name() -> String;

    fn dir() -> PathBuf {
        let p = home::home_dir()
            .unwrap()
            .join(format!(".local/share/{}", Self::dir_name()));
        std::fs::create_dir_all(p.as_path()).unwrap();
        p
    }

    fn path(&self) -> PathBuf {
        let mut x = Self::dir().join(self.name());
        x.set_extension("toml");
        x
    }

    fn save(&self) {
        std::fs::create_dir_all(Self::dir()).unwrap();
        let s = toml::to_string_pretty(self).unwrap();
        std::fs::write(self.path().as_path(), s).unwrap();
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
}

/// For singleton stuff, like config files, they need to implement default so
/// a new one is created at first load.
pub trait SingletonPersist: for<'a> Deserialize<'a> + Serialize + Default {
    fn name() -> String;
    fn dir_name() -> String;
    fn save(&self) {
        std::fs::create_dir_all(Self::dir()).unwrap();
        let s = toml::to_string_pretty(self).unwrap();
        std::fs::write(Self::path().as_path(), s).unwrap();
    }

    fn dir() -> PathBuf {
        let p = home::home_dir()
            .unwrap()
            .join(format!(".local/share/{}", Self::dir_name()));
        std::fs::create_dir_all(p.as_path()).unwrap();
        p
    }

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
}

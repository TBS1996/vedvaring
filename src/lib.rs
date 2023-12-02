use std::path::PathBuf;
use std::fs::read_to_string;
use serde::{Serialize, Deserialize};


pub trait Persistence: for<'a> Deserialize<'a> + Serialize + Default {
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
            panic!("file not found: {}", path.display());
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

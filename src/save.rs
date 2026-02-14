use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Serialize, Deserialize)]
pub struct Save {
    pub items: Vec<Item>,

    #[serde(skip)]
    pub(crate) path: PathBuf,
}
impl Save {
    pub fn new(path: PathBuf) -> Self {
        Self {
            items: Vec::new(),
            path,
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Save> {
        let content = fs::read_to_string(&path)?;
        let mut save: Save = toml::from_str(&content).unwrap();
        save.path = path.as_ref().to_path_buf();
        Ok(save)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn flush(&self) -> io::Result<()> {
        let mut file = if fs::exists(&self.path)? {
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&self.path)?
        } else {
            fs::File::create_new(&self.path)?
        };
        write!(file, "{}", toml::to_string(self).unwrap())
    }
}

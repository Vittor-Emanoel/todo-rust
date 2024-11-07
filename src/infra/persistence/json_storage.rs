use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::PathBuf;

pub struct JsonStorage {
    file_path: PathBuf,
}

impl JsonStorage {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn read<T: DeserializeOwned>(&self) -> Result<Option<T>> {
        if !self.file_path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&self.file_path)?;
        let data = serde_json::from_str(&content)?;
        Ok(Some(data))
    }

    pub fn write<T: Serialize>(&self, data: &T) -> Result<()> {
        let content = serde_json::to_string_pretty(data)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }
}

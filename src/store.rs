use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

/// Get user storage directory.
pub fn get_store_directory() -> Result<PathBuf> {
    let mut path = dirs::home_dir().ok_or_else(|| anyhow!("Could not find home dir"))?;
    path.push(".anyshortcut");
    // Create the directory if not exist before write date to file.
    fs::create_dir_all(path.to_str().unwrap())?;
    Ok(path)
}

/// **Storage** trait which required supertraits (Serialize, DeserializeOwned)
/// to persist() and parse() target file.
pub trait Storage: Serialize + DeserializeOwned {
    /// Get storage file name.
    fn get_file_name() -> String;

    fn persist(&self) -> Result<()> {
        let mut path = get_store_directory()?;
        path.push(Self::get_file_name());

        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }

    fn parse() -> Result<Self> {
        let mut path = get_store_directory()?;
        path.push(Self::get_file_name());

        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    fn clear() -> Result<()> {
        let mut path = get_store_directory()?;
        path.push(Self::get_file_name());

        if path.exists() {
            fs::remove_file(path).unwrap();
        }

        Ok(())
    }
}

use constants;
use failure;
use failure::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;


/// **Storage** trait which required supertraits (Serialize, DeserializeOwned)
/// to persist() and parse() target file.
pub trait Storage: Serialize + DeserializeOwned {
    /// Get storage file name.
    fn get_file_name() -> String;

    fn persist(&self) -> Result<(), Error> {
        let mut path = env::home_dir()
            .ok_or_else(|| failure::err_msg("Could not find home dir"))?;
        path.push(constants::DIRECTORY_NAME);
        // Create the directory if not exist before write date to file.
        fs::create_dir_all(path.to_str().unwrap())?;
        path.push(Self::get_file_name());

        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }

    fn parse() -> Result<Self, Error> {
        let mut path = env::home_dir()
            .ok_or_else(|| failure::err_msg("Could not find home dir"))?;
        path.push(constants::DIRECTORY_NAME);
        // Create the directory if not exist before write date to file.
        fs::create_dir_all(path.to_str().unwrap())?;
        path.push(Self::get_file_name());

        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }
}
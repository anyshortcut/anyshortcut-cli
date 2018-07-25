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


lazy_static! {
    pub static ref META_STORAGE: Storage = Storage::new(constants::META_FILE_NAME);
}


pub struct Storage {
    file_name: String,
}

impl Storage {
    pub fn new(file_name: &str) -> Storage {
        Storage { file_name: file_name.to_string() }
    }

    pub fn persist<T: Serialize>(&self, content: T) -> Result<(), Error> {
        let mut path = env::home_dir()
            .ok_or_else(|| failure::err_msg("Could not find home dir"))?;
        path.push(constants::DIRECTORY_NAME);
        // Create the directory if not exist before write date to file.
        fs::create_dir_all(path.to_str().unwrap())?;
        path.push(&self.file_name);

        let file = File::create(path)?;
        serde_json::to_writer(file, &content)?;
        Ok(())
    }

    pub fn parse<T: DeserializeOwned>(&self) -> Result<T, Error> {
        let mut path = env::home_dir()
            .ok_or_else(|| failure::err_msg("Could not find home dir"))?;
        path.push(constants::DIRECTORY_NAME);
        // Create the directory if not exist before write date to file.
        fs::create_dir_all(path.to_str().unwrap())?;
        path.push(&self.file_name);

        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use models::*;

    #[test]
    fn storage_parse_works() {
        let r = META_STORAGE.parse::<Meta>();
        assert!(r.is_ok());
        println!("meta {:?}", r.unwrap());
    }

    #[test]
    fn storage_persist_works() {
        let meta = Meta { token: "abcdefg".to_string() };
        let r = META_STORAGE.persist::<Meta>(meta);
        assert!(r.is_ok());
    }
}
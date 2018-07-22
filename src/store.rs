use constants;
use failure;
use failure::Error;
use serde::Serialize;
use serde_json;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;


pub struct Storage {
    file_name: String,
}

impl Storage {
    pub fn new(file_name: &str) -> Storage {
        Storage { file_name: file_name.to_string() }
    }

    pub fn meta_storage() -> Storage {
        Storage::new(constants::META_FILE_NAME)
    }

    pub fn persist<T: Serialize>(&self, content: T) -> Result<(), Error> {
        let mut path = env::home_dir()
            .ok_or_else(|| failure::err_msg("Could not find home dir"))?;
        path.push(constants::DIRECTORY_NAME);
        // Create the directory if not exist before write date to file.
        fs::create_dir_all(path.to_str().unwrap())?;
        path.push(&self.file_name);

        let mut file = File::create(path)?;
        let mut content_bytes: Vec<u8> = vec![];

        serde_json::to_writer(&mut content_bytes, &content)?;
        file.write_all(&content_bytes)?;

        Ok(())
    }
}
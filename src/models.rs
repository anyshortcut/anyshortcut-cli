use serde_json;
use store::Storage;
use constants;


#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub token: String,
}


///
/// The Shortcut struct.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Shortcut {
    pub key: String,
    pub url: String,
    pub title: String,
    pub comment: String,
    pub favicon: String,
    pub domain: String,
    pub open_times: i32,
}

impl Storage for Meta {
    fn get_file_name() -> String {
        return constants::META_FILE_NAME.to_string();
    }
}
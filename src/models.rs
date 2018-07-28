use constants;
use serde_json;
use std::collections::HashMap;
use store::Storage;

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub token: String,
}

///
/// The Shortcut struct.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Shortcut {
    pub key: ShortcutKey,
    pub url: String,
    pub title: String,
    pub comment: Option<String>,
    pub favicon: Option<String>,
    pub domain: ShortcutDomain,
    pub open_times: i32,
}

/// A aliased type for shortcut key.
pub type ShortcutKey = String;
/// A aliased type for shortcut domain name.
pub type ShortcutDomain = String;
/// A aliased type for { "KEY": { SHORTCUT} } map.
pub type ShortcutMap = HashMap<ShortcutKey, Shortcut>;


#[derive(Serialize, Deserialize, Debug)]
pub struct ShortcutData {
    pub primary: Vec<ShortcutMap>,
    pub secondary: HashMap<ShortcutDomain, ShortcutMap>,
}

impl Meta {
    pub fn get_token() -> String {
        match Self::parse() {
            Ok(meta) => meta.token,
            Err(_) => String::from(""),
        }
    }
}

impl Storage for Meta {
    fn get_file_name() -> String {
        return constants::META_FILE_NAME.to_string();
    }
}
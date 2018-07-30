use constants;
use serde_json;
use std::collections::HashMap;
use std::ops::Deref;
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
    pub domain: ShortcutDomain,
    pub open_times: i32,
}

/// A aliased type for shortcut key.
pub type ShortcutKey = String;
/// A aliased type for shortcut domain name.
pub type ShortcutDomain = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct PrimaryShortcutVec(Vec<Shortcut>);

#[derive(Serialize, Deserialize, Debug)]
pub struct SecondaryShortcutMap(HashMap<ShortcutDomain, Vec<Shortcut>>);

#[derive(Serialize, Deserialize, Debug)]
pub struct ShortcutData {
    pub primary: PrimaryShortcutVec,
    pub secondary: SecondaryShortcutMap,
}

impl Meta {
    pub fn get_token() -> String {
        match Self::parse() {
            Ok(meta) => meta.token,
            Err(_) => String::from(""),
        }
    }
}

impl Deref for PrimaryShortcutVec {
    type Target = Vec<Shortcut>;

    fn deref(&self) -> &Vec<Shortcut> {
        &self.0
    }
}

impl Deref for SecondaryShortcutMap {
    type Target = HashMap<ShortcutDomain, Vec<Shortcut>>;

    fn deref(&self) -> &HashMap<ShortcutDomain, Vec<Shortcut>> {
        &self.0
    }
}

impl Storage for Meta {
    fn get_file_name() -> String {
        constants::META_FILE_NAME.to_string()
    }
}

impl Storage for PrimaryShortcutVec {
    fn get_file_name() -> String {
        constants::PRIMARY_FILE_NAME.to_string()
    }
}

impl Storage for SecondaryShortcutMap {
    fn get_file_name() -> String {
        constants::SECONDARY_FILE_NAME.to_string()
    }
}
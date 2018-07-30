use constants;
use open;
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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

pub struct ShortcutManager {}

impl ShortcutManager {
    pub fn get_primary_by_key(key: &str) -> Option<Shortcut> {
        match PrimaryShortcutVec::parse() {
            Ok(shortcuts) => {
                shortcuts.iter().cloned()
                    .find(|shortcut| shortcut.key.eq_ignore_ascii_case(key))
            }
            Err(_) => None
        }
    }

    pub fn get_secondary_by_domain_key(domain: &str, key: &str) -> Option<Shortcut> {
        match SecondaryShortcutMap::parse() {
            Ok(domain_shortcut_map) => {
                if let Some(shortcuts) = domain_shortcut_map.get(domain) {
                    shortcuts.iter().cloned()
                        .find(|shortcut| shortcut.key.eq_ignore_ascii_case(key))
                } else {
                    None
                }
            }
            Err(_) => None
        }
    }

    #[allow(unused)]
    pub fn get_secondary_by_keys(primary_key: &str, secondary_key: &str) -> Option<Shortcut> {
        if let Some(shortcut) = Self::get_primary_by_key(primary_key) {
            Self::get_secondary_by_domain_key(&shortcut.domain, secondary_key)
        } else {
            None
        }
    }

    pub fn open_primary(key: &str) {
        if let Some(shortcut) = Self::get_primary_by_key(key) {
            open::that(shortcut.url.clone()).unwrap();
        } else {
            println!("Not primary shortcut (key: {}) found.", key.to_uppercase());
        }
    }

    pub fn open_secondary(primary_key: &str, secondary_key: &str) {
        if let Some(primary_shortcut) = Self::get_primary_by_key(primary_key) {
            let domain = &primary_shortcut.domain;
            if let Some(shortcut) = Self::get_secondary_by_domain_key(domain, secondary_key) {
                open::that(shortcut.url.clone()).unwrap();
            } else {
                println!("No secondary shortcut (key: {}) found.", secondary_key.to_uppercase());
            }
        } else {
            println!("Not primary shortcut (key: {}) found.", primary_key.to_uppercase());
        }
    }
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
use ansi_term::{ANSIString, ANSIStrings, Style};
use ansi_term::Color::Yellow;
use chrono::{TimeZone, Utc};
use constants;
use open;
use std::collections::HashMap;
use std::fmt;
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
    pub id: u32,
    pub key: ShortcutKey,
    pub url: String,
    pub title: String,
    pub comment: Option<String>,
    pub domain: ShortcutDomain,
    pub open_times: i32,
    #[serde(rename = "created_time")]
    pub timestamp: i64,
}

impl Shortcut {
    pub fn pretty_print(&self) {
        println!();
        println!("{}", "-".repeat(60));
        let key_str: &[ANSIString<'static>] = &[
            Yellow.paint("["),
            Yellow.bold().paint(self.key.to_uppercase()),
            Yellow.paint("]"),
        ];
        println!("{}  {}", ANSIStrings(key_str), Style::new().bold().paint(&self.title));

        println!();
        self.fixed_label_print("Url:", &self.url);
        self.fixed_label_print("Comment:", self.comment.as_ref().unwrap_or(&String::from("")));
        self.fixed_label_print("Domain:", &self.domain);
        self.fixed_label_print("Open times:", &self.open_times);
        self.fixed_label_print("Created at:", Utc.timestamp_millis(self.timestamp));
        println!();
    }

    fn fixed_label_print(&self, label: &str, text: impl fmt::Display) {
        println!("{:14}{}", label, text);
    }
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
    pub fn get_primary_shortcuts() -> Option<Vec<Shortcut>> {
        PrimaryShortcutVec::parse().ok()
            .and_then(|shortcuts| {
                Some(shortcuts.iter().cloned()
                    .filter(|shortcut| shortcut.key.len() == 1)
                    .collect())
            })
    }

    pub fn get_compound_shortcuts() -> Option<Vec<Shortcut>> {
        PrimaryShortcutVec::parse().ok()
            .and_then(|shortcuts| {
                Some(shortcuts.iter().cloned()
                    .filter(|shortcut| shortcut.key.len() == 2)
                    .collect())
            })
    }

    pub fn get_secondary_shortcuts() -> Option<SecondaryShortcutMap> {
        SecondaryShortcutMap::parse().ok()
    }

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
            Self::open_shortcut(shortcut);
        } else {
            println!("Not primary shortcut (key: {}) found.", key.to_uppercase());
        }
    }

    pub fn open_secondary(primary_key: &str, secondary_key: &str) {
        if let Some(primary_shortcut) = Self::get_primary_by_key(primary_key) {
            let domain = &primary_shortcut.domain;
            if let Some(shortcut) = Self::get_secondary_by_domain_key(domain, secondary_key) {
                Self::open_shortcut(shortcut);
            } else {
                println!("No secondary shortcut (key: {}) found.", secondary_key.to_uppercase());
            }
        } else {
            println!("Not primary shortcut (key: {}) found.", primary_key.to_uppercase());
        }
    }

    fn open_shortcut(shortcut: Shortcut) {
        match open::that(shortcut.url.clone()) {
            Ok(_) => println!("Url: {}", shortcut.url),
            Err(error) => println!("{}", error),
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

    pub fn has_token() -> bool {
        Self::parse().is_ok()
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
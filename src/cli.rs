use clap::ArgMatches;
use commands::{list, login, logout, sync};
use failure::Error;
use models::ShortcutManager;

pub fn handle_matches(matches: &ArgMatches) -> Result<(), Error> {
    match matches.subcommand() {
        ("login", Some(login_matches)) => {
            login::execute(&login_matches)?
        }
        ("logout", Some(logout_matches)) => {
            logout::execute(&logout_matches)?
        }
        ("sync", Some(sync_matches)) => {
            sync::execute(&sync_matches)?
        }
        ("list", Some(list_matches)) => {
            list::execute(&list_matches)?
        }
        _ => {
            match (matches.value_of("primary_key"), matches.value_of("secondary_key")) {
                (Some(primary_key), Some(secondary_key)) => {
                    ShortcutManager::open_secondary(primary_key, secondary_key);
                }
                (Some(primary_key), None) => {
                    ShortcutManager::open_primary(primary_key);
                }
                (_, _) => {
                    // Impossible case, ignore...
                }
            }
        }
    };

    Ok(())
}

/// Validate primary key format.
/// Including one-letter primary key and two-letters compound key.
pub fn validate_primary_key(key: String) -> Result<(), String> {
    if key.chars().all(|c| c.is_ascii_alphanumeric()) {
        if key.len() == 0 || key.len() > 2 {
            return Err(String::from("Invalid key length"));
        }

        Ok(())
    } else {
        Err(String::from("Invalid primary key"))
    }
}

/// Validate secondary key format.
pub fn validate_secondary_key(key: String) -> Result<(), String> {
    if key.len() == 1 && key.chars().all(|c| c.is_ascii_alphanumeric()) {
        Ok(())
    } else {
        Err(String::from("Invalid secondary key"))
    }
}

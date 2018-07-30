use clap::ArgMatches;
use commands::{list, login, logout, sync};
use failure::Error;

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
            if matches.is_present("primary_key") {
                if matches.is_present("secondary_key") {
                    println!(
                        "secondary{:?} {:?}",
                        matches.value_of("primary_key"),
                        matches.value_of("secondary_key")
                    );
                } else {
                    println!("primary:{:?}", matches.value_of("primary_key"));
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

extern crate chrono;
#[macro_use]
extern crate clap;
extern crate curl_http;
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[cfg(test)]
extern crate itertools;
extern crate open;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use clap::ArgMatches;
use commands::{list, login, logout, sync};
use failure::Error;
use models::ShortcutManager;
use std::process;

mod api;
mod constants;
mod cli;
mod commands;
mod utils;
mod store;
mod models;

fn main() {
    let matches = cli::build_cli().get_matches();

    match handle_matches(&matches) {
        Ok(()) => process::exit(0),
        Err(error) => {
            println!("Error: {}", error);
            process::exit(1);
        }
    };
}

fn handle_matches(matches: &ArgMatches) -> Result<(), Error> {
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

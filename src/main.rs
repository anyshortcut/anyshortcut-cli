use std::process;

use clap::ArgMatches;
use failure::Error;

use crate::commands::{list, login, logout, sync};
use crate::models::ShortcutManager;

mod api;
mod cli;
mod commands;
mod models;
mod store;
mod utils;

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
        ("login", Some(login_matches)) => login::execute(&login_matches)?,
        ("logout", Some(logout_matches)) => logout::execute(&logout_matches)?,
        ("sync", Some(sync_matches)) => sync::execute(&sync_matches)?,
        ("list", Some(list_matches)) => list::execute(&list_matches)?,
        _ => {
            match (
                matches.value_of("primary_key"),
                matches.value_of("secondary_key"),
            ) {
                (Some(primary_key), Some(secondary_key)) => {
                    ShortcutManager::open_secondary(primary_key, secondary_key);
                }
                (Some(primary_key), None) => {
                    ShortcutManager::open_primary(primary_key);
                }
                (_, _) => {
                    // No args or subcommand provided, print the long help message.
                    cli::build_cli().print_long_help()?;
                }
            }
        }
    };

    Ok(())
}

use clap::ArgMatches;
use failure::Error;

use commands::{ login, logout, sync, list };

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

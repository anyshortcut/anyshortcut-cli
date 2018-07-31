#[macro_use]
extern crate clap;
extern crate curl;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate open;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use clap::{App, AppSettings, Arg, SubCommand};
use std::process;

mod api;
mod constants;
mod cli;
mod commands;
mod http;
mod utils;
mod store;
mod models;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::NextLineHelp)
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::VersionlessSubcommands)
        .max_term_width(90)
        .help_message("Print this help message.")
        .version_message("Show version information.")
        .args(&[
            Arg::with_name("primary_key")
                .value_name("PRIMARY_KEY | COMPOUND_KEY")
                .help("Using primary shortcut key (A~Z|0~9) or compound shortcut key (AA~ZZ) to open the url.")
                .index(1)
                .validator(cli::validate_primary_key),
            Arg::with_name("secondary_key")
                .value_name("SECONDARY_KEY")
                .help("Use secondary shortcut key (A~Z|0~9) to open the url.")
                .index(2)
                .validator(cli::validate_secondary_key),
        ])
        .subcommand(
            SubCommand::with_name("login")
                .about("Login with the token")
                .arg(
                    Arg::with_name("token")
                        .value_name("TOKEN")
                        .help("[TOKEN] obtained from user dashboard.")
                        .takes_value(true)
                        .empty_values(false)),
        )
        .subcommand(
            SubCommand::with_name("sync")
                .about("Sync all shortcuts after login."),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List shortcuts.")
                .arg(
                    Arg::with_name("primary")
                        .long("primary")
                        .short("p")
                        .conflicts_with_all(&["secondary", "compound"])
                        .help("List all primary shortcuts."),
                )
                .arg(
                    Arg::with_name("secondary")
                        .long("secondary")
                        .short("s")
                        .help("List all secondary shortcuts."),
                )
                .arg(
                    Arg::with_name("compound")
                        .long("compound")
                        .short("c")
                        .help("List all compound shortcuts."),
                ),
        )
        .subcommand(
            SubCommand::with_name("logout")
                .about("Logout and clean local data."),
        )
        .get_matches();

    match cli::handle_matches(&matches) {
        Ok(()) => process::exit(0),
        Err(error) => {
            println!("Error: {}", error);
            process::exit(1);
        }
    };
}

#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

mod cli;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .global_setting(AppSettings::NextLineHelp)
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::VersionlessSubcommands)
        .max_term_width(90)
        .help_message("Print this help message.")
        .version_message("Show version information.")
        .args(&[
            Arg::with_name("PRIMARY_KEY | COMPOUND_KEY")
                .help("Using primary shortcut key (A~Z|0~9) or compound shortcut key (AA~ZZ) to open the url.")
                .index(1),
            Arg::with_name("SECONDARY_KEY")
                .help("Use secondary shortcut key (A~Z|0~9) to open the url.")
                .index(2),
        ])
        .subcommand(
            SubCommand::with_name("login")
                .about("Login with the token")
                .arg(
                    Arg::with_name("TOKEN")
                        .help("[TOKEN] obtained from user dashboard.")
                        .empty_values(false)),
        )
        .subcommand(
            SubCommand::with_name("sync")
                .about("Sync all shortcuts after login."),
        )
        .subcommand(
            SubCommand::with_name("check")
                .about("Check a shortcut detail."),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List shortcuts.")
                .arg(
                    Arg::with_name("primary")
                        .long("primary")
                        .short("p")
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

    cli::handle_matches(&matches);
}

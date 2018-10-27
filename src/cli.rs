use clap::{App, AppSettings, Arg, SubCommand};
use clap::{crate_description, crate_name, crate_version};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::VersionlessSubcommands)
        .max_term_width(100)
        .help_message("Print this help message.")
        .version_message("Show version information.")
        .args(&[
            Arg::with_name("primary_key")
                .value_name("PRIMARY_KEY | COMPOUND_KEY")
                .help("Using primary shortcut key (A~Z|0~9) or compound shortcut key (AA~ZZ) to open the url.")
                .index(1)
                .validator(validate_primary_key),
            Arg::with_name("secondary_key")
                .value_name("SECONDARY_KEY")
                .help("Use secondary shortcut key (A~Z|0~9) to open the url.")
                .index(2)
                .validator(validate_secondary_key),
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
                        .conflicts_with_all(&["primary", "compound"])
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
}

/// Validate primary key format.
/// Including one-letter primary key and two-letters compound key.
fn validate_primary_key(key: String) -> Result<(), String> {
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
fn validate_secondary_key(key: String) -> Result<(), String> {
    if key.len() == 1 && key.chars().all(|c| c.is_ascii_alphanumeric()) {
        Ok(())
    } else {
        Err(String::from("Invalid secondary key"))
    }
}

#[cfg(test)]
mod tests {
    use clap::ErrorKind;
    use itertools::Itertools;
    use super::*;

    #[test]
    fn test_args_conflict() {
        for pair in vec!["-p", "-c", "-s"].iter().combinations(2) {
            let mut args: Vec<&str> = vec![crate_name!(), "list"];
            args.extend(pair);
            let res = build_cli().get_matches_from_safe(args);

            assert_eq!(res.unwrap_err().kind, ErrorKind::ArgumentConflict);
        }
    }

    #[test]
    fn test_args_validator() {
        let args = vec![crate_name!(), "a", "a"];
        let res = build_cli().get_matches_from_safe(args);
        assert!(res.is_ok());

        let args = vec![crate_name!(), "aaaa"];
        let res = build_cli().get_matches_from_safe(args);
        assert!(res.is_err());

        let args = vec![crate_name!(), "a*"];
        let res = build_cli().get_matches_from_safe(args);
        assert!(res.is_err());
    }
}


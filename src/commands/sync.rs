use ansi_term::Color::Red;
use clap::ArgMatches;

use crate::models::Meta;

pub fn execute(_: &ArgMatches) -> anyhow::Result<()> {
    if Meta::has_token() {
        super::sync_all_shortcuts();
    } else {
        println!(
            "{}",
            Red.paint("Can't sync data, you are not in login state. Please run login first.")
        );
    }

    Ok(())
}

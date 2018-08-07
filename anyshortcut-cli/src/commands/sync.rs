use clap::ArgMatches;
use failure::Error;
use models::Meta;

pub fn execute(_: &ArgMatches) -> Result<(), Error> {
    if Meta::has_token() {
        super::sync_all_shortcuts();
    } else {
        println!("Can't sync data, you are not in login state. Please run login first.");
    }

    Ok(())
}
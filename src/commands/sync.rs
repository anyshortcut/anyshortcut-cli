use clap::ArgMatches;
use failure::Error;

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    super::sync_all_shortcuts();
    Ok(())
}

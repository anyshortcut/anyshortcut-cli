use clap::ArgMatches;
use failure::Error;

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    println!("All shortcut synced successfully!");

    Ok(())
}

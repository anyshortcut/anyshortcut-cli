use clap::ArgMatches;
use failure::Error;
use models::*;
use store::Storage;

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    Meta::clear()?;
    PrimaryShortcutVec::clear()?;
    SecondaryShortcutMap::clear()?;
    println!("You are logout successfully.");

    Ok(())
}

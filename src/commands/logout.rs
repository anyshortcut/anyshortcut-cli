use clap::ArgMatches;
use failure::Error;

use crate::models::*;
use crate::store::Storage;

pub fn execute(_: &ArgMatches) -> Result<(), Error> {
    Meta::clear()?;
    PrimaryShortcutVec::clear()?;
    SecondaryShortcutMap::clear()?;
    println!("You are logout successfully.");

    Ok(())
}

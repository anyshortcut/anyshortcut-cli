use clap::ArgMatches;

use crate::models::*;
use crate::store::Storage;

pub fn execute(_: &ArgMatches) -> anyhow::Result<()> {
    Meta::clear()?;
    PrimaryShortcutVec::clear()?;
    SecondaryShortcutMap::clear()?;
    println!("You are logout successfully.");

    Ok(())
}

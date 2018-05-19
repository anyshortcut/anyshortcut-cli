use clap::ArgMatches;
use failure::Error;

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    println!("You are logout successfully.");
    
    Ok(())
}

use clap::ArgMatches;
use failure::Error;

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    println!("list:{:?}", matches);
    
    if matches.is_present("primary") {
        println!("Print all primary shortcuts");
    } else if matches.is_present("secondary") {
        println!("Print all secondary shortcuts");
    } else if matches.is_present("compound") {
        println!("Print all compound shortcuts");
    }

    Ok(())
}

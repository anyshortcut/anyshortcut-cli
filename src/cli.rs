use clap::ArgMatches;

pub fn handle_matches(matches: &ArgMatches) {
    println!("{:?}", &matches);
}
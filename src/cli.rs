use clap::ArgMatches;

pub fn handle_matches(matches: &ArgMatches) {
    println!("{:?}", &matches);

    match matches.subcommand() {
        ("login", Some(login_matches)) => {
            println!("login:{:?}", login_matches.value_of("token"));
        }
        ("logout", Some(logout_matches)) => {
            println!("logout:{:?}", logout_matches);
        }
        ("list", Some(logout_matches)) => {
            println!("list:{:?}", logout_matches);
            if logout_matches.is_present("primary") {
                println!("Print all primary shortcuts");
            } else if logout_matches.is_present("secondary") {
                println!("Print all secondary shortcuts");
            } else if logout_matches.is_present("compound") {
                println!("Print all compound shortcuts");
            }
        }
        _ => {
            if matches.is_present("primary_key") {
                if matches.is_present("secondary_key") {
                    println!(
                        "secondary{:?} {:?}",
                        matches.value_of("primary_key"),
                        matches.value_of("secondary_key")
                    );
                } else {
                    println!("primary:{:?}", matches.value_of("primary_key"));
                }
            }
        }
    };
}

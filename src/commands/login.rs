use clap::ArgMatches;
use open;
use failure::Error;

use utils::ui;

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    println!("This helps you signing in your anyshortcut-cli with an authentication token.");
    println!("If you do not yet have a token ready we can bring up a browser for you");
    println!("to create a token now.");
    println!();
    println!("login:{:?}", matches.value_of("token"));

    if ui::prompt_to_continue("Open browser now?")? {
        let url = "https://anyshortcut.com/account";
        if open::that(url).is_err(){
            println!("Cannot open browser. Please manually go to {}", &url);
        }
    }

    let mut token;
    loop {
        token = ui::prompt("Enter your token:")?;
        println!("Valid token for user");
    }

    Ok(())
}


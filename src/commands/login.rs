use api::Api;
use clap::ArgMatches;
use failure::Error;
use open;
use utils::ui;

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    println!("This helps you signing in your anyshortcut-cli with an authentication token.");
    println!("If you do not yet have a token ready we can bring up a browser for you");
    println!("to create a token now.");
    println!();
    println!("login:{:?}", matches.value_of("token"));

    if ui::prompt_to_continue("Open browser now?")? {
        let url = "https://anyshortcut.com/account";
        if open::that(url).is_err() {
            println!("Cannot open browser. Please manually go to {}", &url);
        }
    }

    let mut access_token;
    loop {
        access_token = ui::prompt("Enter your token:")?;

        match Api::get_current().login_with_access_token(&access_token) {
            Ok(response) => {
                println!("Valid access token.");
                println!("{:?}", response);
                break;
            }
            Err(error) => {
                println!("Invalid access token: {}", error);
            }
        }
    }

    Ok(())
}


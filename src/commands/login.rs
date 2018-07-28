use api::Api;
use clap::ArgMatches;
use failure::Error;
use models::Meta;
use open;
use store::Storage;
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

        let api = Api::get_current();
        match api.login_with_access_token(&access_token) {
            Ok(_) => {
                println!("Valid access token.");
                Meta { token: access_token }.persist()
                    .unwrap_or_else(|error| println!("{}", error));

                println!("Syncing your shortcut data...");
                match api.get_all_shortcuts() {
                    Ok(response) => {
                        println!("{:?}", response["primary"]);
                    }
                    Err(error) => println!("{}", error)
                }
                break;
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }

    Ok(())
}


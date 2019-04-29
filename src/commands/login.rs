use crate::api::Api;
use crate::models::Meta;
use crate::store::Storage;
use crate::utils::ui;
use clap::ArgMatches;
use failure::Error;
use open;

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    if let Some(access_token) = matches.value_of("token") {
        return handle_login(access_token);
    }

    println!("This helps you signing in your anyshortcut-cli with an authentication token.");
    println!("If you do not yet have a token ready we can bring up a browser for you");
    println!("to create a token now.");
    println!();

    if ui::prompt_to_continue("Open browser now?")? {
        let url = "https://anyshortcut.com/account";
        if open::that(url).is_err() {
            println!("Cannot open browser. Please manually go to {}", &url);
        }
    }

    while let Err(error) = handle_login(&ui::prompt("Enter your token:")?) {
        println!("{}", error);
    }

    Ok(())
}

fn handle_login(access_token: &str) -> Result<(), Error> {
    let api = Api::get_current();
    api.login_with_access_token(&access_token).and_then(|_| {
        println!("Valid access token.");
        Meta {
            token: access_token.to_string(),
        }
        .persist()
        .unwrap_or_else(|error| println!("{}", error));

        super::sync_all_shortcuts();
        Ok(())
    })
}

use api::Api;
use constants::DIRECTORY_NAME;
use store::Storage;

pub mod login;
pub mod logout;
pub mod sync;
pub mod list;

/// Sync all shortcuts
pub fn sync_all_shortcuts() {
    println!();
    println!("Syncing your shortcut data...");
    match Api::get_current().get_all_shortcuts() {
        Ok(response) => {
            response.primary.persist().unwrap_or_else(|error| println!("{}", error));
            response.secondary.persist().unwrap_or_else(|error| println!("{}", error));

            println!("Shortcuts synced success!");
            println!("Primary shortcut number: {}, secondary shortcut number: {}",
                     response.primary.len(), response.secondary.len());
            println!();
            println!("All your data stored at ~/{} directory.", DIRECTORY_NAME);
        }
        Err(error) => println!("{}", error)
    }
}

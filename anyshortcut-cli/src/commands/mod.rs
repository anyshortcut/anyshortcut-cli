use ansi_term::Color::{Cyan, Green};
use api::Api;
use store::{self, Storage};

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

            println!();
            println!("{}", Green.paint("Shortcuts synced success!"));
            println!("Primary shortcut number: {}",
                     Cyan.paint(response.primary.len().to_string()));
            println!("Secondary shortcut number: {}",
                     Cyan.paint(response.secondary.values()
                         .fold(0, |acc, shortcuts| acc + shortcuts.len())
                         .to_string()
                     ));
            println!();

            // Get the store directory PathBuf object.
            let dir = store::get_store_directory().unwrap();
            println!("All your data stored at {} directory.",
                     Cyan.paint(format!("{}", dir.display())));
        }
        Err(error) => println!("{}", error)
    }
}

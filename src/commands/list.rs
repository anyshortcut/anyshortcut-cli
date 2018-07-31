use clap::ArgMatches;
use failure::Error;
use models::ShortcutManager;

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    println!("list:{:?}", matches);

    if matches.is_present("primary") {
        if let Some(shortcuts) = ShortcutManager::get_primary_shortcuts() {
            shortcuts.iter().for_each(|shortcut| shortcut.pretty_print());

            println!();
            println!("Total primary shortcut number: {}", shortcuts.iter().count());
            println!("Total primary shortcut open times: {}",
                     shortcuts.iter().fold(0, |acc, shortcut| acc + shortcut.open_times));
        } else {
            println!("No primary shortcut found");
        };
    } else if matches.is_present("secondary") {
        println!("Print all secondary shortcuts");
    } else if matches.is_present("compound") {
        if let Some(shortcuts) = ShortcutManager::get_compound_shortcuts() {
            shortcuts.iter().for_each(|shortcut| shortcut.pretty_print());

            println!();
            println!("Total compound shortcut number: {}", shortcuts.iter().count());
            println!("Total compound shortcut open times: {}",
                     shortcuts.iter().fold(0, |acc, shortcut| acc + shortcut.open_times));
        } else {
            println!("No compound shortcut found.");
        }
    }

    Ok(())
}

use clap::ArgMatches;
use failure::Error;
use models::ShortcutManager;

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    if matches.is_present("primary") {
        if let Some(shortcuts) = ShortcutManager::get_primary_shortcuts() {
            shortcuts.iter().for_each(|shortcut| shortcut.pretty_print());

            println!();
            println!("Total primary shortcut number: {}", shortcuts.len());
            println!("Total primary shortcut open times: {}",
                     shortcuts.iter().fold(0, |acc, shortcut| acc + shortcut.open_times));
        } else {
            println!("No primary shortcut found");
        };
    } else if matches.is_present("secondary") {
        if let Some(domain_shortcut_map) = ShortcutManager::get_secondary_shortcuts() {
            let mut total_number = 0;
            let mut total_open_times = 0;
            for (domain, shortcuts) in domain_shortcut_map.iter() {
                println!();
                println!("Domain: {}", domain);

                shortcuts.iter().for_each(|shortcut| shortcut.pretty_print());
                total_number += shortcuts.len();
                total_open_times += shortcuts.iter()
                    .fold(0, |acc, shortcut| acc + shortcut.open_times);
            }

            println!();
            println!("Total domain number: {}", domain_shortcut_map.len());
            println!("Total secondary shortcut number: {}", total_number);
            println!("Total secondary shortcut open times: {}", total_open_times);
        } else {
            println!("No secondary shortcut found.");
        }
    } else if matches.is_present("compound") {
        if let Some(shortcuts) = ShortcutManager::get_compound_shortcuts() {
            shortcuts.iter().for_each(|shortcut| shortcut.pretty_print());

            println!();
            println!("Total compound shortcut number: {}", shortcuts.len());
            println!("Total compound shortcut open times: {}",
                     shortcuts.iter().fold(0, |acc, shortcut| acc + shortcut.open_times));
        } else {
            println!("No compound shortcut found.");
        }
    } else {
        println!("{}", matches.usage());
        println!("For detail usage, please run with -h or --help.")
    }

    Ok(())
}

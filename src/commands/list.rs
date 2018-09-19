use ansi_term::Color::{Red, Cyan};
use clap::ArgMatches;
use failure::Error;
use models::ShortcutManager;

pub fn execute(matches: &ArgMatches) -> Result<(), Error> {
    if matches.is_present("primary") {
        if let Some(shortcuts) = ShortcutManager::get_primary_shortcuts() {
            shortcuts.iter().for_each(|shortcut| shortcut.pretty_print());

            println!();
            println!("Total primary shortcut number: {}",
                     Cyan.paint(shortcuts.len().to_string()));
            println!("Total primary shortcut open times: {}",
                     Cyan.paint(shortcuts.iter()
                         .fold(0, |acc, shortcut| acc + shortcut.open_times)
                         .to_string()
                     ));
        } else {
            println!("{}", Red.paint("No primary shortcut found"));
        };
    } else if matches.is_present("secondary") {
        if let Some(domain_shortcut_map) = ShortcutManager::get_secondary_shortcuts() {
            let mut total_number = 0;
            let mut total_open_times = 0;
            for (domain, shortcuts) in domain_shortcut_map.iter() {
                println!();
                println!("[{}]", Cyan.bold().paint(domain));

                shortcuts.iter().for_each(|shortcut| shortcut.pretty_print());
                total_number += shortcuts.len();
                total_open_times += shortcuts.iter()
                    .fold(0, |acc, shortcut| acc + shortcut.open_times);
            }

            println!();
            println!("Total domain number: {}",
                     Cyan.paint(domain_shortcut_map.len().to_string()));
            println!("Total secondary shortcut number: {}",
                     Cyan.paint(total_number.to_string()));
            println!("Total secondary shortcut open times: {}",
                     Cyan.paint(total_open_times.to_string()));
        } else {
            println!("{}", Red.paint("No secondary shortcut found."));
        }
    } else if matches.is_present("compound") {
        if let Some(shortcuts) = ShortcutManager::get_compound_shortcuts() {
            shortcuts.iter().for_each(|shortcut| shortcut.pretty_print());

            println!();
            println!("Total compound shortcut number: {}",
                     Cyan.paint(shortcuts.len().to_string()));
            println!("Total compound shortcut open times: {}",
                     Cyan.paint(shortcuts.iter()
                         .fold(0, |acc, shortcut| acc + shortcut.open_times)
                         .to_string()
                     ));
        } else {
            println!("{}", Red.paint("No compound shortcut found."));
        }
    } else {
        println!("{}", matches.usage());
        println!("For detail usage, please run with -h or --help.")
    }

    Ok(())
}

extern crate clap;

use clap::Shell;
use std::env;

include!("src/cli.rs");

pub fn main() {
    let mut app = build_cli();
    // Generate shell completion files.
    for shell_name in vec![Shell::Bash, Shell::Zsh, Shell::PowerShell] {
        app.gen_completions(crate_name!(), shell_name, env::var_os("OUT_DIR").unwrap());
    }
}
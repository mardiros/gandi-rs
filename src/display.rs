//! Display options.
use clap::{App, Arg, ArgMatches};
use colored::*;
use std::convert::From;

/// Output format
#[derive(PartialEq)]
pub enum Format {
    JSON,
    TOML,
    YAML,
    HUMAN,
}

/// Retrieve the format from the clap subcommand arguments
impl<'a> From<&'a ArgMatches<'a>> for Format {
    fn from(params: &ArgMatches<'a>) -> Self {
        let mut format = Format::HUMAN;
        if params.is_present("JSON") {
            format = Format::JSON;
        } else if params.is_present("TOML") {
            format = Format::TOML;
        } else if params.is_present("YAML") {
            format = Format::YAML;
        }
        format
    }
}

/// Create the clap subcommand with its arguments.
pub fn add_subcommand_options<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("JSON")
            .long("json")
            .help("Display result in json"),
    )
    .arg(
        Arg::with_name("TOML")
            .long("toml")
            .conflicts_with("JSON")
            .help("Display result in toml"),
    )
    .arg(
        Arg::with_name("YAML")
            .long("yaml")
            .conflicts_with("JSON")
            .conflicts_with("TOML")
            .help("Display result in yaml"),
    )
}

/// Helper to print line with color in the human format
pub fn print_info(key: &str, val: &str) {
    println!("{}: {}", key.bright_blue(), val.green());
}

/// Helper to print line with color in the human format
pub fn print_flag(key: &str, val: bool) {
    if val {
        println!("{}: {}", key.bright_blue(), "active".bright_green());
    } else {
        println!("{}: {}", key.bright_blue(), "inactive".red());
    }
}

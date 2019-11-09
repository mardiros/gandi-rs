//! Display options.
use colored::*;
use std::convert::From;
use clap::{App, Arg, ArgMatches};

/// Output format
pub enum Format {
    JSON,
    YAML,
    HUMAN,
}

/// Retrieve the format from the clap subcommand arguments
impl<'a> From<&'a ArgMatches<'a>> for Format {
    fn from(params: &ArgMatches<'a>) -> Self {
        let mut format = Format::HUMAN;
        if params.is_present("JSON") {
            format = Format::JSON;
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
        Arg::with_name("YAML")
            .long("yaml")
            .conflicts_with("JSON")
            .help("Display result in yaml"),
    )
}

/// Helper to print line with color in the human format
pub fn print_info(key: &str, val: &str) {
    println!("{}: {}", key.bright_blue(), val.green());
}

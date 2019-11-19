//! Display options for domain names.

use clap::{App, Arg};

/// Create the clap subcommand with its arguments.
pub fn add_fqdn_options<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("FQDN")
            .index(1)
            .required(true)
            .help("domain name to query"),
    )
}

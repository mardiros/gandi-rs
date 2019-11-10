//! # Alternative Gandi ClI in rust

use clap::{App, SubCommand};
use log::debug;
use pretty_env_logger;

/// api module
mod api;
/// defined constants
mod constants;
/// output options
mod display;
/// error and result wrapping
mod errors;
/// serde helpers
mod formatter;
/// pagination CLI arguments
mod pagination;
/// sharing_id parameter
mod sharing_id;
/// http user agent helpers
mod user_agent;

use self::errors::GandiResult;
use api::domain_check;
use api::domain_list;
use api::user_info;

/// Parse Command line and run appropriate command.
fn run() -> GandiResult<()> {
    let matches = App::new(constants::NAME)
        .version(constants::VERSION)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("check")
                .about("Check for domain availability")
                .subcommand(domain_check::subcommand()),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Used to retrieve informations")
                .subcommand(user_info::subcommand()),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Used to retrieve informations")
                .subcommand(domain_list::subcommand()),
        )
        .get_matches();

    domain_check::handle(&matches)?;
    domain_list::handle(&matches)?;
    user_info::handle(&matches)?;

    Ok(())
}

/// Entry point of the program.
/// The command will call the run function and set an exit code to 1
/// in case an error happens.
fn main() {
    pretty_env_logger::init();
    debug!("Starting gandi cli");
    match run() {
        Ok(()) => {
            debug!("Command gandi ended succesfully");
        }
        Err(err) => {
            let _ = eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

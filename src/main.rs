
//! # Alternative Gandi ClI in rust

use clap::{App,SubCommand};
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
/// http user agent helpers
mod user_agent;

use self::errors::GandiResult;
use api::user_info;

/// Parse Command line and run appropriate command.
fn run() -> GandiResult<()> {
    let matches = App::new(constants::NAME)
        .version(constants::VERSION)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("get")
                .about("Used to retrieve informations")
                .subcommand(user_info::subcommand()),
        )
        .get_matches();

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

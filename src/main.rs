//! # Alternative Gandi ClI in rust

use clap::{App, Arg, SubCommand};
use log::debug;
use pretty_env_logger;

/// api module
mod api;
/// defined constants
mod constants;
/// CLI configuration
mod config;
/// output options
mod display;
/// error and result wrapping
mod errors;
/// params in the CLI that are send to ReqwestBuilder
mod filter;
/// serde helpers
mod formatter;

use errors::GandiResult;
use api::domain_check;
use api::domain_list;
use api::domain_show;
use api::organization_list;
use api::user_info;
use config::Configuration;


/// Parse Command line and run appropriate command.
fn run() -> GandiResult<()> {
    let matches = App::new(constants::NAME)
        .version(constants::VERSION)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("CONFIG")
                .short("c")
                .long("config")
                .default_value("")
                .takes_value(true)
                .help("Number of element per page"),
        )
        .subcommand(
            SubCommand::with_name("check")
                .about("Check for domain availability")
                .subcommand(domain_check::subcommand()),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Used to retrieve informations")
                .subcommand(domain_show::subcommand())
                .subcommand(user_info::subcommand()),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Used to retrieve informations")
                .subcommand(domain_list::subcommand())
                .subcommand(organization_list::subcommand()),
        )
        .get_matches();

    let config = Configuration::from(&matches);
    domain_check::handle(&config, &matches)?;
    domain_show::handle(&config, &matches)?;
    domain_list::handle(&config, &matches)?;
    organization_list::handle(&config, &matches)?;
    user_info::handle(&config, &matches)?;

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

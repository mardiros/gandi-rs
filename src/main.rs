//! # Alternative Gandi ClI in rust

use clap::{App, Arg, SubCommand};
use log::debug;
use pretty_env_logger;

/// api module
mod api;
/// CLI subcommand handler
mod command_handler;
/// CLI configuration
mod config;
/// defined constants
mod constants;
/// output options
mod display;
/// error and result wrapping
mod errors;
/// params in the CLI that are send to ReqwestBuilder
mod filter;
/// serde helpers
mod formatter;

use api::domain_check::DomainCheckCommand;
use api::domain_list::DomainListCommand;
use api::domain_show::DomainShowCommand;
use api::organization_list;
use api::user_info::UserInfoCommand;
use command_handler::GandiSubCommandHandler;
use config::Configuration;
use errors::GandiResult;

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
                .subcommand(DomainCheckCommand::subcommand()),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Used to retrieve informations")
                .subcommand(DomainShowCommand::subcommand())
                .subcommand(UserInfoCommand::subcommand()),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Used to retrieve informations")
                .subcommand(DomainListCommand::subcommand())
                .subcommand(organization_list::subcommand()),
        )
        .get_matches();

    let config = Configuration::from(&matches);
    DomainCheckCommand::handle(&config, &matches)?;
    DomainShowCommand::handle(&config, &matches)?;
    DomainListCommand::handle(&config, &matches)?;
    organization_list::handle(&config, &matches)?;
    UserInfoCommand::handle(&config, &matches)?;

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

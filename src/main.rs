//! # Alternative Gandi ClI in rust

use clap::{App, Arg, SubCommand};
use log::debug;
use pretty_env_logger;

/// api module
mod api;
/// Common params in the CLI that are repetitive
mod args;
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
/// serde helpers
mod formatter;

use api::dns::list::list_dns_subcommand;
use api::dns::list_records::DnsRecordsListCommand;
use api::dns::list_snapshots::DnsSnapshotsListCommand;
use api::domain::check::DomainCheckCommand;
use api::domain::list::DomainListCommand;
use api::domain::show::DomainShowCommand;
use api::domain::show_contacts::DomainContactsShowCommand;
use api::domain::show_gluerecords::DomainGlueRecordsShowCommand;
use api::organization_list::OrganizationListCommand;
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
                .takes_value(true)
                .help("Extract Configuration from TOML file"),
        )
        .subcommand(
            SubCommand::with_name("check")
                .about("Check for domain availability")
                .subcommand(DomainCheckCommand::subcommand()),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Used to retrieve informations from one resource in particulary")
                .subcommand(DomainShowCommand::subcommand())
                .subcommand(DomainContactsShowCommand::subcommand())
                .subcommand(DomainGlueRecordsShowCommand::subcommand())
                .subcommand(UserInfoCommand::subcommand()),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Used to list resources")
                .subcommand(DomainListCommand::subcommand())
                .subcommand(OrganizationListCommand::subcommand())
                .subcommand(list_dns_subcommand()),
        )
        .get_matches();

    let config = Configuration::from(&matches);
    DnsRecordsListCommand::handle(&config, &matches)?;
    DnsSnapshotsListCommand::handle(&config, &matches)?;
    DomainCheckCommand::handle(&config, &matches)?;
    DomainShowCommand::handle(&config, &matches)?;
    DomainContactsShowCommand::handle(&config, &matches)?;
    DomainGlueRecordsShowCommand::handle(&config, &matches)?;
    DomainListCommand::handle(&config, &matches)?;
    OrganizationListCommand::handle(&config, &matches)?;
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

use clap::{App, SubCommand};

use super::super::super::command_handler::GandiSubCommandHandler;
use super::list_records::DnsRecordsListCommand;
use super::list_snapshots::DnsSnapshotsListCommand;

pub fn list_dns_subcommand<'a, 'b>() -> App<'a, 'b> {
    let subcommand = SubCommand::with_name("dns");
    subcommand
        .subcommand(DnsRecordsListCommand::subcommand())
        .subcommand(DnsSnapshotsListCommand::subcommand())
}

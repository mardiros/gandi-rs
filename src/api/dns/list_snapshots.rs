//! [dns snapshots list](https://api.gandi.net/docs/livedns/#get-v5-livedns-domains-fqdn-snapshots) route binding
//!
use std::vec::Vec;

use chrono::{DateTime, Utc};
use clap::{App, ArgMatches, SubCommand};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::super::super::args::fqdn::add_fqdn_options;
use super::super::super::command_handler::GandiSubCommandHandler;
use super::super::super::config::Configuration;
use super::super::super::display::{add_subcommand_options, print_info};
use super::super::super::formatter::date_formatter_z;

macro_rules! ROUTE {
    () => {
        "/v5/livedns/domains/{}/snapshots"
    };
}

/// Contact information
#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    /// Identifier of the snapshot
    id: String,
    /// Creation date of the snapshot (UTC)
    #[serde(with = "date_formatter_z")]
    created_at: DateTime<Utc>,
    /// name of the snapshot
    name: String,
    // /// URL of the snapshot
    //snapshot_href: String,
}

const SUB_COMMAND: &'static str = "snapshot";

pub struct DnsSnapshotsListCommand {}

impl GandiSubCommandHandler for DnsSnapshotsListCommand {
    const COMMAND_GROUP: &'static str = "list";
    const COMMAND: &'static str = "dns";
    type Item = Vec<Snapshot>;

    /// Create the route
    fn build_req(config: &Configuration, params: &ArgMatches) -> RequestBuilder {
        let fqdn = params.value_of("FQDN").unwrap().to_string();
        config.build_req(format!(ROUTE!(), fqdn).as_str())
    }

    /// Override it to display extra informations from the response header
    // fn display_human_headers(headers: &HeaderMap) -> GandiResult<()> {
    //     let total_count = headers
    //         .get("Total-Count")
    //         .map(|hdr| hdr.to_str().unwrap())
    //         .unwrap_or("MISSING");
    //     println!("");
    //     print_info("Total Count of domains:", total_count);
    //     Ok(())
    // }

    /// Display the records important data
    fn display_human_result(items: Self::Item) {
        for snapshot in items {
            print!("");
            print_info("Id:", snapshot.id.as_str());
            print_info("Name:", snapshot.name.as_str());
            print_info("Created at:", snapshot.created_at.to_rfc2822().as_str());
        }
    }

    /// Check if the operation in case the matches is processable.
    fn can_handle<'a>(matches: &'a ArgMatches) -> Option<&'a ArgMatches<'a>> {
        if matches.is_present(Self::COMMAND_GROUP) {
            let subcommand = matches.subcommand_matches(Self::COMMAND_GROUP).unwrap();
            if subcommand.is_present(Self::COMMAND) {
                let subcommand = subcommand.subcommand_matches(Self::COMMAND).unwrap();
                if subcommand.is_present(SUB_COMMAND) {
                    let params = subcommand.subcommand_matches(SUB_COMMAND).unwrap();
                    return Some(params);
                }
            }
        }
        None
    }

    /// Create the clap subcommand with its arguments.
    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        let subcommand = SubCommand::with_name(SUB_COMMAND);
        //let subcommand = add_pagination_options(subcommand);
        //let subcommand = add_sharing_id_options(subcommand);
        let subcommand = add_fqdn_options(subcommand);
        add_subcommand_options(subcommand)
    }
}

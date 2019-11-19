//! [dns records list](https://api.gandi.net/docs/livedns/#get-v5-livedns-domains-fqdn-records) route binding
//!
use std::vec::Vec;

use clap::{App, ArgMatches, SubCommand};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::super::super::args::fqdn::add_fqdn_options;
use super::super::super::command_handler::GandiSubCommandHandler;
use super::super::super::config::Configuration;
use super::super::super::display::{add_subcommand_options, print_line};

macro_rules! ROUTE {
    () => {
        "/v5/livedns/domains/{}/records"
    };
}

/// Contact information
#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    /// URL for the record
    rrset_href: String,
    /// Time to live of the record
    rrset_ttl: usize,
    /// Name of the record
    rrset_name: String,
    /// One of: "A", "AAAA", "ALIAS", "CAA", "CDS", "CNAME", "DNAME", "DS", "KEY", "LOC", "MX", "NS", "OPENPGPKEY", "PTR", "SPF", "SRV", "SSHFP", "TLSA", "TXT", "WKS"
    rrset_type: String,
    /// A list of values for this record
    rrset_values: Vec<String>,
}

const SUB_COMMAND: &'static str = "records";

pub struct DnsRecordsListCommand {}

impl GandiSubCommandHandler for DnsRecordsListCommand {
    const COMMAND_GROUP: &'static str = "list";
    const COMMAND: &'static str = "dns";
    type Item = Vec<Record>;

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
        for record in items {
            for val in record.rrset_values {
                print_line(
                    format!(
                        "{} {} IN {} {}",
                        record.rrset_name.as_str(),
                        record.rrset_ttl,
                        record.rrset_type.as_str(),
                        val
                    )
                    .as_str(),
                );

            } 
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

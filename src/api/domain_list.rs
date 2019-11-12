//! [List domains](https://api.gandi.net/docs/domains/#get-v5-domain-domains) route binding
use std::vec::Vec;

use chrono::{DateTime, Utc};
use clap::{App, ArgMatches, SubCommand};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::super::config::Configuration;
use super::super::command_handler::GandiSubCommandHandler;
use super::super::display::{add_subcommand_options, print_flag, print_info};
use super::super::filter::pagination::{
    add_subcommand_options as add_pagination_options, Pagination,
};
use super::super::filter::sharing_id::{
    add_subcommand_options as add_sharing_id_options, SharingSpace,
};
use super::super::formatter::date_formatter_z;
use super::super::formatter::optional_date_formatter_z;

pub const ROUTE: &str = "/v5/domain/domains";
pub const COMMAND_GROUP: &str = "list";
pub const COMMAND: &str = "domains";

/// Name Server Information
#[derive(Debug, Serialize, Deserialize)]
struct NameServer {
    /// Label of the nameserver (abc, livedns or other)
    current: String,
    /// In the doc, but always null
    #[serde(skip_serializing_if = "Option::is_none")]
    hosts: Option<Vec<String>>,
}

/// Domain's life cycle dates.
#[derive(Debug, Serialize, Deserialize)]
struct Dates {
    #[serde(with = "date_formatter_z")]
    registry_created_at: DateTime<Utc>,
    #[serde(with = "date_formatter_z")]
    updated_at: DateTime<Utc>,
    #[serde(
        default,
        with = "optional_date_formatter_z",
        skip_serializing_if = "Option::is_none"
    )]
    authinfo_expires_at: Option<DateTime<Utc>>,
    #[serde(
        default,
        with = "optional_date_formatter_z",
        skip_serializing_if = "Option::is_none"
    )]
    created_at: Option<DateTime<Utc>>,
    #[serde(
        default,
        with = "optional_date_formatter_z",
        skip_serializing_if = "Option::is_none"
    )]
    deletes_at: Option<DateTime<Utc>>,
    #[serde(
        default,
        with = "optional_date_formatter_z",
        skip_serializing_if = "Option::is_none"
    )]
    hold_begins_at: Option<DateTime<Utc>>,
    #[serde(
        default,
        with = "optional_date_formatter_z",
        skip_serializing_if = "Option::is_none"
    )]
    hold_ends_at: Option<DateTime<Utc>>,
    #[serde(
        default,
        with = "optional_date_formatter_z",
        skip_serializing_if = "Option::is_none"
    )]
    pending_delete_ends_at: Option<DateTime<Utc>>,
    #[serde(
        default,
        with = "optional_date_formatter_z",
        skip_serializing_if = "Option::is_none"
    )]
    registry_ends_at: Option<DateTime<Utc>>,
    #[serde(
        default,
        with = "optional_date_formatter_z",
        skip_serializing_if = "Option::is_none"
    )]
    renew_begins_at: Option<DateTime<Utc>>,
    #[serde(
        default,
        with = "optional_date_formatter_z",
        skip_serializing_if = "Option::is_none"
    )]
    restore_ends_at: Option<DateTime<Utc>>,
}

/// Domain Information Format, returned by the API
#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
    /// the id of the domain
    id: String,

    /// UNDOCUMENTED
    orga_owner: String,
    /// UNDOCUMENTED
    owner: String,

    /// the sharing id of the owner (an organization id)
    sharing_id: Option<String>,

    /// Fully qualified domain name, written in its native alphabet (IDN)
    fqdn: String,
    /// Fully qualified domain name, written in unicode
    fqdn_unicode: String,
    /// flag to renew automatically the domain name before it expires
    autorenew: bool,
    /// the tld of the domain
    tld: String,

    /// tags
    tags: Option<Vec<String>>,

    /// Domain's life cycle dates
    dates: Dates,

    /// flag to renew automatically the domain name before it expires
    nameserver: NameServer,
}

pub struct DomainListCommand {}

impl GandiSubCommandHandler for DomainListCommand {

    type Item = Vec<Domain>;

    /// Create the route
    fn build_req(config: &Configuration, params: &ArgMatches) -> RequestBuilder {
        let pagination = Pagination::from(params);
        let sharing_space = SharingSpace::from(params);
        let req = config.build_req(ROUTE);
        let req = pagination.build_req(req);
        sharing_space.build_req(req)
    }

    /// Display the domain important data
    fn display_human_result(items: Self::Item) {
        //let total_count, domains = items;
        //println!("Total count of domains: {}", total_count);
        for domain in items {
            println!("");
            print_info("fqdn", domain.fqdn_unicode.as_str());
            print_info("id", domain.id.as_str());
            print_info("organization", domain.orga_owner.as_str());
            //print_info("sharing_id", domain.sharing_id.as_str());
            if domain.owner != domain.orga_owner {
                print_info("owner", domain.owner.as_str());
            }
            // print_info("tld", domain.tld.as_str());
            // print_info("nameserver", domain.nameserver.current.as_str());
            print_flag("autorenew", domain.autorenew);
            if let Some(tags) = domain.tags {
                if tags.len() > 0 {
                    print_info("tags", tags.join(" ").as_str());
                }
            }
        }
    }

    /// Create the clap subcommand with its arguments.
    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        let subcommand = SubCommand::with_name(COMMAND);
        let subcommand = add_pagination_options(subcommand);
        let subcommand = add_sharing_id_options(subcommand);
        add_subcommand_options(subcommand)
    }

    /// Process the operation in case the matches is processable.
    fn can_handle<'a>(matches: &'a ArgMatches) -> Option<&'a ArgMatches<'a>> {
        if matches.is_present(COMMAND_GROUP) {
            let subcommand = matches.subcommand_matches(COMMAND_GROUP).unwrap();
            if subcommand.is_present(COMMAND) {
                let params = subcommand.subcommand_matches(COMMAND).unwrap();
                return Some(params);
            }
        }
        None
    }

}

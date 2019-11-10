//! [domains list](https://api.gandi.net/docs/domains/#get-v5-domain-domains) route binding
use std::vec::Vec;

use chrono::{DateTime, Utc};
use clap::{App, ArgMatches, SubCommand};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use toml;

use super::super::formatter::date_formatter_z;
use super::super::formatter::optional_date_formatter_z;
use super::super::display::{add_subcommand_options, print_flag, print_info, Format};
use super::super::errors::{GandiError, GandiResult};
use super::super::pagination::{add_subcommand_options as add_pagination_options, Pagination};
use super::super::user_agent::get_client;

pub const ROUTE: &str = "/v5/domain/domains";
pub const COMMAND_GROUP: &str = "list";
pub const COMMAND: &str = "domains";

/// Name Server Information
#[derive(Debug, Serialize, Deserialize)]
struct NameServer {
    /// Label of the nameserver (abc, livedns or other)
    current: String,
    /// In the doc, but always null
    hosts: Option<Vec<String>>,
}

/// Domain's life cycle dates.
#[derive(Debug, Serialize, Deserialize)]
struct Dates {
    #[serde(with = "date_formatter_z")]
    registry_created_at: DateTime<Utc>,
    #[serde(with = "date_formatter_z")]
    updated_at: DateTime<Utc>,
    #[serde(default, with = "optional_date_formatter_z")]
    authinfo_expires_at: Option<DateTime<Utc>>,
    #[serde(default, with = "optional_date_formatter_z")]
    created_at: Option<DateTime<Utc>>,
    #[serde(default, with = "optional_date_formatter_z")]
    deletes_at: Option<DateTime<Utc>>,
    #[serde(default, with = "optional_date_formatter_z")]
    hold_begins_at: Option<DateTime<Utc>>,
    #[serde(default, with = "optional_date_formatter_z")]
    hold_ends_at: Option<DateTime<Utc>>,
    #[serde(default, with = "optional_date_formatter_z")]
    pending_delete_ends_at: Option<DateTime<Utc>>,
    #[serde(default, with = "optional_date_formatter_z")]
    registry_ends_at: Option<DateTime<Utc>>,
    #[serde(default, with = "optional_date_formatter_z")]
    renew_begins_at: Option<DateTime<Utc>>,
    #[serde(default, with = "optional_date_formatter_z")]
    restore_ends_at: Option<DateTime<Utc>>,
}

/// Domain Information Format, returned by the API
#[derive(Debug, Serialize, Deserialize)]
struct Domain {
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

/// Display the result for human
fn display_result(domains: Vec<Domain>, total_count: &str, format: Format) -> GandiResult<()> {
    match format {
        Format::JSON => {
            let resp = serde_json::to_string(&domains)?;
            println!("{}", resp);
        }
        Format::YAML => {
            let resp = serde_yaml::to_string(&domains)?;
            println!("{}", resp);
        }
        Format::TOML => {
            let resp = toml::to_string(&domains)?;
            println!("{}", resp);
        }
        Format::HUMAN => {
            println!("Total count of domains: {}", total_count);
            for domain in domains {
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
    }
    Ok(())
}

/// Process the http request and display the result.
fn process(pagination: Pagination, format: Format) -> GandiResult<()> {
    let client = get_client(ROUTE)
        .query(&[("page", pagination.page.as_str())])
        .query(&[("per_page", pagination.per_page.as_str())]);
    let mut resp = client.send()?;
    if resp.status().is_success() {
        // println!("{}", resp.text().unwrap_or("".to_string()));
        let domains: Vec<Domain> = resp.json()?;
        let total_count = resp
            .headers()
            .get("Total-Count")
            .map(|hdr| hdr.to_str().unwrap())
            .unwrap_or("MISSING");
        display_result(domains, total_count, format)?;
        Ok(())
    } else {
        Err(GandiError::ReqwestResponseError(
            format!("{}", resp.status()),
            resp.text().unwrap_or("".to_string()),
        ))
    }
}

/// Create the clap subcommand with its arguments.
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    let subcommand = SubCommand::with_name(COMMAND);
    let subcommand = add_pagination_options(subcommand);
    add_subcommand_options(subcommand)
}

/// Process the operation in case the matches is processable.
pub fn handle(matches: &ArgMatches) -> GandiResult<bool> {
    if matches.is_present(COMMAND_GROUP) {
        let subcommand = matches.subcommand_matches(COMMAND_GROUP).unwrap();
        if subcommand.is_present(COMMAND) {
            let params = subcommand.subcommand_matches(COMMAND).unwrap();
            let format = Format::from(params);
            let pagination = Pagination::from(params);
            process(pagination, format)?;
            return Ok(true);
        }
    }
    Ok(false)
}

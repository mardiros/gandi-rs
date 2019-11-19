//! [Show domain information](https://api.gandi.net/docs/domains/#v5-domain-domains-domain) route binding
use std::vec::Vec;

use chrono::{DateTime, Utc};
use clap::{App, ArgMatches, SubCommand};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::super::super::args::fqdn::add_fqdn_options;
use super::super::super::command_handler::GandiSubCommandHandler;
use super::super::super::config::Configuration;
use super::super::super::display::{
    add_subcommand_options, print_flag, print_info, print_list, print_tags,
};
use super::super::super::formatter::date_formatter_z;
use super::super::super::formatter::optional_date_formatter_z;
use super::show_contact::{print_contacts, Contacts, SharingSpace};

macro_rules! ROUTE {
    () => {
        "/v5/domain/domains/{}"
    };
}

/// Autorenew Informations
#[derive(Debug, Serialize, Deserialize)]
struct Autorenew {
    // what is this ?
    href: String,
    // dates ⁠array[ datetime ]
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<Vec<String>>,
    // not explained - should have a duration unit too ?
    duration: usize,
    // use it to disable the autorenew
    enabled: bool,
    /// sharing_id that pay the renew
    org_id: Option<String>,
}

/// Domain's life cycle dates.
#[derive(Debug, Serialize, Deserialize)]
pub struct Dates {
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
    // optional ?
    id: String,
    /// Fully qualified domain name, written in its native alphabet (IDN)
    fqdn: String,
    /// Fully qualified domain name, written in unicode
    fqdn_unicode: String,
    /// the tld of the domain
    tld: String,
    /// information assiociated to the tld of the domain, about lock registry support
    can_tld_lock: bool,
    /// the authinfo code used to transfer out the domain
    authinfo: String,
    /// fqdn of name servers
    #[serde(skip_serializing_if = "Option::is_none")]
    nameservers: Option<Vec<String>>,
    /// List of Gandi services attached to this domain
    #[serde(skip_serializing_if = "Option::is_none")]
    services: Option<Vec<String>>,
    /// list of tags
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,
    /// the organization that own the domain
    sharing_space: SharingSpace,
    // sharing_id: String optional, not sent, but we have the sharing_space here
    /// autorenew informations
    autorenew: Autorenew,
    /// Domain's life cycle dates
    dates: Dates,
    contacts: Contacts,
}

/// Implement the "show domain" subcommand
pub struct DomainShowCommand {}

impl GandiSubCommandHandler for DomainShowCommand {
    const COMMAND_GROUP: &'static str = "show";
    const COMMAND: &'static str = "domain";

    type Item = Domain;

    /// Create the route
    fn build_req(config: &Configuration, params: &ArgMatches) -> RequestBuilder {
        let fqdn = params.value_of("FQDN").unwrap().to_string();
        config.build_req(format!(ROUTE!(), fqdn).as_str())
    }

    /// Display the domain important data
    fn display_human_result(domain: Self::Item) {
        print_info("id", domain.id.as_str());
        print_info("fqdn", domain.fqdn_unicode.as_str());
        print_flag("autorenew", domain.autorenew.enabled);
        print_list("nameservers", &domain.nameservers);
        print_list("services", &domain.services);
        print_contacts(&domain.contacts, Some(&domain.sharing_space));
        print_tags(&domain.tags);
    }

    /// Create the clap subcommand with its arguments.
    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        let subcommand = SubCommand::with_name(Self::COMMAND);
        let subcommand = add_fqdn_options(subcommand);
        add_subcommand_options(subcommand)
    }
}

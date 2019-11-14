//! [Show domain information](https://api.gandi.net/docs/domains/#v5-domain-domains-domain) route binding
use std::collections::HashMap;
use std::vec::Vec;

use chrono::{DateTime, Utc};
use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::super::command_handler::GandiSubCommandHandler;
use super::super::config::Configuration;
use super::super::display::{add_subcommand_options, print_flag, print_info, print_tags, print_list};
use super::super::formatter::date_formatter_z;
use super::super::formatter::optional_date_formatter_z;

pub const COMMAND_GROUP: &str = "show";
pub const COMMAND: &str = "domain";

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

/// Contact information
#[derive(Debug, Serialize, Deserialize)]
struct Contact {
    /// Will be true when the contact used is the same as the owner.
    /// always none for the owner contact, because it does not make sense.
    #[serde(skip_serializing_if = "Option::is_none")]
    same_as_owner: Option<bool>,

    /// 0: person, 1: company, 2: association, 3: public body
    // 4: reseller is bad
    #[serde(rename(deserialize = "type", serialize = "type"))]
    type_: usize,

    /// legal name of the company, association, or public body if the contact type is not 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    orgname: Option<String>,
    given: String,
    family: String,
    streetaddr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    country: String,

    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mail_obfuscated: Option<bool>,
    // why both ?
    /// One of "pending", "done", "failed", "deleted", "none"
    #[serde(skip_serializing_if = "Option::is_none")]
    reachability: Option<String>,
    /// One of "pending", "done", "failed", "deleted", "none"
    #[serde(skip_serializing_if = "Option::is_none")]
    validation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mobile: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data_obfuscated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    extra_parameters: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    siren: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    brand_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jo_announce_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jo_announce_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jo_declaration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jo_publication_date: Option<String>,
    // One: of: "pending", "done", "failed", "deleted", "none"

    // why is there a sharing_id here ?
    #[serde(skip_serializing_if = "Option::is_none")]
    sharing_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Contacts {
    owner: Contact,
    admin: Contact,
    tech: Contact,
    bill: Contact,
}

/// Organization information
#[derive(Debug, Serialize, Deserialize)]
struct SharingSpace {
    /// id that pay the renew
    id: String,
    /// sharing_id that pay the renew
    name: String,
    /// reseller flag organization
    #[serde(skip_serializing_if = "Option::is_none")]
    reseller: Option<bool>,
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


/// Helper to print tags in the human format
fn print_contact(type_: &str, contact: &Contact, sharing_space: Option<&SharingSpace>) {
    let mut contact = if contact.type_ == 0 {
        format!(
            r#""{} {}" <{}>"#,
            contact.given,
            contact.family,
            contact.email
        )
    } else {
        format!(
            r#""{}" <{}>"#,
            contact.orgname.as_ref().map(|orgname| orgname.as_str())
                .unwrap_or("NO ORGNAME SET"),
            contact.email
        )
    };
    if let Some(sharing) = sharing_space {
        contact = format!("{} ({})", contact, sharing.name);
    }
    print_info(type_, contact.as_str());
}


/// Implement the "show domain" subcommand
pub struct DomainShowCommand {}

impl GandiSubCommandHandler for DomainShowCommand {
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
        print_contact("owner", &domain.contacts.owner, Some(&domain.sharing_space));
        if !domain.contacts.admin.same_as_owner.unwrap_or(false) {
            print_contact("admin", &domain.contacts.admin, None);
        }
        if !domain.contacts.tech.same_as_owner.unwrap_or(false) {
            print_contact("tech", &domain.contacts.tech, None);
        }
        if !domain.contacts.bill.same_as_owner.unwrap_or(false) {
            print_contact("bill", &domain.contacts.bill, None);
        }
        print_tags(&domain.tags);
    }

    /// Create the clap subcommand with its arguments.
    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        let subcommand = SubCommand::with_name(COMMAND).arg(
            Arg::with_name("FQDN")
                .index(1)
                .required(true)
                .help("domain name to query"),
        );
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

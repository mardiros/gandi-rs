//! [Show domain information](https://api.gandi.net/docs/domains/#v5-domain-domains-domain) route binding
use std::collections::HashMap;

use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::super::command_handler::GandiSubCommandHandler;
use super::super::config::Configuration;
use super::super::display::{add_subcommand_options, print_info};

pub const COMMAND_GROUP: &str = "show";
pub const COMMAND: &str = "contacts";

macro_rules! ROUTE {
    () => {
        "/v5/domain/domains/{}/contacts"
    };
}

/// Organization information
#[derive(Debug, Serialize, Deserialize)]
pub struct SharingSpace {
    /// id that pay the renew
    id: String,
    /// sharing_id that pay the renew
    name: String,
    /// reseller flag organization
    #[serde(skip_serializing_if = "Option::is_none")]
    reseller: Option<bool>,
}

/// Contact information
#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
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
pub struct Contacts {
    owner: Contact,
    admin: Contact,
    tech: Contact,
    bill: Contact,
}

/// Helper to print tags in the human format
pub fn print_contact(type_: &str, contact: &Contact, sharing_space: Option<&SharingSpace>) {
    let mut contact = if contact.type_ == 0 {
        format!(
            r#""{} {}" <{}>"#,
            contact.given, contact.family, contact.email
        )
    } else {
        format!(
            r#""{}" <{}>"#,
            contact
                .orgname
                .as_ref()
                .map(|orgname| orgname.as_str())
                .unwrap_or("NO ORGNAME SET"),
            contact.email
        )
    };
    if let Some(sharing) = sharing_space {
        contact = format!("{} ({})", contact, sharing.name);
    }
    print_info(type_, contact.as_str());
}

/// Helper to print tags in the human format
pub fn print_contacts(contacts: &Contacts, sharing_space: Option<&SharingSpace>) {
    print_contact("owner", &contacts.owner, sharing_space);
    if !contacts.admin.same_as_owner.unwrap_or(false) {
        print_contact("admin", &contacts.admin, None);
    }
    if !contacts.tech.same_as_owner.unwrap_or(false) {
        print_contact("tech", &contacts.tech, None);
    }
    if !contacts.bill.same_as_owner.unwrap_or(false) {
        print_contact("bill", &contacts.bill, None);
    }

}

/// Implement the "show domain" subcommand
pub struct DomainContactsShowCommand {}

impl GandiSubCommandHandler for DomainContactsShowCommand {
    type Item = Contacts;

    /// Create the route
    fn build_req(config: &Configuration, params: &ArgMatches) -> RequestBuilder {
        let fqdn = params.value_of("FQDN").unwrap().to_string();
        config.build_req(format!(ROUTE!(), fqdn).as_str())
    }

    /// Display the domain contacts important data
    fn display_human_result(contacts: Self::Item) {
        print_contacts(&contacts, None)
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

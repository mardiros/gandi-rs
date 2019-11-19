//! [organizations list](https://api.gandi.net/docs/organization/#get-v5-organization-organizations) route binding
use std::vec::Vec;

use clap::{App, ArgMatches, SubCommand};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::super::args::pagination::{
    add_subcommand_options as add_pagination_options, Pagination,
};
use super::super::args::sharing_id::{
    add_subcommand_options as add_sharing_id_options, SharingSpace,
};
use super::super::command_handler::GandiSubCommandHandler;
use super::super::config::Configuration;
use super::super::display::{add_subcommand_options, print_flag, print_info};

pub const ROUTE: &str = "/v5/organization/organizations";

/// Organization Information Format, returned by the API
#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    /// id of the organizaiton
    id: String,
    /// display name
    name: String,
    /// type of the organization.
    #[serde(rename(deserialize = "type", serialize = "type"))]
    type_: String, // Should not be optional

    /// Flag to indicate the corporate status for the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    corporate: Option<bool>,
    /// Flag to indicate the reseller status for the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    reseller: Option<bool>,
    /// Email address of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    /// first name of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    firstname: Option<String>,
    /// last name of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    lastname: Option<String>,
    /// The company, association, or public body name of the (non-individual) organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    orgname: Option<String>,
    /// Siren number of the (non-individual) organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    siren: Option<String>,
    /// VAT number of the (non-individual) organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    vat_number: Option<String>,
}

/// Implement the "show domain" subcommand
pub struct OrganizationListCommand {}

impl GandiSubCommandHandler for OrganizationListCommand {
    const COMMAND_GROUP: &'static str = "list";
    const COMMAND: &'static str = "organizations";
    type Item = Vec<Organization>;

    /// Create the route
    fn build_req(config: &Configuration, params: &ArgMatches) -> RequestBuilder {
        let pagination = Pagination::from(params);
        let sharing_space = SharingSpace::from(params);
        let req = config.build_req(ROUTE);
        let req = pagination.build_req(req);
        sharing_space.build_req(req)
    }

    /// Display the organizaiton main data
    fn display_human_result(organizations: Self::Item) {
        for organization in organizations {
            println!("");
            print_info("id", organization.id.as_str());
            print_info("type", organization.type_.as_str());
            print_info("name", organization.name.as_str());
            if let Some(orgname) = organization.orgname {
                print_info("orgname", orgname.as_str());
            } else if let (Some(firstname), Some(lastname)) =
                (organization.firstname, organization.lastname)
            {
                print_info("orgname", format!("{} {}", firstname, lastname).as_str());
            }
            if let Some(email) = organization.email {
                print_info("email", email.as_str());
            }
            if let Some(reseller) = organization.reseller {
                if reseller {
                    print_flag("reseller", true);
                }
            }
            if let Some(corporate) = organization.corporate {
                if corporate {
                    print_flag("corporate", true);
                }
            }
        }
    }

    /// Create the clap subcommand with its arguments.
    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        let subcommand = SubCommand::with_name(Self::COMMAND);
        let subcommand = add_pagination_options(subcommand);
        let subcommand = add_sharing_id_options(subcommand);
        add_subcommand_options(subcommand)
    }
}

//! [organizations list](https://api.gandi.net/docs/organization/#get-v5-organization-organizations) route binding
use std::vec::Vec;

use clap::{App, ArgMatches, SubCommand};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use toml;

use super::super::config::Configuration;
use super::super::display::{add_subcommand_options, print_flag, print_info, Format};
use super::super::errors::{GandiError, GandiResult};
use super::super::filter::pagination::{
    add_subcommand_options as add_pagination_options, Pagination,
};
use super::super::filter::sharing_id::{
    add_subcommand_options as add_sharing_id_options, SharingSpace,
};

pub const ROUTE: &str = "/v5/organization/organizations";
pub const COMMAND_GROUP: &str = "list";
pub const COMMAND: &str = "organizations";

/// Organization Information Format, returned by the API
#[derive(Debug, Serialize, Deserialize)]
struct Organization {
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

/// Display the result to stdout
fn display_result(
    organizations: Vec<Organization>,
    total_count: &str,
    format: Format,
) -> GandiResult<()> {
    match format {
        Format::JSON => {
            let resp = serde_json::to_string(&organizations)?;
            println!("{}", resp);
        }
        Format::YAML => {
            let resp = serde_yaml::to_string(&organizations)?;
            println!("{}", resp);
        }
        Format::TOML => {
            let resp = toml::to_string(&organizations)?;
            println!("{}", resp);
        }
        Format::HUMAN => {
            println!("Total count of organizations: {}", total_count);
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
    }
    Ok(())
}

/// Process the http request and display the result.
fn process(
    sharing_space: SharingSpace,
    pagination: Pagination,
    config: &Configuration,
    format: Format,
) -> GandiResult<()> {
    let req = config.build_req(ROUTE);
    let req = pagination.build_req(req);
    let req = sharing_space.build_req(req);
    let mut resp = req.send()?;
    if resp.status().is_success() {
        // println!("{}", resp.text().unwrap_or("".to_string()));
        let organizations: Vec<Organization> = resp.json()?;
        let total_count = resp
            .headers()
            .get("Total-Count")
            .map(|hdr| hdr.to_str().unwrap())
            .unwrap_or("MISSING"); // actually missing
        display_result(organizations, total_count, format)?;
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
    let subcommand = add_sharing_id_options(subcommand);
    add_subcommand_options(subcommand)
}

/// Process the operation in case the matches is processable.
pub fn handle(config: &Configuration, matches: &ArgMatches) -> GandiResult<bool> {
    if matches.is_present(COMMAND_GROUP) {
        let subcommand = matches.subcommand_matches(COMMAND_GROUP).unwrap();
        if subcommand.is_present(COMMAND) {
            let params = subcommand.subcommand_matches(COMMAND).unwrap();
            let format = Format::from(params);
            let pagination = Pagination::from(params);
            let sharing_space = SharingSpace::from(params);
            process(sharing_space, pagination, config, format)?;
            return Ok(true);
        }
    }
    Ok(false)
}

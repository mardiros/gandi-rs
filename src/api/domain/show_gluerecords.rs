//! [Show domain information](https://api.gandi.net/docs/domains/#v5-domain-domains-domain) route binding

use clap::{App, ArgMatches, SubCommand};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::super::super::args::fqdn::add_fqdn_options;
use super::super::super::command_handler::GandiSubCommandHandler;
use super::super::super::config::Configuration;
use super::super::super::display::{add_subcommand_options, print_info, print_list};

macro_rules! ROUTE {
    () => {
        "/v5/domain/domains/{}/hosts"
    };
}

/// Contact information
#[derive(Debug, Serialize, Deserialize)]
pub struct GlueRecord {
    /// Fully qualified domain name, written in its native alphabet (IDN).
    fqdn: String,
    /// Fully qualified domain name, written in unicode.
    fqdn_unicode: String,
    /// Name of this host (FQDN without the domain part).
    name: String,
    /// URL to this host's details.
    href: String,
    /// List of this host's registered IP addresse.
    ips: Vec<String>,
}

/// Implement the "show domain" subcommand
pub struct DomainGlueRecordsShowCommand {}

impl GandiSubCommandHandler for DomainGlueRecordsShowCommand {
    const COMMAND_GROUP: &'static str = "show";
    const COMMAND: &'static str = "glue-records";

    type Item = Vec<GlueRecord>;

    /// Create the route
    fn build_req(config: &Configuration, params: &ArgMatches) -> RequestBuilder {
        let fqdn = params.value_of("FQDN").unwrap().to_string();
        config.build_req(format!(ROUTE!(), fqdn).as_str())
    }

    /// Display the domain contacts important data
    fn display_human_result(glues: Self::Item) {
        for glue in glues {
            print!("");
            print_info("fqdn", glue.fqdn_unicode.as_str());
            print_info("name", glue.name.as_str());
            print_list("ips", &Some(glue.ips));
        }
    }

    /// Create the clap subcommand with its arguments.
    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        let subcommand = SubCommand::with_name(Self::COMMAND);
        let subcommand = add_fqdn_options(subcommand);
        add_subcommand_options(subcommand)
    }
}

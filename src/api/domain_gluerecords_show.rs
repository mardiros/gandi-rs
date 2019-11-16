//! [Show domain information](https://api.gandi.net/docs/domains/#v5-domain-domains-domain) route binding

use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::super::command_handler::GandiSubCommandHandler;
use super::super::config::Configuration;
use super::super::display::{add_subcommand_options, print_info, print_list};

pub const COMMAND_GROUP: &str = "show";
pub const COMMAND: &str = "glue-records";

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

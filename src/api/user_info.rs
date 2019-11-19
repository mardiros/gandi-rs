///! The [user-info](https://api.gandi.net/docs/organization/#get-v5-organization-user-info) route binding
use clap::{App, ArgMatches, SubCommand};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::super::command_handler::GandiSubCommandHandler;
use super::super::config::Configuration;
use super::super::display::{add_subcommand_options, print_info};

/// endpoint of the route.
const ROUTE: &str = "/v5/organization/user-info";

/// User Information format, returned by the API
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    /// the sharing id of the user.
    id: String,
    /// the username of the user.
    username: String,
    /// the email address of the user.
    email: String,
    /// language used by the user.
    lang: String,
    /// the sharing name of the user.
    name: String,
    /// the city name of the address.
    city: Option<String>,
    /// country ISO code of the address.
    country: Option<String>,
    /// fax number.
    fax: Option<String>,
    /// the first name of the user.
    firstname: Option<String>,
    /// the last name of the user.
    lastname: Option<String>,
    /// phone number.
    phone: Option<String>,
    /// state ISO code of the address.
    state: Option<String>,
    /// the street address of the user.
    streetaddr: Option<String>,
    /// additional street address info of the user.
    streetaddr2: Option<String>,
    /// zip code of the address.
    zip: Option<String>,
}

/// Implement the "show domain" subcommand
pub struct UserInfoCommand {}

impl GandiSubCommandHandler for UserInfoCommand {
    type Item = UserInfo;
    /// CLAP first sub command name.
    const COMMAND_GROUP: &'static str = "show";
    /// CLAP second sub command name.
    const COMMAND: &'static str = "user-info";

    /// Create the route
    fn build_req(config: &Configuration, _: &ArgMatches) -> RequestBuilder {
        config.build_req(ROUTE)
    }
    /// Display the user info main data
    fn display_human_result(user_info: Self::Item) {
        println!("{}\n", "User Information");
        print_info("id", user_info.id.as_str());
        print_info("username", user_info.username.as_str());
        print_info("email", user_info.email.as_str());
        print_info("lang", user_info.lang.as_str());
    }

    /// Create the clap subcommand with its arguments.
    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        add_subcommand_options(SubCommand::with_name(Self::COMMAND))
    }
}

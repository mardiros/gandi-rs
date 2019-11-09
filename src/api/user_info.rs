///! The [user-info](https://api.gandi.net/docs/organization/#get-v5-organization-user-info) route binding

use clap::{App, ArgMatches, SubCommand};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;

use super::super::errors::GandiResult;
use super::super::user_agent::get_client;
use super::super::display::{Format, add_subcommand_options, print_info};

/// endpoint of the route.
const ROUTE: &str = "/v5/organization/user-info";
/// CLAP first sub command name.
const COMMAND_GROUP: &str = "get";
/// CLAP second sub command name.
const COMMAND: &str = "user-info";

/// User Information format, returned by the API
#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
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


/// Display the result for human
fn display_result(user_info: UserInfo, format: Format) -> GandiResult<()> {
    match format {
        Format::JSON => {
            let resp = serde_json::to_string(&user_info)?;
            println!("{}", resp);
        }
        Format::YAML => {
            let resp = serde_yaml::to_string(&user_info)?;
            println!("{}", resp);
        }
        Format::HUMAN => {
            println!("{}\n", "User Information");
            print_info("id", user_info.id.as_str());
            print_info("username", user_info.username.as_str());
            print_info("email", user_info.email.as_str());
            print_info("lang", user_info.lang.as_str());
        }
    }
    Ok(())
}

/// Process the http request and display the result.
fn process(format: Format) -> GandiResult<()> {
    let mut resp = get_client(ROUTE).send()?;
    let user_info: UserInfo = resp.json()?;
    display_result(user_info, format)?;
    Ok(())
}

/// Create the clap subcommand with its arguments.
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    add_subcommand_options(SubCommand::with_name(COMMAND))
}

/// Process the operation in case the matches is processable.
pub fn handle(matches: &ArgMatches) -> GandiResult<bool> {
    if matches.is_present(COMMAND_GROUP) {
        let subcommand = matches.subcommand_matches(COMMAND_GROUP).unwrap();
        if subcommand.is_present(COMMAND) {
            let params = subcommand.subcommand_matches(COMMAND).unwrap();
            let format = Format::from(params);
            process(format)?;
            return Ok(true);
        }
    }
    Ok(false)
}

//! Command Handler Trait.
//! Commands are generic to behave the mode possible identically for user.
//! For instance --json, --yaml and --toml can be used on every commands
//! 
use clap::{App, ArgMatches};
use reqwest::RequestBuilder;
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use serde_yaml;
use toml;

use super::config::Configuration;
use super::display::Format;
use super::errors::{GandiError, GandiResult};

/// Implement this trait on subcommand
pub trait GandiSubCommandHandler
where
    Self::Item: Serialize + DeserializeOwned,
{
    type Item;

    /// Create the clap subcommand with its arguments.
    fn subcommand<'a, 'b>() -> App<'a, 'b>;

    /// Take the parameters and extract the subcommand parameters to properly handle the request
    fn can_handle<'a>(matches: &'a ArgMatches) -> Option<&'a ArgMatches<'a>>;

    /// Build the http request that will be executed
    fn build_req(config: &Configuration, matches: &ArgMatches) -> RequestBuilder;

    /// Display to stdout in case there is no format defined
    fn display_human_result(item: Self::Item);
    /// Override it to display extra informations from the response header
    fn display_human_headers(_: &HeaderMap) -> GandiResult<()> { Ok(()) }

    /// Display the result for human
    fn display_result(item: Self::Item, format: &Format) -> GandiResult<()> {
        match format {
            Format::JSON => {
                let resp = serde_json::to_string(&item)?;
                println!("{}", resp);
            }
            Format::YAML => {
                let resp = serde_yaml::to_string(&item)?;
                println!("{}", resp);
            }
            Format::TOML => {
                let resp = toml::to_string(&item)?;
                println!("{}", resp);
            }
            Format::HUMAN => {
                Self::display_human_result(item);
            }
        }
        Ok(())
    }

    /// Process the operation in case the matches is processable.
    fn handle(config: &Configuration, params: &ArgMatches) -> GandiResult<()> {
        if let Some(params) = Self::can_handle(params) {
            Self::process(config, params)?;
        }
        Ok(())
    }

    /// Process the http request and display the result.
    fn process(config: &Configuration, params: &ArgMatches) -> GandiResult<()> {
        let format = Format::from(params);
        let req = Self::build_req(config, &params);
        let mut resp = req.send()?;
        if resp.status().is_success() {
            // println!("{}", resp.text().unwrap_or("".to_string()));
            let item: Self::Item = resp.json()?;
            Self::display_result(item, &format)?;
            if format == Format::HUMAN {
                Self::display_human_headers(resp.headers())?;
            }
            Ok(())
        } else {
            Err(GandiError::ReqwestResponseError(
                format!("{}", resp.status()),
                resp.text().unwrap_or("".to_string()),
            ))
        }
    }
}

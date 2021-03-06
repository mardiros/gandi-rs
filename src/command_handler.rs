//! Command Handler Trait.
//! Commands are generic to behave the mode possible identically for user.
//! For instance --json, --yaml and --toml can be used on every commands
//!
use clap::{App, ArgMatches};
use reqwest::header::HeaderMap;
use reqwest::RequestBuilder;
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
    const COMMAND_GROUP: &'static str;
    const COMMAND: &'static str;
    type Item;

    /// Create the clap subcommand with its arguments.
    fn subcommand<'a, 'b>() -> App<'a, 'b>;

    /// Take the parameters and extract the subcommand parameters to properly handle the request
    /// fn can_handle<'a>(matches: &'a ArgMatches) -> Option<&'a ArgMatches<'a>>;

    /// Build the http request that will be executed
    fn build_req(config: &Configuration, matches: &ArgMatches) -> RequestBuilder;

    /// Display to stdout in case there is no format defined
    fn display_human_result(item: Self::Item);
    /// Override it to display extra informations from the response header
    fn display_human_headers(_: &HeaderMap) -> GandiResult<()> {
        Ok(())
    }

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

    /// Check if the operation in case the matches is processable.
    fn can_handle<'a>(matches: &'a ArgMatches) -> Option<&'a ArgMatches<'a>> {
        if matches.is_present(Self::COMMAND_GROUP) {
            let subcommand = matches.subcommand_matches(Self::COMMAND_GROUP).unwrap();
            if subcommand.is_present(Self::COMMAND) {
                let params = subcommand.subcommand_matches(Self::COMMAND).unwrap();
                return Some(params);
            }
        }
        None
    }
}

// pub struct GandiCommandHandler<I, F> where
//     I: Serialize + DeserializeOwned,
//     F: FnOnce(I) {
//     command_group: String,
//     command: String,
//     item: I,
//     human_display: F,
// }

// impl<I, F> GandiCommandHandler<I, F>
// where
//     I: Serialize + DeserializeOwned,
//     F: Fn(I),
// {
//     pub fn new(command_group: &str, command: &str, item: I, func: F) -> Self {
//         GandiCommandHandler {
//             command_group: command_group.to_owned(),
//             command: command.to_owned(),
//             item: item,
//             human_display: func,
//         }
//     }

//     /// Check if the operation in case the matches is processable.
//     fn can_handle<'a>(&self, matches: &'a ArgMatches) -> Option<&'a ArgMatches<'a>> {
//         if matches.is_present(self.command_group.as_str()) {
//             let subcommand = matches.subcommand_matches(self.command_group.as_str()).unwrap();
//             if subcommand.is_present(self.command.as_str()) {
//                 let params = subcommand.subcommand_matches(self.command.as_str()).unwrap();
//                 return Some(params);
//             }
//         }
//         None
//     }

//     /// Process the operation in case the matches is processable.
//     fn handle(&self, config: &Configuration, params: &ArgMatches) -> GandiResult<()> {
//         if let Some(params) = self.can_handle(params) {
//             self.process(config, params)?;
//         }
//         Ok(())
//     }

//    /// Process the http request and display the result.
//     fn process(&self, config: &Configuration, params: &ArgMatches) -> GandiResult<()> {
//         let format = Format::from(params);
//         Ok(())

//         // let req = self.build_req(config, &params);
//         // let mut resp = req.send()?;
//         // if resp.status().is_success() {
//         //     // println!("{}", resp.text().unwrap_or("".to_string()));
//         //     let item: I = resp.json()?;
//         //     self.display_result(item, &format)?;
//         //     if format == Format::HUMAN {
//         //         self.display_human_headers(resp.headers())?;
//         //     }
//         //     Ok(())
//         // } else {
//         //     Err(GandiError::ReqwestResponseError(
//         //         format!("{}", resp.status()),
//         //         resp.text().unwrap_or("".to_string()),
//         //     ))
//         // }
//     }
//     /// Display the result for human
//     fn display_result(&self, item: I, format: &Format) -> GandiResult<()> {
//         match format {
//             Format::JSON => {
//                 let resp = serde_json::to_string(&item)?;
//                 println!("{}", resp);
//             }
//             Format::YAML => {
//                 let resp = serde_yaml::to_string(&item)?;
//                 println!("{}", resp);
//             }
//             Format::TOML => {
//                 let resp = toml::to_string(&item)?;
//                 println!("{}", resp);
//             }
//             Format::HUMAN => {
//                 self.human_display(item);
//             }
//         }
//         Ok(())
//     }
// }

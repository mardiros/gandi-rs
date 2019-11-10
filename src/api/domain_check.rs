//! [domains check](https://api.gandi.net/docs/domains/#get-v5-domain-check) route binding
use std::vec::Vec;

use chrono::{DateTime, Utc};
use clap::{App, Arg, ArgMatches, SubCommand};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use toml;

use super::super::date_formatter;
use super::super::display::{add_subcommand_options, print_flag, print_info, Format};
use super::super::errors::{GandiError, GandiResult};
use super::super::user_agent::get_client;

pub const ROUTE: &str = "/v5/domain/check";
pub const COMMAND_GROUP: &str = "check";
pub const COMMAND: &str = "domain";

/// Price tax
#[derive(Debug, Serialize, Deserialize)]
struct Tax {
    /// name of the tax
    name: String,
    /// type of the tax
    #[serde(rename(deserialize = "type"))]
    type_: String,
    /// tax rate in percent
    rate: f32,
}

/// Options Product prices
#[derive(Debug, Serialize, Deserialize)]
struct PriceOptions {
    /// registration period: sunrise, landrush, golive
    period: Option<String>, // badly documented
}

#[derive(Debug, Serialize, Deserialize)]
struct Period {
    /// name of the tax
    name: String,
    /// starting date
    #[serde(with = "date_formatter")]
    starts_at: DateTime<Utc>,
    /// ending date
    #[serde(with = "date_formatter")]
    ends_at: DateTime<Utc>,
}

/// Product prices
#[derive(Debug, Serialize, Deserialize)]
struct Price {
    /// minimum duration for the price
    min_duration: usize,
    /// maximum duration for the price
    max_duration: usize,
    /// duration unit the price expose it
    duration_unit: String,
    /// temporary discount prices
    discount: Option<bool>,

    /// price without taxes
    price_before_taxes: f32,
    /// price taxes included
    price_after_taxes: f32,

    /// options of what ?
    options: PriceOptions,
}

/// Product prices wrapped by process and status
#[derive(Debug, Serialize, Deserialize)]
struct Product {
    /// Status prices are exposed
    process: Option<String>, // marked as optional ?
    /// Status prices are exposed
    status: String,
    /// the fqdn
    name: String,

    /// prices
    prices: Option<Vec<Price>>,
    /// Applied taxes if any
    taxes: Vec<Tax>,
    period: Option<Vec<Period>>,
}

/// Domain Availability Check Format, returned by the API
#[derive(Debug, Serialize, Deserialize)]
struct DomainCheck {
    /// currency prices are exposed
    currency: String,
    /// Gandi grid
    grid: String,
    /// products
    products: Option<Vec<Product>>,
}

/// Display the result for human
fn display_result(check: DomainCheck, format: Format) -> GandiResult<()> {
    match format {
        Format::JSON => {
            let resp = serde_json::to_string(&check)?;
            println!("{}", resp);
        }
        Format::YAML => {
            let resp = serde_yaml::to_string(&check)?;
            println!("{}", resp);
        }
        Format::TOML => {
            let resp = toml::to_string(&check)?;
            println!("{}", resp);
        }
        Format::HUMAN => {
            let golive = "golive".to_string();
            let missing_process = "???".to_string();
            //println!("Check: {:?}", check);

            let products = check.products.unwrap_or(vec![]);
            for product in products {
                if product.status != "available" {
                    print_info(
                        format!(
                            "{} {}",
                            product.process.as_ref().unwrap_or(&missing_process),
                            product.name
                        )
                        .as_str(),
                        product.status.as_str(),
                    )
                } else {
                    let prices = product.prices.unwrap_or(vec![]);
                    for price in prices {
                        print_info(
                            format!(
                                "{} {} {} {}",
                                product.process.as_ref().unwrap_or(&missing_process),
                                product.name,
                                format!(
                                    "{}{}->{}{}",
                                    price.min_duration,
                                    price.duration_unit,
                                    price.max_duration,
                                    price.duration_unit
                                ),
                                price.options.period.as_ref().unwrap_or(&golive),
                            )
                            .as_str(),
                            format!("{} {}", price.price_after_taxes, check.currency).as_str(),
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

/// Process the http request and display the result.
fn process(fqdn: &str, format: Format) -> GandiResult<()> {
    let client = get_client(ROUTE).query(&[("name", fqdn)]);
    let mut resp = client.send()?;
    if resp.status().is_success() {
        let check: DomainCheck = resp.json()?;
        display_result(check, format)?;
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
    let subcommand = SubCommand::with_name(COMMAND).arg(
        Arg::with_name("FQDN")
            .index(1)
            .required(true)
            .help("domain name to query"),
    );
    add_subcommand_options(subcommand)
}

/// Process the operation in case the matches is processable.
pub fn handle(matches: &ArgMatches) -> GandiResult<bool> {
    if matches.is_present(COMMAND_GROUP) {
        let subcommand = matches.subcommand_matches(COMMAND_GROUP).unwrap();
        if subcommand.is_present(COMMAND) {
            let params = subcommand.subcommand_matches(COMMAND).unwrap();
            let format = Format::from(params);
            let fqdn = params.value_of("FQDN").unwrap().to_string();
            process(fqdn.as_str(), format)?;
            return Ok(true);
        }
    }
    Ok(false)
}

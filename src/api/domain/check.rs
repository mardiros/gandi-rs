//! [Check domain availability](https://api.gandi.net/docs/domains/#get-v5-domain-check) route binding
use std::vec::Vec;

use chrono::{DateTime, Utc};
use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

use super::super::super::command_handler::GandiSubCommandHandler;
use super::super::super::config::Configuration;
use super::super::super::display::{add_subcommand_options, print_info};
use super::super::super::filter::sharing_id::{
    add_subcommand_options as add_sharing_id_options, SharingSpace,
};
use super::super::super::formatter::date_formatter;

pub const ROUTE: &str = "/v5/domain/check";

/// Price tax
#[derive(Debug, Serialize, Deserialize)]
struct Tax {
    /// name of the tax
    name: String,
    /// type of the tax
    #[serde(rename(deserialize = "type", serialize = "type"))]
    type_: String,
    /// tax rate in percent
    rate: f32,
}

/// Options Product prices
#[derive(Debug, Serialize, Deserialize)]
struct PriceOptions {
    /// registration period: sunrise, landrush, golive
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    prices: Option<Vec<Price>>,
    /// Applied taxes if any
    taxes: Vec<Tax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Vec<Period>>,
}

/// Domain Availability Check Format, returned by the API
#[derive(Debug, Serialize, Deserialize)]
pub struct DomainCheck {
    /// currency prices are exposed
    currency: String,
    /// Gandi grid
    grid: String,
    /// products
    #[serde(skip_serializing_if = "Option::is_none")]
    products: Option<Vec<Product>>,
}

/// implement the "check domain" subcommand
pub struct DomainCheckCommand {}

impl GandiSubCommandHandler for DomainCheckCommand {
    const COMMAND_GROUP: &'static str = "check";
    const COMMAND: &'static str = "domain";
    type Item = DomainCheck;

    /// Create the route
    fn build_req(config: &Configuration, params: &ArgMatches) -> RequestBuilder {
        let fqdn = params.value_of("FQDN").unwrap().to_string();
        let sharing_space = SharingSpace::from(params);
        let req = config.build_req(ROUTE).query(&[("name", fqdn)]);
        sharing_space.build_req(req)
    }

    /// Display the domain important data
    fn display_human_result(item: Self::Item) {
        let golive = "golive".to_string();
        let missing_process = "???".to_string();
        //println!("Check: {:?}", check);

        let products = item.products.unwrap_or(vec![]);
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
                        format!("{} {}", price.price_after_taxes, item.currency).as_str(),
                    );
                }
            }
        }
    }

    /// Create the clap subcommand with its arguments.
    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        let subcommand = SubCommand::with_name(Self::COMMAND).arg(
            Arg::with_name("FQDN")
                .index(1)
                .required(true)
                .help("domain name to query"),
        );
        let subcommand = add_sharing_id_options(subcommand);
        add_subcommand_options(subcommand)
    }
}

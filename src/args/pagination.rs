//! Display options.
use std::convert::From;

use clap::{App, Arg, ArgMatches};
use reqwest::RequestBuilder;

/// Output format
pub struct Pagination {
    pub page: String,
    pub per_page: String,
}

impl Pagination {
    /// Inject the parameters of the cli in the http request
    pub fn build_req(&self, req: RequestBuilder) -> RequestBuilder {
        req.query(&[("page", self.page.as_str())])
            .query(&[("per_page", self.per_page.as_str())])
    }
}

/// Retrieve the format from the clap subcommand arguments
impl<'a> From<&'a ArgMatches<'a>> for Pagination {
    fn from(params: &ArgMatches<'a>) -> Self {
        Pagination {
            page: params.value_of("PAGE").unwrap().to_string(),
            per_page: params.value_of("PER_PAGE").unwrap().to_string(),
        }
    }
}

/// Create the clap subcommand with its arguments.
pub fn add_subcommand_options<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("PAGE")
            .short("p")
            .long("page")
            .default_value("1")
            .takes_value(true)
            .help("Page Number"),
    )
    .arg(
        Arg::with_name("PER_PAGE")
            .long("per-page")
            .default_value("100")
            .takes_value(true)
            .help("Number of element per page"),
    )
}

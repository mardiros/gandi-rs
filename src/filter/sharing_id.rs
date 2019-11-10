//! Display options.
use std::convert::From;

use clap::{App, Arg, ArgMatches};
use reqwest::RequestBuilder;

/// Output format
pub struct SharingSpace {
    pub sharing_id: String,
}


impl SharingSpace {
    /// Inject the parameters of the cli in the http request
    pub fn build_req(&self, req: RequestBuilder) -> RequestBuilder {
        if self.sharing_id.len() > 0 {
            req.query(&[("sharing_id", self.sharing_id.as_str())])
        }
        else {
            req
        }
    }
    
}

/// Retrieve the format from the clap subcommand arguments
impl<'a> From<&'a ArgMatches<'a>> for SharingSpace {
    fn from(params: &ArgMatches<'a>) -> Self {
        SharingSpace {
            sharing_id: params.value_of("SHARING_ID").unwrap().to_string(),
        }
    }
}

/// Create the clap subcommand with its arguments.
pub fn add_subcommand_options<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
    app.arg(
        Arg::with_name("SHARING_ID")
            .short("s")
            .long("sharing-id")
            .default_value("")
            .takes_value(true)
            .help("The Organization ID"),
    )
}

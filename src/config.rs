use std::env::var as env_var;
use std::io::prelude::*;
//use std::path::PathBuf;
use std::fs::File;

use clap::ArgMatches;
use reqwest::header;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use toml;

use super::constants::VERSION;
use super::errors::GandiResult;

// Build a user agent for our http client
fn user_agent() -> String {
    format!("{}/{}", env!("CARGO_PKG_NAME"), VERSION)
}

fn default_endpoint() -> String {
    "https://api.gandi.net".to_string()
}

fn default_use_env_vars() -> bool {
    false
}

/// CLI Configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    /// API Key used to do the call
    apikey: String,
    /// Endpoint of the public api
    #[serde(default = "default_endpoint")]
    endpoint: String,
    /// If true, then configuration is overridable via environment variable
    #[serde(default = "default_use_env_vars")]
    use_env_vars: bool,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            apikey: env_var("GANDI_APIKEY").unwrap_or("".to_string()),
            endpoint: env_var("GANDI_API_ENDPOINT").unwrap_or(default_endpoint()),
            use_env_vars: true,
        }
    }
}

/// Retrieve the format from the clap subcommand arguments
impl<'a> From<&'a ArgMatches<'a>> for Configuration {
    fn from(params: &ArgMatches<'a>) -> Self {
        let filepath = params.value_of("CONFIG").unwrap_or("").to_string();
        if filepath.len() > 0 {
            Configuration::from_file(filepath.as_str())
                .map_err(|err| {
                    eprintln!("Unable to load configuration file: {}", filepath);
                    eprintln!("{}", err);
                    panic!("Cannot continue due to prefious error");
                })
                .unwrap()
        } else {
            Configuration::default()
        }
    }
}

impl Configuration {
    /// Load the configuration from the given filepath
    pub fn from_file(filepath: &str) -> GandiResult<Self> {
        let mut file = File::open(filepath)?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;
        let res = String::from_utf8(buf).unwrap(); // UTF-8 error will crash...
        let mut res: Configuration = toml::from_str(res.as_str())?;
        if res.use_env_vars {
            if let Ok(key) = env_var("GANDI_APIKEY") {
                res.apikey = key;
            }
            if let Ok(endpoint) = env_var("GANDI_API_ENDPOINT") {
                res.endpoint = endpoint;
            }
        }
        Ok(res)
    }

    /// Build http headers for our configuration
    fn construct_headers(&self) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(format!("Apikey {}", self.apikey()).as_str()).unwrap(),
        );
        headers.insert(
            reqwest::header::USER_AGENT,
            header::HeaderValue::from_str(user_agent().as_str()).unwrap(),
        );
        headers
    }

    pub fn build_req(&self, route: &str) -> RequestBuilder {
        let url = format!("{}{}", self.endpoint(), route);
        let client = reqwest::Client::new();
        client.get(url.as_str()).headers(self.construct_headers())
    }
}

impl Configuration {
    /// the apikey to use
    fn apikey(&self) -> &str {
        self.apikey.as_str()
    }

    /// the http endpoint of the api
    fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
}

//! Default values
//!

use std::env::{var as env_var};
use lazy_static::lazy_static;

/// Version of the CLI
pub const NAME: &str = "gandi";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref APIKEY: String = env_var("GANDI_APIKEY").unwrap_or("".to_string());
    pub static ref ENDPOINT: String = env_var("GANDI_API_ENDPOINT").unwrap_or("https://api.gandi.net".to_string());
}

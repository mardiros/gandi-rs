//! Define results and error. `Result<T, GandiError>`

use std::error::Error;
use std::fmt::{self, Display};

use reqwest::Error as ReqwestError;
use serde_yaml::Error as SerdeYamlError;
use serde_json::error::Error as SerdeJsonError;
use toml::ser::Error as TomlSerError;


#[derive(Debug)]
/// Errors in Gandi CLI
pub enum GandiError {
    // Wrapped errors
    ReqwestError(ReqwestError),
    SerdeJsonError(String),
    SerdeYamlError(String),
    TomlSerError(String),
}

/// Result used by method that can failed.
pub type GandiResult<T> = Result<T, GandiError>;

impl Display for GandiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            GandiError::ReqwestError(err) => format!("ReqwestError: {}", err),
            GandiError::SerdeJsonError(err) => format!("Json Formatting Error: {}", err),
            GandiError::SerdeYamlError(err) => format!("Yaml Formatting Error: {}", err),
            GandiError::TomlSerError(err) => format!("Toml Formatting Error: {}", err),
        };
        write!(f, "{}", description)
    }
}

impl Error for GandiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        let err: Option<&(dyn Error + 'static)> = match self {
            GandiError::ReqwestError(err) => Some(err),
            _ => None,
        };
        err
    }
}

impl From<SerdeJsonError> for GandiError {
    fn from(err: SerdeJsonError) -> GandiError {
        GandiError::SerdeJsonError(format!("{}", err))
    }
}

impl From<SerdeYamlError> for GandiError {
    fn from(err: SerdeYamlError) -> GandiError {
        GandiError::SerdeYamlError(format!("{}", err))
    }
}

impl From<ReqwestError> for GandiError {
    fn from(err: ReqwestError) -> GandiError {
        GandiError::ReqwestError(err)
    }
}


impl From<TomlSerError> for GandiError {
    fn from(err: TomlSerError) -> GandiError {
        GandiError::TomlSerError(format!("{}", err))
    }
}

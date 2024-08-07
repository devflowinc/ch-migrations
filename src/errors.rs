use std::env::VarError;

use derive_more::Display;

#[derive(Debug, Display, Clone)]
pub enum CLIError {
    #[display(fmt = "BadArgs: {_0}")]
    BadArgs(String),
    #[display(fmt = "InternalError: {_0}")]
    InternalError(String),
    #[display(fmt = "DBError: {_0}")]
    DBError(String),
}

impl From<std::io::Error> for CLIError {
    fn from(value: std::io::Error) -> Self {
        Self::InternalError(format!("IO error: {:?}", value))
    }
}

impl From<VarError> for CLIError {
    fn from(value: VarError) -> Self {
        Self::BadArgs(format!("{:?}", value))
    }
}

impl From<clickhouse::error::Error> for CLIError {
    fn from(value: clickhouse::error::Error) -> Self {
        Self::DBError(value.to_string())
    }
}

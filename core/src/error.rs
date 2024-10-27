use std::path::PathBuf;

use base64::DecodeError;
use http::header::{InvalidHeaderValue, ToStrError};
use thiserror::Error as ThisError;

use crate::api_key::ApiKey;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("unable to authenticate key: {key:?}")]
    BadKey { key: ApiKey },

    #[error("missing key header")]
    MissingKeyHeader,

    #[error("could not parse address: {0}")]
    ParseAddress(String),

    #[error("could not find the path to {0}")]
    UnknownPath(String),

    #[error("error reading file {file:?}: {source}")]
    FileRead {
        source: std::io::Error,
        file: PathBuf,
    },

    #[error("unable to deserialize TOML: {0:?}")]
    Toml(#[from] toml::de::Error),

    #[error("unable to parse header: {0:?}")]
    HeaderParse(#[from] ToStrError),

    #[error("header included illegal characters: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error("unable to decode base64 value: {0}")]
    Base64Decode(#[from] DecodeError),

    #[error("unable to load API key")]
    MissingKeyConfig,
}

impl From<Error> for Box<pingora::Error> {
    fn from(value: Error) -> Self {
        match &value {
            Error::BadKey { key: _ } => pingora::Error::because(
                pingora::ErrorType::HTTPStatus(403),
                "could not validate key",
                value,
            ),
            Error::MissingKeyHeader => pingora::Error::because(
                pingora::ErrorType::HTTPStatus(403),
                "missing header",
                value,
            ),
            Error::ParseAddress(_) => pingora::Error::because(
                pingora::ErrorType::InternalError,
                "error parsing address",
                value,
            ),
            Error::UnknownPath(_) => pingora::Error::because(
                pingora::ErrorType::ConnectRefused,
                "couldn't find path in routes",
                value,
            ),
            Error::FileRead { source: _, file: _ } => pingora::Error::because(
                pingora::ErrorType::InternalError,
                "error reading file",
                value,
            ),
            Error::Toml(_) => pingora::Error::because(
                pingora::ErrorType::InternalError,
                "TOML deserialization error",
                value,
            ),
            Error::HeaderParse(_) => pingora::Error::because(
                pingora::ErrorType::ConnectRefused,
                "could not parse header value",
                value,
            ),
            Error::Base64Decode(_) => pingora::Error::because(
                pingora::ErrorType::ConnectRefused,
                "error parsing base64 value",
                value,
            ),
            Error::InvalidHeaderValue(_) => pingora::Error::because(
                pingora::ErrorType::ConnectRefused,
                "error parsing base64 value",
                value,
            ),
            Error::MissingKeyConfig => pingora::Error::because(
                pingora::ErrorType::InternalError,
                "no API key found in configuration",
                value,
            ),
        }
    }
}

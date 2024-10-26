use base64::DecodeError;
use http::{header::ToStrError, HeaderValue};
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("unable to authenticate key: {key:?}")]
    BadKey { key: Vec<u8> },
    #[error("missing key header")]
    MissingKey,

    #[error("could not parse address: {0}")]
    ParseAddress(String),

    #[error("could not find the path to {0}")]
    UnknownPath(String),

    #[error("error reading file: {0}")]
    FileRead(std::io::Error),

    #[error("unable to deserialize TOML: {0:?}")]
    Toml(#[from] toml::de::Error),

    #[error("unable to parse header: {0:?}")]
    HeaderParse(#[from] ToStrError),

    #[error("unable to decode base64 value: {0}")]
    Base64Decode(#[from] DecodeError),
}

impl From<Error> for Box<pingora::Error> {
    fn from(value: Error) -> Self {
        match &value {
            Error::BadKey { key: _ } => pingora::Error::because(
                pingora::ErrorType::HTTPStatus(403),
                "could not validate key",
                value,
            ),
            Error::MissingKey => pingora::Error::because(
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
            Error::FileRead(_) => pingora::Error::because(
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
        }
    }
}

use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("unable to authenticate key: {key}")]
    BadKey { key: String },
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
}

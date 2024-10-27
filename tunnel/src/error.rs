use pingora::Error as PingoraError;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] gandalf_core::error::Error),

    #[error("API key is required to run the tunnel")]
    MissingApiKey,

    #[error("there was an unexpected auth header in the request")]
    ExistingHeader,

    #[error(transparent)]
    XdgConfig(#[from] xdg::BaseDirectoriesError),
}

impl From<Error> for Box<PingoraError> {
    fn from(value: Error) -> Self {
        match value {
            Error::Core(error) => error.into(),
            Error::MissingApiKey => pingora::Error::because(
                pingora::ErrorType::ConnectRefused,
                "API key is required",
                value,
            ),
            Error::ExistingHeader => pingora::Error::because(
                pingora::ErrorType::ConnectRefused,
                "there was an unexpected existing auth header",
                value,
            ),
            Error::XdgConfig(_) => pingora::Error::because(
                pingora::ErrorType::InternalError,
                "there was an unexpected existing auth header",
                value,
            ),
        }
    }
}

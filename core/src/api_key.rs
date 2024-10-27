use std::{collections::HashSet, path::Path};

use base64::{engine::GeneralPurpose, Engine as _};
use http::HeaderValue;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    io::read_to_string,
};

const API_KEY_ENV_VAR: &str = "GANDALF_API_KEY";
const BASE64_ENGINE: GeneralPurpose = base64::engine::general_purpose::URL_SAFE;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ApiKey(Vec<u8>);

impl ApiKey {
    pub fn from_env() -> Result<ApiKey> {
        Self::from_env_var(API_KEY_ENV_VAR)
    }

    pub fn from_env_var(name: &'static str) -> Result<ApiKey> {
        std::env::var(name)
            .map(|s| ApiKey(s.as_bytes().to_vec()))
            .map_err(|_err| Error::MissingKeyConfig)
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<HashSet<ApiKey>> {
        let keys = read_to_string(path.as_ref())?;
        Ok(keys
            .lines()
            .map(|line| ApiKey(line.as_bytes().to_vec()))
            .collect())
    }
}

impl TryFrom<&ApiKeyBase64> for ApiKey {
    type Error = Error;

    fn try_from(value: &ApiKeyBase64) -> Result<Self> {
        let api_key = BASE64_ENGINE.decode(value.0.as_bytes())?;

        Ok(ApiKey(api_key))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ApiKeyBase64(String);

impl From<&ApiKey> for ApiKeyBase64 {
    fn from(value: &ApiKey) -> Self {
        Self(BASE64_ENGINE.encode(&value.0))
    }
}

impl TryFrom<&HeaderValue> for ApiKeyBase64 {
    type Error = Error;

    fn try_from(value: &HeaderValue) -> Result<Self> {
        let string = value.to_str()?.to_string();
        Ok(Self(string))
    }
}

impl TryFrom<&ApiKeyBase64> for HeaderValue {
    type Error = Error;

    fn try_from(value: &ApiKeyBase64) -> Result<Self> {
        let header_value = HeaderValue::from_str(&value.0)?;
        Ok(header_value)
    }
}

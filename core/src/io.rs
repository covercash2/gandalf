use std::{
    net::{SocketAddr, ToSocketAddrs as _},
    path::Path,
};

use serde::de::DeserializeOwned;

use crate::error::{Error, Result};

pub fn read_to_string(path: impl AsRef<Path>) -> Result<String> {
    std::fs::read_to_string(path.as_ref()).map_err(|err| Error::FileRead {
        source: err,
        file: path.as_ref().into(),
    })
}

pub fn read_toml_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let contents = read_to_string(path)?;
    let deserialized: T = toml::from_str(&contents)?;
    Ok(deserialized)
}

pub trait ToSocketAddr {
    fn to_socket_addr(&self) -> Result<SocketAddr>;
}

impl ToSocketAddr for &str {
    fn to_socket_addr(&self) -> Result<SocketAddr> {
        let address = self
            .to_socket_addrs()
            .map_err(|err| Error::ParseAddress(err.to_string()))?
            .next()
            .ok_or(Error::ParseAddress(
                "got empty iterator parsing address".to_string(),
            ))?;

        Ok(address)
    }
}

impl ToSocketAddr for (&str, u16) {
    fn to_socket_addr(&self) -> Result<SocketAddr> {
        let address = (self.0, self.1)
            .to_socket_addrs()
            .map_err(|err| Error::ParseAddress(err.to_string()))?
            .next()
            .ok_or(Error::ParseAddress(
                "got empty iterator parsing address".to_string(),
            ))?;

        Ok(address)
    }
}

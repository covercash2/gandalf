use std::{
    collections::{HashMap, HashSet},
    net::{SocketAddr, ToSocketAddrs as _},
    path::Path,
};

use async_trait::async_trait;
use http::{HeaderName, Uri};
use pingora::{
    prelude::HttpPeer,
    proxy::{ProxyHttp, Session},
    {Error as PingoraError, Result as PingoraResult},
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::error::{Error, Result};

const KEY_HEADER: HeaderName = HeaderName::from_static("fellowship");

pub struct ApiGateway {
    routes: HashMap<String, PeerRoute>,
    keys: HashSet<String>,
}

impl ApiGateway {
    pub fn new(routes: Vec<PeerRoute>, keys: HashSet<String>) -> Self {
        let routes = routes
            .into_iter()
            .map(|route| (route.request_path.clone(), route))
            .collect();
        ApiGateway { routes, keys }
    }

    /// Parse the path from the `uri`
    /// and determine the address of that path
    fn get_address(&self, uri: &Uri) -> Result<SocketAddr> {
        let path = parse_uri_root_path(uri)?;
        self.routes
            .get(path)
            .ok_or(Error::UnknownPath(path.into()))?
            .socket_addr()
    }
}

fn parse_uri_root_path(uri: &Uri) -> Result<&str> {
    let path: &Path = uri.path().as_ref();
    let path = path
        .iter()
        .nth(1)
        .and_then(|os_str| os_str.to_str())
        .ok_or(Error::UnknownPath(format!("{path:?}")))?;
    Ok(path)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeerRoute {
    request_path: String,
    host: String,
    port: u16,
    name: String,
}

impl PeerRoute {
    pub fn socket_addr(&self) -> Result<SocketAddr> {
        let address = (self.host.as_str(), self.port)
            .to_socket_addrs()
            .map_err(|err| Error::ParseAddress(err.to_string()))?
            .next()
            .ok_or(Error::ParseAddress(
                "got empty iterator parsing address".to_string(),
            ))?;

        Ok(address)
    }
}

#[async_trait]
impl ProxyHttp for ApiGateway {
    type CTX = ();
    fn new_ctx(&self) {}

    #[instrument(skip(session, self))]
    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut (),
    ) -> PingoraResult<Box<HttpPeer>> {
        tracing::debug!("processing peer request");
        let address = self
            .get_address(&session.req_header().uri)
            .map_err(|error| {
                PingoraError::because(
                    pingora::ErrorType::InternalError,
                    "error parsing address",
                    error,
                )
            })?;
        let peer = HttpPeer::new(address, false, "hoss".to_string());

        tracing::info!(?peer, "configured peer");

        Ok(Box::new(peer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_URIS: &[(&str, &str)] = &[("http://0.0.0.0/api/users", "api"), ("/api", "api")];

    #[test]
    fn parse_path() {
        for (uri, expected) in TEST_URIS {
            let parsed: Uri = uri.parse().expect("should be able to parse test URIs");
            let result =
                parse_uri_root_path(&parsed).expect("should be able to parse path from URI");
            assert_eq!(result, *expected);
        }
    }
}

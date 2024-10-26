use std::{
    collections::{HashMap, HashSet},
    net::{SocketAddr, ToSocketAddrs as _},
};

use async_trait::async_trait;
use http::HeaderName;
use pingora::{
    prelude::HttpPeer,
    proxy::{ProxyHttp, Session},
    Result as PingoraResult,
};
use serde::{Deserialize, Serialize};

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

    fn get_address(&self, path: &str) -> Result<SocketAddr> {
        self.routes
            .get(path)
            .ok_or(Error::UnknownPath(path.into()))?
            .socket_addr()
    }
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

    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut (),
    ) -> PingoraResult<Box<HttpPeer>> {
        let address = self.get_address(session.req_header().uri.path()).unwrap();
        let peer = HttpPeer::new(address, false, "hoss".to_string());

        Ok(Box::new(peer))
    }
}

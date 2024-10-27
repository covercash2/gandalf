use std::net::SocketAddr;

use async_trait::async_trait;
use gandalf_core::{api_key::ApiKeyBase64, KEY_HEADER};
use pingora::{
    prelude::HttpPeer,
    proxy::{ProxyHttp, Session},
    Result as PingoraResult,
};
use tracing::instrument;

use crate::error::Error;

pub struct Tunnel {
    key: ApiKeyBase64,
    proxy_address: SocketAddr,
    https_enable: bool,
}

impl Tunnel {
    pub fn new(key: ApiKeyBase64, proxy_address: SocketAddr, https_enable: bool) -> Self {
        Tunnel {
            key,
            proxy_address,
            https_enable,
        }
    }
}

#[async_trait]
impl ProxyHttp for Tunnel {
    type CTX = ();
    fn new_ctx(&self) {}

    #[instrument(skip(session, self))]
    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut (),
    ) -> PingoraResult<Box<HttpPeer>> {
        tracing::debug!("processing peer request");

        let header_already_exists = session
            .req_header_mut()
            .append_header(KEY_HEADER, &self.key)?;

        if header_already_exists {
            return Err(Error::ExistingHeader.into());
        }

        // TODO: HTTPS
        let peer = HttpPeer::new(self.proxy_address, self.https_enable, "chrasharca.de".to_string());

        tracing::info!(?peer, "configured peer");

        Ok(Box::new(peer))
    }
}

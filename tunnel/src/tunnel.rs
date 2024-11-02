use std::net::SocketAddr;

use async_trait::async_trait;
use gandalf_core::{api_key::ApiKeyBase64, KEY_HEADER};
use pingora::{
    http::ResponseHeader,
    prelude::HttpPeer,
    proxy::{ProxyHttp, Session},
    utils::tls::CertKey,
    Result as PingoraResult,
};
use tracing::instrument;

pub struct Tunnel {
    key: ApiKeyBase64,
    proxy_address: SocketAddr,
}

impl Tunnel {
    pub fn new(key: ApiKeyBase64, proxy_address: SocketAddr) -> Self {
        Tunnel { key, proxy_address }
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
        let uri = &session.req_header().uri;
        tracing::debug!(%uri, "processing peer request");

        let header_already_exists = session
            .req_header_mut()
            .append_header(KEY_HEADER, &self.key)?;

        tracing::debug!(
            headers = ?session.req_header().headers,
            "request headers"
        );

        if header_already_exists {
            tracing::info!("found an exising auth header");
            // return Err(Error::ExistingHeader.into());
        }

        let mut peer = HttpPeer::new(self.proxy_address, true, "chrasharca.de".to_string());
        // let cert: Vec<u8> = include_bytes!("../../ssl/localhost+4.pem").to_vec();
        let (certs, key) = pingora::tls::load_certs_and_key_files(
            "./ssl/localhost+4.pem",
            "./ssl/localhost+4-key.pem",
        )
        .unwrap()
        .unwrap();
        // let key: Vec<u8> = include_bytes!("../../ssl/localhost+4-key.pem").to_vec();
        let cert_key = CertKey::new(
            certs.into_iter().map(|cert| cert.to_vec()).collect(),
            key.secret_der().to_vec(),
        );
        peer.client_cert_key = Some(cert_key.into());

        Ok(Box::new(peer))
    }

    async fn request_filter(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> PingoraResult<bool> {
        tracing::info!(uri = %session.req_header().uri);
        if session.req_header().uri.to_string() == *"/" {
            tracing::info!("root pinged");
            session.write_response_body(Some("OK".into()), true).await?;
            let headers = ResponseHeader::build(200, None)?;
            session.set_keepalive(None);
            session
                .write_response_header(Box::new(headers), true)
                .await?;
            session.finish_body().await?;
            return Ok(true);
        }
        Ok(false)
    }
}

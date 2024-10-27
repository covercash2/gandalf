use config::Config;
use gandalf_core::{api_key::ApiKey, io::ToSocketAddr};
use pingora::{proxy::http_proxy_service, server::Server};
use tunnel::Tunnel;

mod config;
mod error;
mod tunnel;

fn main() -> anyhow::Result<()> {
    let api_key = ApiKey::from_env()?;

    let config = Config::load()?;
    gandalf_core::setup_tracing(config.log_level.as_str());

    let tunnel = Tunnel::new(
        (&api_key).into(),
        config.proxy_address.as_str().to_socket_addr()?,
    );

    let mut server = Server::new(None).expect("unable to build server");

    let mut proxy_service = http_proxy_service(&server.configuration, tunnel);
    let address = format!("127.0.0.1:{}", config.port);
    proxy_service.add_tcp(&address);

    server.bootstrap();
    server.add_service(proxy_service);
    server.run_forever();
}

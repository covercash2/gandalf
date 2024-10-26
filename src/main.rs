use api_gateway::ApiGateway;
use pingora::{proxy::http_proxy_service, server::Server};
use tracing_subscriber::fmt::format::FmtSpan;

mod api_gateway;
mod config;
mod error;
mod io;

fn setup_tracing(level: &str) {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_env_filter(level)
        .json()
        .init();
}

fn main() -> anyhow::Result<()> {
    let config = option_env!("CONFIG").unwrap_or("test").to_lowercase();
    let config = config::load_config(&config)?;
    setup_tracing(config.log_level.as_str());

    let keys = config.keys()?;
    let gateway = ApiGateway::new(config.peers.clone(), keys);

    let mut server = Server::new(None).expect("unable to build server");

    let mut proxy_service = http_proxy_service(&server.configuration, gateway);
    let address = format!("0.0.0.0:{}", config.port);
    proxy_service.add_tcp(&address);

    server.bootstrap();
    server.add_service(proxy_service);
    server.run_forever();
}

use api_gateway::ApiGateway;
use gandalf_core::api_key::ApiKey;
use pingora::{proxy::http_proxy_service, server::Server};

mod api_gateway;
mod config;

fn main() -> anyhow::Result<()> {
    let config = option_env!("CONFIG").unwrap_or("test").to_lowercase();
    let config = config::load_config(&config)?;
    gandalf_core::setup_tracing(config.log_level.as_str());

    let keys = ApiKey::from_file(&config.key_path)?;
    let gateway = ApiGateway::new(config.peers.clone(), keys);

    let mut server = Server::new(None).expect("unable to build server");

    let mut proxy_service = http_proxy_service(&server.configuration, gateway);
    let address = format!("0.0.0.0:{}", config.port);
    proxy_service.add_tcp(&address);

    server.bootstrap();
    server.add_service(proxy_service);
    server.run_forever();
}

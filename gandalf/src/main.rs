use api_gateway::ApiGateway;
use clap::Parser;
use gandalf_core::api_key::ApiKey;
use pingora::{
    prelude::Opt,
    proxy::http_proxy_service,
    server::{configuration::ServerConf, Server},
};

mod api_gateway;
mod config;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    config: String,
}

fn main() -> anyhow::Result<()> {
    let cli_args = Cli::parse();
    let config = config::load_config(&cli_args.config)?;
    gandalf_core::setup_tracing(config.log_level.as_str());

    let keys = ApiKey::from_file(&config.key_path)?;
    let gateway = ApiGateway::new(config.peers.clone(), keys);

    let server_config = ServerConf {
        ca_file: config.ca_file.clone().map(|file| file.display().to_string()),
        ..Default::default()
    };
    let mut server = Server::new_with_opt_and_conf(Opt::default(), server_config);
    server.bootstrap();

    tracing::info!(
        server_config = ?server.configuration,
        "server config loaded",
    );

    let mut proxy_service = http_proxy_service(&server.configuration, gateway);
    let address = format!("0.0.0.0:{}", config.port);
    tracing::info!(%address, "starting TCP server");

    proxy_service.add_tls(
        &address,
        config.ca_file.unwrap().to_string_lossy().as_ref(),
        config.key_file.unwrap().to_string_lossy().as_ref(),
    ).unwrap();

    server.add_service(proxy_service);
    server.run_forever();
}

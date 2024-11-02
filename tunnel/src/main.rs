use std::path::PathBuf;

use clap::Parser;
use config::Config;
use gandalf_core::{api_key::ApiKey, io::ToSocketAddr};
use pingora::{
    prelude::Opt,
    proxy::http_proxy_service,
    server::{configuration::ServerConf, Server},
};
use tunnel::Tunnel;

mod config;
mod error;
mod tunnel;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli_args = Cli::parse();
    let api_key = ApiKey::from_env()?;

    let config = Config::load(cli_args.config)?;
    gandalf_core::setup_tracing(config.log_level.as_str());

    tracing::info!(?config, "starting tunnel");

    let tunnel = Tunnel::new(
        (&api_key).into(),
        config.proxy_address.as_str().to_socket_addr()?,
    );

    let server_config = ServerConf {
        ca_file: config.ca_file.map(|file| file.display().to_string()),
        ..Default::default()
    };
    let mut server = Server::new_with_opt_and_conf(Opt::default(), server_config);
    server.bootstrap();

    tracing::info!(
        server_config = ?server.configuration,
        "server config loaded",
    );

    let mut proxy_service = http_proxy_service(&server.configuration, tunnel);
    let address = format!("127.0.0.1:{}", config.port);
    proxy_service.add_tcp(&address);
    tracing::info!(%address, "started TCP server");

    server.add_service(proxy_service);
    server.run_forever();
}

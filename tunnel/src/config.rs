use std::path::{Path, PathBuf};

use gandalf_core::{io::read_toml_file, CONFIG_DIR_PREFIX};
use serde::{Deserialize, Serialize};

use crate::error::Result;

const CONFIG_FILENAME: &str = "tunnel.toml";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub proxy_address: String,
    pub port: u16,
    pub log_level: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;
        let config = load_config(&config_path)?;
        Ok(config)
    }
}

/// Get the path to the config for this app.
///
fn get_config_path() -> Result<PathBuf> {
    match option_env!("GANDALF_CONFIG") {
        Some(s) => Ok(s.into()),
        None => {
            let xdg = xdg::BaseDirectories::with_prefix(CONFIG_DIR_PREFIX)?;
            let config_path = xdg.get_config_file(CONFIG_FILENAME);
            Ok(config_path)
        }
    }
}

fn load_config(config_path: impl AsRef<Path>) -> Result<Config> {
    let config: Config = read_toml_file(config_path)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn config_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./config")
    }

    #[test]
    fn load_test_config() {
        let config_dir = config_dir();
        let _config = load_config(config_dir).expect("should be able to load test config");
    }
}

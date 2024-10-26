use std::{collections::HashSet, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{api_gateway::PeerRoute, error::Result};

fn config_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./config")
}

pub fn load_config(name: &str) -> Result<Config> {
    let path = config_dir().join(format!("{name}.toml"));
    let contents = crate::io::read_to_string(path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub peers: Vec<PeerRoute>,
    pub port: u16,
    pub key_path: PathBuf,
}

impl Config {
    pub fn keys(&self) -> Result<HashSet<String>> {
        let keys = crate::io::read_to_string(&self.key_path)?;
        Ok(keys.lines().map(ToString::to_string).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configs_are_parsed() {
        config_dir()
            .read_dir()
            .expect("should be able to get config dir")
            .for_each(|dir_entry| {
                let dir_entry = dir_entry.expect("should be able to read config dir contents");
                let contents = std::fs::read_to_string(dir_entry.path())
                    .expect("should be able to read config files");
                let _config: Config =
                    toml::from_str(&contents).expect("should be able to parse configs");
            });
    }
}

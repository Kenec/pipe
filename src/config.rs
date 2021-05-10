use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub destination: Destination,
    pub sources: Sources,
}

#[derive(Deserialize, Debug)]
pub struct Destination {
    pub elasticsearch: Elasticsearch,
}

#[derive(Deserialize, Debug)]
pub struct Sources {
    pub files: Files,
}

#[derive(Deserialize, Debug)]
pub struct Elasticsearch {
    pub host: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Files {
    pub logs: Vec<HashMap<String, String>>,
}

impl Config {
    pub fn load<T: AsRef<Path>>(config_path: T) -> anyhow::Result<Config> {
        if !config_path.as_ref().exists() {
            anyhow::bail!("Configuration path cannot be found or path is not readable");
        }

        match serde_any::from_file(config_path) {
            Ok(cfg) => Ok(cfg),
            Err(error) => Err(anyhow::anyhow!(
                "An error has occurred while loading config, {}",
                error
            )),
        }
    }
}

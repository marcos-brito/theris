use crate::{Applier, Theme};
use anyhow::{anyhow, Context, Result};
use log::warn;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::PathBuf;

#[derive(PartialEq, Debug, Clone)]
pub struct Config {
    appliers: Vec<Applier>,
    themes: Vec<Theme>,
}

impl Default for Config {
    fn default() -> Self {
        todo!()
    }
}

impl Config {
    fn new(config_dir: PathBuf) -> Result<Self> {
        Ok(Config {
            themes: Config::read_file::<Vec<Theme>>(config_dir.join("themes.yaml"))?,
            appliers: Config::read_file::<Vec<Applier>>(config_dir.join("appliers.yaml"))?,
        })
    }

    fn read_file<T>(path: PathBuf) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let yaml =
            fs::read_to_string(&path).context(anyhow!("Failed to read {}", path.display()))?;

        if yaml.is_empty() {
            warn!("{} is empty", path.display());
        }

        Ok(serde_yaml::from_str::<T>(&yaml)?)
    }
}

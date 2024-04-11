use std::{
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mod {
    pub name: String,
    pub description: String,
    pub author: String,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub steam_path: PathBuf,
    pub td_path: PathBuf,
    pub mods: Vec<Mod>,
}

pub fn get_config() -> Result<Config, Error> {
    let config_file = Path::new("patcher.tdcfg");

    if config_file.exists() {
        let open = fs::read(config_file)?;
        match bincode::deserialize(&open) {
            Ok(c) => Ok(c),
            Err(e) => {
                error!("Error while deserializing config file: {e:?}");
                Err(Error::new(
                    ErrorKind::InvalidData,
                    "Couldn't deserialize config",
                ))
            }
        }
    } else {
        error!("Config file not found!");
        Err(Error::new(ErrorKind::NotFound, "Config file not found"))
    }
}

pub fn save_config(cfg: &Config) -> Result<(), Error> {
    let config_file = Path::new("patcher.tdcfg");

    match bincode::serialize(&cfg) {
        Ok(v) => {
            fs::write(config_file, v)?;
            info!("Successfully written to config: {cfg:?}");
            Ok(())
        }
        Err(e) => {
            error!("Error while serializing config: {e:?}");
            Err(Error::new(
                ErrorKind::InvalidData,
                "Couldn't serialize config",
            ))
        }
    }
}

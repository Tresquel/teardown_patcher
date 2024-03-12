use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mod {
    pub name: String,
    pub description: String,
    pub author: String,
    pub active: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub steam_path: String,
    pub td_path: String,
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
        let default_config = Config {
            steam_path: String::new(),
            td_path: String::new(),
            mods: vec![],
        };

        info!("Config didn't exist, returning a empty one: {default_config:?}");
        Ok(default_config)
    }
}

pub fn save_config(cfg: Config) -> Result<(), Error> {
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

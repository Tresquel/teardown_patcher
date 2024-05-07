use std::{
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};

use crate::{steam, teardown};

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
    pub patched_files: Vec<PathBuf>,
}

pub fn get_config() -> Result<Config, Error> {
    let config_file = Path::new("patcher.tdcfg");
    info!("get_config(): checking if config exists");

    if config_file.exists() {
        info!("get_config(): config exists, opening and deserializing...");
        let open = fs::read(config_file)?;

        match bincode::deserialize(&open) {
            Ok(c) => Ok(c),
            Err(e) => {
                error!("get_config(): Error while deserializing config file: {e:?}");
                Err(Error::new(
                    ErrorKind::InvalidData,
                    "Couldn't deserialize config",
                ))
            }
        }
    } else {
        error!("get_config(): Config file not found!");
        Err(Error::new(ErrorKind::NotFound, "Config file not found"))
    }
}

pub fn save_config(cfg: &Config) -> Result<(), Error> {
    let config_file = Path::new("patcher.tdcfg");

    match bincode::serialize(&cfg) {
        Ok(v) => match fs::write(config_file, v) {
            Ok(()) => {
                info!("save_config(): Successfully written to config: {cfg:?}");
                Ok(())
            }
            Err(e) => {
                error!("save_config(): Error occured while writing config: {}", e);
                Err(e)
            }
        },
        Err(e) => {
            error!("save_config(): Error while serializing config: {e:?}");
            Err(Error::new(
                ErrorKind::InvalidData,
                "Couldn't serialize config",
            ))
        }
    }
}

pub fn init_config() -> Result<Config, Error> {
    match get_config() {
        Ok(v) => {
            debug!("init_config(): Config file exists");
            return Ok(v);
        }
        Err(_) => {
            warn!("init_config(): Config file doesn't exist, intializing a new one..");
        }
    }

    let mut config = Config {
        steam_path: steam::get_steam_path()?,
        td_path: PathBuf::new(),
        patched_files: vec![],
    };

    if let Ok(v) = steam::get_teardown_path() {
        config.td_path = v;
    } else {
        info!("init_config(): Not found, asking user..");
        config.td_path = teardown::ask_for_directory()?;
    }

    info!("init_config(): Saving config");
    save_config(&config)?;

    Ok(config)
}

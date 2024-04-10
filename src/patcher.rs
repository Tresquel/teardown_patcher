use std::{fs, io::Error, path::Path};

use log::{info, warn};

use crate::{
    config::{self, Config, Mod},
    steam, teardown,
};

pub fn patch() -> Result<bool, Error> {
    let mut config = init_config()?;
    config.mods = list_mods()?;
    
    Ok(true)
}

fn init_config() -> Result<Config, Error> {
    match config::get_config() {
        Ok(v) => {
            return Ok(v);
        }
        Err(_) => {
            warn!("Config file doesn't exist, intializing a new one..")
        }
    }

    let mut config = Config {
        steam_path: steam::get_steam_path()?,
        td_path: String::new(),
        mods: vec![], // its gonna get refreshed anyway
    };

    match steam::get_teardown_path() {
        Ok(v) => config.td_path = v,
        Err(_) => {
            config.td_path = teardown::ask_for_directory()?;
        }
    }

    config::save_config(&config)?;

    Ok(config)
}

fn list_mods() -> Result<Vec<Mod>, Error> {
    let mut mods: Vec<Mod> = Vec::new();

    let mods_path = Path::new("./mods");
    if !mods_path.try_exists()? {
        warn!("Mods path doesn't exist, creating");
        fs::create_dir(mods_path)?;
    }

    for entry in fs::read_dir("./mods")? {
        let path = entry?.path();
        info!("Found entry {:?}", path);
    }

    Ok(mods)
}

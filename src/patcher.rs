use log::{error, info, warn};
use serde::Deserialize;
use std::{
    fs::{self, File},
    io::{Error, Read},
    path::Path,
};
use zip::ZipArchive;

use crate::{
    config::{self, Config, Mod},
    steam, teardown,
};

#[derive(Deserialize)]
struct Manifest {
    name: String,
    description: String,
    author: String,
}

pub fn patch() -> Result<bool, Box<dyn std::error::Error>> {
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

fn list_mods() -> Result<Vec<Mod>, Box<dyn std::error::Error>> {
    let mut mods: Vec<Mod> = Vec::new();

    let mods_path = Path::new(".\\mods");
    if !mods_path.try_exists()? {
        warn!("Mods path doesn't exist, creating");
        fs::create_dir(mods_path)?;
    }

    for entry in fs::read_dir(".\\mods")? {
        let path = entry?.path();
        info!("Found entry {:?}", path);

        let zip_file = File::open(&path)?;
        let mut archive = ZipArchive::new(zip_file)?;

        // read file
        let mut manifest_file = String::new();
        match archive.by_name("manifest.toml") {
            Ok(mut v) => v.read_to_string(&mut manifest_file)?,
            Err(_) => {
                error!("manifest.toml not found in archive {:?}", &path);
                continue;
            }
        };

        // deserialize it and add it to the list
        let manifest: Manifest = toml::from_str(&manifest_file)?;
        let found_mod = Mod {
            name: manifest.name,
            description: manifest.description,
            author: manifest.author,
            path: path.into_os_string().into_string().unwrap(),
            active: false,
        };

        info!("Adding mod to the mods list: {:?}", found_mod);
        mods.push(found_mod);
    }

    Ok(mods)
}

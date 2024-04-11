use log::{debug, error, info, warn};
use serde::Deserialize;
use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Read},
    path::{Path, PathBuf},
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

    backup(config.td_path.join("data\\ui\\splash.lua"))?;
    restore(config.td_path.join("data\\ui\\splash.lua.bak"))?;

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
        td_path: PathBuf::new(),
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
            path,
        };

        info!("Adding mod to the mods list: {:?}", found_mod);
        mods.push(found_mod);
    }

    Ok(mods)
}

fn backup(file: PathBuf) -> Result<(), Error> {
    let mut backup_path = file.clone();

    if let Some(ext) = backup_path.extension() {
        let mut new_ext = ext.to_os_string();
        new_ext.push(".bak");
        backup_path.set_extension(new_ext);
    }

    info!("Backing up file {:?}", file);

    if backup_path.exists() {
        error!("Backup failed. File {:?} already exists!", backup_path);
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "The file is already backed up!",
        ));
    }

    debug!("original path: {:?}", file);
    debug!("backup path: {:?}", backup_path);
    fs::rename(file, backup_path)?;

    Ok(())
}

fn restore(file: PathBuf) -> Result<(), Error> {
    let mut restore_path = file.clone();

    if let Some(extension) = restore_path.extension() {
        if extension == "bak" {
            let stem = restore_path.file_stem().unwrap();
            let parent = restore_path.parent().unwrap();
            restore_path = parent.join(stem);
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "File doesn't contain a .bak extension!",
            ));
        }
    }

    info!("Restoring file {:?}", file);

    debug!("original path: {:?}", file);
    debug!("restore path: {:?}", restore_path);
    fs::rename(file, restore_path)?;

    Ok(())
}

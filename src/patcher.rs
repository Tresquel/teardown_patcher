use log::{debug, error, info, warn};
use serde::Deserialize;
use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::{self, File},
    io::{self, Error, ErrorKind, Read},
    path::{Path, PathBuf},
};
use zip::ZipArchive;

use crate::config::{self, Mod};

#[derive(Deserialize)]
struct Manifest {
    name: String,
    description: String,
    author: String,
}

pub fn patch() -> Result<bool, Box<dyn std::error::Error>> {
    let mut config = config::init_config()?;

    info!("patch(): Patching the game...");
    println!("Patching the game...");

    let mods_path = Path::new(".\\mods");
    if !mods_path.try_exists()? {
        warn!("list_mods(): Mods path doesn't exist, creating");
        fs::create_dir(mods_path)?;
    }

    for entry in fs::read_dir(".\\mods")? {
        let path = entry?.path();
        info!("patch(): Found path {:?}", path);

        if path.extension().unwrap_or(OsStr::new("")) != "zip" || path.is_dir() {
            warn!("patch(): {:?} isn't a zip file", path);
            continue;
        }

        debug!("patch(): Opening the zip file");
        let zip_file = File::open(&path)?;
        let mut archive = ZipArchive::new(zip_file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_name = file.enclosed_name().unwrap().clone();

            // pre checks
            if file.is_dir() {
                continue;
            }
            if file_name.ends_with("manifest.toml") {
                continue;
            }

            info!("patch(): Patching file: {:?}", &file_name);
            print!("Patching file: {:?}...", &file_name);

            // backup
            if let Err(e) = backup(config.td_path.join(&file_name)) {
                if e.kind() == ErrorKind::AlreadyExists {
                } else {
                    error!("patch(): Backup failed: {}", e);
                    println!("Backup failed: {}", e);
                    return Err(Box::new(e));
                }
            }

            // copying
            info!("patch(): Copying file");
            let mut outfile = File::create(config.td_path.join(&file_name))?;
            io::copy(&mut file, &mut outfile)?;

            info!("patch(): Successfully patched file: {:?}", &file_name);
            println!(" done");

            config.patched_files.push(file_name);
        }
    }

    config.patched_files = remove_duplicates(config.patched_files);
    config::save_config(&config)?;

    Ok(true)
}

pub fn unpatch() -> Result<bool, Box<dyn std::error::Error>> {
    let mut config = config::init_config()?;
    let patched = config.patched_files.clone();

    info!("unpatch(): Restoring the game...");
    println!("Restoring the game...");

    for file in patched {
        let mut restore_path = file.clone();

        if let Some(ext) = restore_path.extension() {
            let mut new_ext = ext.to_os_string();
            new_ext.push(".bak");
            restore_path.set_extension(new_ext);
        }

        info!("unpatch(): Restoring file: {:?}", &restore_path);
        print!("Restoring file: {:?}...", &restore_path);

        restore(config.td_path.join(restore_path.clone()))?;

        info!("unpatch(): Successfully restored file: {:?}", &restore_path);
        println!(" done");

        config.patched_files.remove(0);
    }

    config.patched_files = remove_duplicates(config.patched_files);
    config::save_config(&config)?;

    Ok(true)
}

pub fn list_mods() -> Result<Vec<Mod>, Box<dyn std::error::Error>> {
    let mut mods: Vec<Mod> = Vec::new();

    let mods_path = Path::new(".\\mods");
    if !mods_path.try_exists()? {
        warn!("list_mods(): Mods path doesn't exist, creating");
        fs::create_dir(mods_path)?;
    }

    for entry in fs::read_dir(".\\mods")? {
        let path = entry?.path();
        info!("list_mods(): Found file {:?}", path);

        if path.extension().unwrap_or(OsStr::new("")) != "zip" || path.is_dir() {
            warn!("list_mods(): {:?} isn't a zip file", path);
            continue;
        }

        let zip_file = File::open(&path)?;
        let mut archive = ZipArchive::new(zip_file)?;

        // read file
        debug!("list_mods(): Reading the manifest.toml");
        let mut manifest_file = String::new();
        if let Ok(mut v) = archive.by_name("manifest.toml") {
            v.read_to_string(&mut manifest_file)?
        } else {
            error!(
                "list_mods(): manifest.toml not found in archive {:?}",
                &path
            );
            continue;
        };

        debug!("list_mods(): Deserializing the .toml");
        // deserialize it and add it to the list
        let manifest: Manifest = toml::from_str(&manifest_file)?;
        let found_mod = Mod {
            name: manifest.name,
            description: manifest.description,
            author: manifest.author,
            path,
        };

        info!("list_mods(): Adding mod to the mods list: {:?}", found_mod);
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

    if backup_path.exists() {
        error!(
            "backup(): Backup failed. File {:?} already exists!",
            backup_path
        );
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "The file is already backed up!",
        ));
    }

    info!("backup(): Backed up file {:?}", file);
    debug!("backup(): Original path: {:?}", file);
    debug!("backup(): Backup path: {:?}", backup_path);

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
            error!("restore(): File doesn't contain a .bak extension!");
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "File doesn't contain a .bak extension!",
            ));
        }
    }

    info!("restore(): Restored file {:?}", file);
    debug!("restore(): Original path: {:?}", file);
    debug!("restore(): Restore path: {:?}", restore_path);

    fs::rename(file, restore_path)?;

    Ok(())
}

fn remove_duplicates<T: std::hash::Hash + std::cmp::Eq + Clone>(vec: Vec<T>) -> Vec<T> {
    let set: HashSet<_> = vec.into_iter().collect();
    set.into_iter().collect()
}

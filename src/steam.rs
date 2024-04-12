use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

use log::{error, info, warn};
use winreg::{enums::*, RegKey};

pub fn get_steam_path() -> Result<PathBuf, Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    match hklm.open_subkey("SOFTWARE\\Valve\\Steam") {
        Ok(v) => {
            info!("get_steam_path(): Found x86 key");
            if let Ok(v) = v.get_value::<String, &str>("InstallPath") {
                warn!("get_steam_path(): x86 doesn't have a value");
                return Ok(PathBuf::from(v));
            };
        }
        Err(e) => {
            warn!("get_steam_path(): x86 key error: {}", e);
        }
    };

    match hklm.open_subkey("SOFTWARE\\Wow6432Node\\Valve\\Steam") {
        Ok(v) => {
            info!("get_steam_path(): Found x64 key");
            let value: String = v.get_value("InstallPath")?;
            Ok(PathBuf::from(value))
        }
        Err(e) => {
            error!("get_steam_path(): x64 key error: {}", e);
            Err(e)
        }
    }
}

pub fn get_teardown_path() -> Result<PathBuf, Error> {
    let steam_path = get_steam_path()?;
    let teardown_path = steam_path.join("steamapps\\common\\Teardown");

    if teardown_path.exists() {
        info!("get_teardown_path(): Teardown folder exists");
        return Ok(teardown_path);
    }

    error!("get_teardown_path(): Teardown folder doesn't exist");
    Err(Error::new(ErrorKind::NotFound, "folder doesn't exist"))
}

pub fn check_wine() -> Result<bool, Error> {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);

    match hklm.open_subkey("SOFTWARE\\Wine") {
        Ok(_) => {
            info!("get_steam_path(): Running on Wine");
            Ok(true)
        }
        Err(e) => {
            warn!("check_wine(): Wine key error: {}", e);
            Ok(false)
        }
    }
}

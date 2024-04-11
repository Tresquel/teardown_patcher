use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

use log::error;
use winreg::{enums::*, RegKey};

pub fn get_steam_path() -> Result<PathBuf, Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let steam32_key = hklm.open_subkey("SOFTWARE\\Valve\\Steam")?;
    let steam64_key = hklm.open_subkey("SOFTWARE\\Wow6432Node\\Valve\\Steam")?;

    if let Ok(v) = steam32_key.get_value::<String, _>("InstallPath") {
        return Ok(PathBuf::from(v));
    }

    match steam64_key.get_value::<String, _>("InstallPath") {
        Ok(v) => Ok(PathBuf::from(v)),
        Err(e) => {
            error!("Error encountered while getting Steam's path: {}", e);
            Err(e)
        }
    }
}

pub fn get_teardown_path() -> Result<PathBuf, Error> {
    let steam_path = get_steam_path()?;
    let teardown_path = steam_path.join("steamapps\\common\\Teardown");

    if teardown_path.exists() {
        return Ok(teardown_path);
    }

    Err(Error::new(ErrorKind::NotFound, "folder doesn't exist"))
}

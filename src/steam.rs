use std::io::{Error, ErrorKind};
use std::path::Path;

use winreg::enums::*;
use winreg::RegKey;

pub fn get_steam_path() -> Result<String, Error> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let steam32_key = hklm.open_subkey("SOFTWARE\\Valve\\Steam")?;
    let steam64_key = hklm.open_subkey("SOFTWARE\\Wow6432Node\\Valve\\Steam")?;

    return match steam32_key.get_value("InstallPath") {
        Ok(v) => Ok(v),
        Err(_) => Ok(steam64_key.get_value("InstallPath")?),
    };
}

pub fn get_teardown_path() -> Result<String, Error> {
    let steam_path = get_steam_path()?;
    let teardown_path = format!("{}\\steamapps\\common\\Teardown", steam_path);

    if Path::new(&teardown_path).exists() {
        return Ok(teardown_path);
    }

    Err(Error::new(ErrorKind::NotFound, "folder doesn't exist"))
}

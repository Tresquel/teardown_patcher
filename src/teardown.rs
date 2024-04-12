use log::{debug, error, info, warn};
use rfd::FileDialog;

use std::{
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

use crate::steam;

pub fn ask_for_directory() -> Result<PathBuf, Error> {
    println!("We couldn't find Teardown in the Steam folder.\nPlease select the Teardown executable in the window that pops up. ");
    info!("ask_for_directory(): Asking for Teardown's exe");

    let path = std::env::current_dir().unwrap();

    loop {
        let folder = FileDialog::new()
            .add_filter("teardown", &["exe"])
            .set_file_name("teardown.exe")
            .set_directory(&path)
            .pick_file();

        if let Some(mut td_path) = folder {
            info!("ask_for_directory(): User selected {:?}", td_path);
            if check_path(&td_path)? {
                debug!("ask_for_directory(): Path is OK");
                td_path.pop();
                return Ok(td_path);
            } else {
                error!(
                    "ask_for_directory(): {:?} is not a valid executable",
                    &td_path
                );
                println!(
                    "{:?} is not a valid Teardown executable. Please select the correct one.",
                    &td_path
                );
            }
        } else {
            error!("ask_for_directory(): User didn't select a file");
            return Err(Error::new(ErrorKind::NotFound, "User didn't select a file"));
        }
    }
}

fn check_path(path: &PathBuf) -> Result<bool, Error> {
    let lf_path = Path::new(&steam::get_steam_path()?).join("steamapps/libraryfolders.vdf");
    let lf_contents = fs::read_to_string(lf_path)?;

    if !lf_contents.contains(r#""1167630""#) && !steam::check_wine()? {
        error!("check_path(): lf doesn't contain id");
        return Err(Error::new(ErrorKind::NotFound, "lf doesn't contain id"));
    }

    let td_path = Path::new(&path);

    if td_path.ends_with("steamapps/common/Teardown/teardown.exe") {
        debug!("check_path(): Path is OK");
        Ok(true)
    } else {
        warn!("check_path(): Path is not OK");
        Ok(false)
    }
}

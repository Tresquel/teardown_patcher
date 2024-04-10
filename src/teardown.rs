use log::{debug, error, info};
use rfd::FileDialog;

use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

use crate::steam;


pub fn ask_for_directory() -> Result<String, Error> {
    println!("We couldn't find Teardown in the Steam folder.\nPlease select the Teardown executable in the window that pops up. ");
    info!("Asking for Teardown's exe");

    let path = std::env::current_dir().unwrap();

    loop {
        let folder = FileDialog::new()
            .add_filter("teardown", &["exe"])
            .set_file_name("teardown.exe")
            .set_directory(&path)
            .pick_file();

        if let Some(td_path) = folder {
            debug!("User selected {:?}", td_path);
            if check_path(&td_path)? {
                return Ok(td_path
                    .into_os_string()
                    .into_string()
                    .expect("Couldn't convert into string"));
            } else {
                error!("{:?} is a invalid executable", &td_path);
                println!(
                    "{:?} is not a valid Teardown executable. Please select the correct one.",
                    &td_path
                );
            }
        } else {
            error!("User didn't select a file");
            return Err(Error::new(ErrorKind::NotFound, "User didn't select a file"));
        }
    }
}

fn check_path(path: &PathBuf) -> Result<bool, Error> {
    let lf_path = Path::new(&steam::get_steam_path()?).join("steamapps/libraryfolders.vdf");
    let lf_contents = fs::read_to_string(lf_path)?;
    
    if !lf_contents.contains(r#""1167630""#) {
        error!("lf doesn't contain id");
        return Err(Error::new(ErrorKind::NotFound, "lf doesn't contain id"));
    }
    
    let td_path = Path::new(&path);

    if td_path.ends_with("steamapps/common/Teardown/teardown.exe") {
        Ok(true)
    } else {
        Ok(false)
    }
}

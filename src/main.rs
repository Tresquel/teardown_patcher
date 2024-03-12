mod config;
mod steam;
mod teardown;

use log::{error, info, LevelFilter};
use std::env;
use std::io::Error;

fn help() {
    println!("very helpful help message")
}

fn main() -> Result<(), Error> {
    simple_logging::log_to_file("teardown_patcher.log", LevelFilter::Debug)?;
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); // remove first argument (the path)

    if args.len() < 1 {
        println!("Bad arguments(s)!");
        error!("Bad argument(s): {:?}", args);
        help();
        return Ok(());
    }

    let mut launch_game = false;

    for arg in args {
        match arg.as_str() {
            "--launch" | "-l" => {
                info!("Launching the game after all the arguments are parsed...");
                launch_game = true;
            }

            "--patch" | "-p" => {

            }

            _ => {
                error!("Unknown argument {arg}");
                eprintln!("Unknown argument {arg}");
                continue;
            }
        }
    }

    if launch_game {
        info!("Launching game...");
        println!("Launching the game...");
        //open::that_detached("steam://rungameid/1167630")?;
    }

    // let steam_path = steam::get_steam_path()?;
    // info!("Steam's path is: {steam_path}");
    //
    // println!("Steam's path is: {steam_path}");
    //
    // #[allow(unused_assignments)]
    // let mut td_path = String::new();
    // match steam::get_teardown_path() {
    //     Ok(v) => {
    //         td_path = v;
    //     }
    //     Err(_) => {
    //         td_path = teardown::ask_for_directory()?;
    //     }
    // }
    // info!("Teardown's path is: {td_path}");
    // println!("Teardown's path is: {td_path}\n\n");
    //
    // let mut cfg = config::get_config()?;
    //
    // if cfg.td_path == "" {
    //     println!("Config was empty, setting the correct values");
    //     cfg.steam_path = steam_path;
    //     cfg.td_path = td_path;
    //     config::save_config(cfg)?;
    //     println!("Config saved");
    // } else {
    //     println!("Reading from config:");
    //     println!(
    //         "Steam's path is: {}\nTeardown's path is: {}",
    //         cfg.steam_path, cfg.td_path
    //     );
    // }

    Ok(())
}

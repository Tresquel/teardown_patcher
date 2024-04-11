mod config;
mod patcher;
mod steam;
mod teardown;

use log::{error, info, LevelFilter};
use std::env;
#[cfg(debug_assertions)]
use std::fs;

fn help() {
    println!(
        "usage:
--launch | -l
    Launches the game thru Steam
--patch | -p
    Patches the game with the mods provided in the ./mods folder
--restore | -r
    Restores base game files
--list | -L
    Lists all mods and their info
--help | -h
    Displays this"
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logging::log_to_file("teardown_patcher.log", LevelFilter::Debug)?;

    let mut args: Vec<String> = env::args().collect();
    info!("main(): Ran with arguments: {:?}", args);
    args.remove(0); // remove first argument (the path)

    if args.is_empty() {
        println!("No arguments provided!");
        error!("main(): No arguments provided!");
        help();
        return Ok(());
    }

    let mut launch_game = false;

    for arg in args {
        match arg.as_str() {
            "--launch" | "-l" => {
                info!("main(): Launching the game after all the arguments are parsed...");
                launch_game = true;
            }

            "--patch" | "-p" => {
                if let Err(e) = patcher::patch() {
                    error!("main(): Patching has encountered an error! '{}'", e);
                    println!("Patching has encountered an error! '{}', stopping..", e);
                    return Err(e);
                }
            }

            "--restore" | "-r" => {
                if let Err(e) = patcher::unpatch() {
                    error!("main(): Restoring has encountered an error! '{}'", e);
                    println!("Restoring has encountered an error! '{}', stopping..", e);
                    return Err(e);
                }
            }

            "--list" | "-L" => {
                let mods = patcher::list_mods()?;
                for found_mod in mods {
                    println!("{:?}:", found_mod.path);
                    println!("  - Name: {}", found_mod.name);
                    println!("  - Description: {}", found_mod.description);
                    println!("  - Made by: {}", found_mod.author);
                }
            }

            "--help" | "-h" => {
                help();
            }

            #[cfg(debug_assertions)]
            "--config-reset" | "-R" => {
                info!("main(): Removing tdcfg file");
                println!("Removing tdcfg file");
                fs::remove_file("patcher.tdcfg")?;
            }

            _ => {
                error!("main(): Unknown argument {arg}");
                eprintln!("Unknown argument {arg}");
                continue;
            }
        }
    }

    if launch_game {
        info!("main(): Launching game...");
        println!("Launching the game...");
        open::that_detached("steam://rungameid/1167630")?;
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

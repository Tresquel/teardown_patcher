mod config;
mod patcher;
mod steam;
mod teardown;

use eframe::egui::{self, ScrollArea};
use log::{error, info, warn, LevelFilter};
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
    Displays this
--version | -v
    Prints out the current version installed"
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logging::log_to_file("teardown_patcher.log", LevelFilter::Debug)?;

    info!("teardown_patcher version: '{}'", env!("CARGO_PKG_VERSION"));

    let mut args: Vec<String> = env::args().collect();
    info!("main(): Ran with arguments: {:?}", args);
    args.remove(0); // remove first argument (the path)

    if args.is_empty() {
        warn!("main(): No arguments provided, starting ui");
        ui()?;
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
                    eprintln!("Patching has encountered an error! '{}', stopping..", e);
                    return Err(e);
                }
            }

            "--restore" | "-r" => {
                if let Err(e) = patcher::unpatch() {
                    error!("main(): Restoring has encountered an error! '{}'", e);
                    eprintln!("Restoring has encountered an error! '{}', stopping..", e);
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

            "--version" | "-v" => {
                println!("Teardown Patcher version '{}'", env!("CARGO_PKG_VERSION"));
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
        teardown::launch()?;
    }

    Ok(())
}

fn ui() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 320.0]),
        ..Default::default()
    };

    let mut patched = false;
    let mut restored = false;

    eframe::run_simple_native("Teardown Patcher", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Teardown Patcher {}", env!("CARGO_PKG_VERSION")));

            ui.horizontal(|ui| {
                if ui.button("Patch game").clicked() {
                    if let Err(e) = patcher::patch() {
                        error!("ui(): Patching has encountered an error! '{}'", e);
                        eprintln!("Patching has encountered an error! '{}', stopping..", e);
                        return Err(e);
                    }
                    patched = true;
                }

                if patched {
                    restored = false;
                    ui.label("Done!");
                }

                Ok(())
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                if ui.button("Restore game").clicked() {
                    if let Err(e) = patcher::unpatch() {
                        error!("ui(): Restoring has encountered an error! '{}'", e);
                        eprintln!("Restoring has encountered an error! '{}', stopping..", e);
                        return Err(e);
                    }
                    restored = true;
                }

                if restored {
                    patched = false;
                    ui.label("Done!");
                }

                Ok(())
            });

            ui.add_space(10.0);

            ui.label("Mods:");

            ui.separator();
            ScrollArea::vertical()
                .auto_shrink(false)
                .max_height(128.0)
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                        |ui| {
                            let mods = patcher::list_mods().unwrap();
                            for found_mod in mods {
                                ui.collapsing(found_mod.name, |ui| {
                                    ui.label(found_mod.description);
                                    ui.label(format!("Made by: {}", found_mod.author));
                                    ui.label(format!("File: {:?}", found_mod.path));
                                });
                            }
                        },
                    );
                });

            ui.separator();

            if ui.button("Launch Teardown").clicked() {
                let _ = teardown::launch();
                std::process::exit(0);
            }

            ui.with_layout(
                egui::Layout::bottom_up(egui::Align::BOTTOM).with_cross_justify(true),
                |ui| {
                    use egui::special_emojis::GITHUB;
                    ui.hyperlink_to(
                        format!("{GITHUB} View on github"),
                        "https://github.com/Tresquel/teardown_patcher",
                    );
                },
            )
        });
    })?;

    Ok(())
}

# Teardown patcher
Makes modifying game files easy

# How does it work?
It takes all the files inside a zip file and copies them into the Teardown folder while also keeping a backup of the original files.

# Usage
- `--launch | -l`
    - Launches the game thru Steam
- `--patch | -p`
    - Patches the game with the mods provided in the ./mods folder
- `--restore | -r`
    - Restores base game files
- `--list | -L`
    - Lists all mods and their info
- `--help | -h`
    - Displays the help page
- `--version | -v`
    - Displays the version currently running
  
# How do I make my mod work with this?
You need a `manifest.toml` file inside of the zip file that looks like this:
```toml
name = "Name of the mod"
description = "The description"
author = "Author(s)"
```
Your zip file should also imitate Teardown's folder structure, so for example if you're making a mod that replaces the `splash.lua` file in the `/data/ui` folder, your zip file should look like this:
```
my_mod:
    - data
        - ui
            - splash.lua
    - manifest.toml
```
I also included a [sample mod](/mods/splash_skip.zip) if you need it
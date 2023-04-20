///! Config system is currently only updated on the startup of the application and when the "start" button is pressed. This is to minimize unnecessary reads and writes to the config file.
use std::{
    fs::File,
    io::{Read, Write},
};

use crate::Args;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub kill_dwm: bool,
    pub kill_explorer: bool,
    pub disable_idle: bool,
    pub timer_resolution: f64,
}

impl Config {
    pub fn default() -> Config {
        Config {
            kill_dwm: false,
            kill_explorer: true,
            disable_idle: false,
            timer_resolution: 1.0,
        }
    }

    pub fn read() -> Config {
        // if using command arguments, dont read from the config file
        let args = Args::parse();
        if args != Args::default() {
            let config = Config {
                kill_dwm: args.kill_dwm,
                kill_explorer: args.kill_explorer,
                disable_idle: args.disable_idle,
                timer_resolution: args.timer_resolution,
            };
            return config;
        }
        let file = File::open("gameutil.toml");
        match file {
            Err(_) => {
                let config = Config::default();
                config.write().expect("Failed to write config file!");
                config
            }
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("Failed to read config file!");
                let mut config: Config =
                    toml::from_str(&contents).expect("Failed to parse config file!");
                // Prevent both from being true, dwm kills explorer already
                if config.kill_dwm && config.kill_explorer {
                    config.kill_explorer = false;
                }
                config
            }
        }
    }

    pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create("gameutil.toml")?;
        let serialized = toml::to_string(&self)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}

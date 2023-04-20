mod config;
mod gui;

use crate::config::Config;
use clap::Parser;
use std::fs::File;

#[derive(Parser, PartialEq)]
#[command(version, about, long_about = None)]
struct Args {
    /// Automatically start gameutil's tasks
    //#[arg(short, long, default_value_t = false)]
    //autostart: bool,

    /// Whether or not to kill dwm (true/false)
    #[arg(long, default_value_t = false)]
    kill_dwm: bool,

    /// Whether or not to kill explorer (true/false)
    #[arg(long, default_value_t = true)]
    kill_explorer: bool,

    /// Whether or not to disable idle (true/false)
    #[arg(long, default_value_t = false)]
    disable_idle: bool,

    /// Set the global timer resolution (doesn't work on Windows 2004+) (true/false)
    #[arg(long, default_value_t = 1.0)]
    timer_resolution: f64,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            kill_dwm: false,
            kill_explorer: true,
            disable_idle: false,
            timer_resolution: 1.0,
        }
    }
}

fn main() {
    let file = File::open("gameutil.toml");
    match file {
        Err(_) => {
            let config = Config::default();
            config.write().expect("Failed to write config file!");
        }
        _ => {}
    }

    let hotkey = gui::sys::hotkey();
    gui::gui_init();
    hotkey
        .unregister(livesplit_hotkey::KeyCode::F4)
        .expect("failed to unregister hotkey!");
}

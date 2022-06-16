#![windows_subsystem = "windows"]

mod config;
mod gui;

use std::fs::File;

use crate::config::Config;

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

#![windows_subsystem = "windows"]

mod gui;

fn main() {
    let hotkey = gui::sys::hotkey();
    gui::gui_init();
    hotkey.unregister(livesplit_hotkey::KeyCode::F4).expect("failed to unregister hotkey!");
}
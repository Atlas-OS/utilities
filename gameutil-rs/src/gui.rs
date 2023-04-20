use std::rc::Rc;

use nwg::{CheckBoxState, NativeUi};

use crate::config::Config;

pub mod sys;

#[derive(Default)]
pub struct GameUtil {
    window: nwg::Window,
    layout: nwg::GridLayout,
    timer_resolution: nwg::TextInput,
    start_button: nwg::Button,
    clean_button: nwg::Button,
    disableidle_button: nwg::CheckBox,
    kill_dwm: nwg::CheckBox,
    kill_explorer: nwg::CheckBox,
    timerresval_label: nwg::Label,
    timer_tooltip: nwg::Tooltip,
    idle_tooltip: nwg::Tooltip,
    clean_tooltip: nwg::Tooltip,
}
mod app_gui {
    use nwg::CheckBoxState;

    use super::*;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;
    pub struct GameUtilUi {
        inner: Rc<GameUtil>,
        default_handler: RefCell<Option<nwg::EventHandler>>,
    }
    impl nwg::NativeUi<GameUtilUi> for GameUtil {
        fn build_ui(mut data: GameUtil) -> Result<GameUtilUi, nwg::NwgError> {
            use nwg::Event as E;

            let config: Config = Config::read();

            let mut font = nwg::Font::default();
            nwg::Font::builder()
                .family("MS Shell Dlg 2")
                .size(16)
                .build(&mut font)
                .expect("Failed to build font");
            nwg::Font::set_global_default(Some(font));

            // Controls
            nwg::Window::builder()
                .flags(
                    nwg::WindowFlags::WINDOW
                        | nwg::WindowFlags::VISIBLE
                        | nwg::WindowFlags::MINIMIZE_BOX,
                )
                .size((250, 125))
                .title("GameUtil")
                .build(&mut data.window)?;

            // give text next to number select
            nwg::Label::builder()
                .text("Timer Resolution:")
                .parent(&data.window)
                .build(&mut data.timerresval_label)?;

            nwg::TextInput::builder()
                .text(&format!("{}", config.timer_resolution))
                .limit(3)
                .parent(&data.window)
                .build(&mut data.timer_resolution)?;

            // Disable Idle Option
            nwg::CheckBox::builder()
                .text("Disable Idle")
                //.position((10, 80))
                .parent(&data.window)
                .check_state(bool_to_checkbox_state(config.disable_idle))
                .build(&mut data.disableidle_button)?;

            nwg::Button::builder()
                .text("Start")
                .parent(&data.window)
                .build(&mut data.start_button)?;

            // radio button for kill type
            nwg::CheckBox::builder()
                .text("Kill DWM")
                .check_state(bool_to_checkbox_state(config.kill_dwm))
                .parent(&data.window)
                .build(&mut data.kill_dwm)?;

            // radio button for kill type
            nwg::CheckBox::builder()
                .text("Kill Explorer")
                .check_state(bool_to_checkbox_state(config.kill_explorer))
                .parent(&data.window)
                .build(&mut data.kill_explorer)?;

            nwg::Button::builder()
                .text("Clean Memory")
                .parent(&data.window)
                .build(&mut data.clean_button)?;

            nwg::Tooltip::builder()
                .register(
                    &*&mut data.timer_resolution,
                    "Has no effect on Windows 2004+, 0.0 to disable.",
                )
                .build(&mut data.timer_tooltip)?;

            nwg::Tooltip::builder()
                .register(&*&mut data.disableidle_button, "Disables the system idle process. Taskmgr will display CPU usage as 100%, is it not actually under 100% load.")
                .build(&mut data.idle_tooltip)?;

            nwg::Tooltip::builder()
                .register(&*&mut data.clean_button, "Hotkey: F4. Cleans the working set of all processes. Can cause a slight stutter after clicking so if using in-game run it when you are safe.")
                .build(&mut data.clean_tooltip)?;

            // Wrap-up
            let ui = GameUtilUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle | {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => {
                            if handle == ui.start_button {
                                // TODO: switch case
                                if ui.start_button.text() == "Start" {
                                    start(ui.clone());
                                } else {
                                    restore(ui.clone());
                                }
                            }
                            if handle == ui.clean_button {
                                sys::cleanworkingset();
                            }
                            if handle == ui.disableidle_button {
                                // "Prevent" from changing checkbox when gameutil is running
                                if ui.start_button.text() == "Restore" {
                                    if ui.disableidle_button.check_state()
                                        == nwg::CheckBoxState::Checked
                                    {
                                        ui.disableidle_button
                                            .set_check_state(nwg::CheckBoxState::Unchecked);
                                    } else {
                                        ui.disableidle_button
                                            .set_check_state(nwg::CheckBoxState::Checked);
                                    }
                                }
                            }
                            if handle == ui.kill_dwm {
                                if ui.kill_dwm.check_state() == CheckBoxState::Checked {
                                    ui.kill_explorer.set_check_state(CheckBoxState::Unchecked);
                                }
                            }
                            if handle == ui.kill_explorer {
                                if ui.kill_explorer.check_state() == CheckBoxState::Checked {
                                    ui.kill_dwm.set_check_state(CheckBoxState::Unchecked);
                                }
                            }
                        }
                        E::OnWindowClose => {
                            if handle == ui.window {
                                if ui.start_button.text() == "Restore" {
                                    restore(ui.clone());
                                }
                                nwg::stop_thread_dispatch();
                            }
                        }
                        E::OnTextInput => {
                            if handle == ui.timer_resolution {
                                // make sure numbers only
                                #[allow(unused_variables)]
                                // don't check for incorrect types if input is empty
                                if !ui.timer_resolution.text().is_empty() {
                                    if let Err(num) = ui.timer_resolution.text().parse::<f32>() {
                                        // warning message
                                        //nwg::modal_info_message(&ui.window, "Error", &format!("{} is not a valid number", ui.timer_resolution.text()));

                                        // filter to only numbers
                                        let timer_resolution: String = ui
                                            .timer_resolution
                                            .text()
                                            .chars()
                                            .filter(|&c| c.is_numeric() || c == '.')
                                            .collect::<String>();
                                        ui.timer_resolution.set_text(&timer_resolution);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            };
            *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
                &ui.window.handle,
                handle_events,
            ));
            // Layouts
            nwg::GridLayout::builder()
                .parent(&ui.window)
                .spacing(1)
                .child(0, 0, &ui.timerresval_label)
                .child(1, 0, &ui.timer_resolution)
                .child(0, 1, &ui.disableidle_button)
                .child(1, 3, &ui.clean_button)
                .child(0, 2, &ui.kill_dwm)
                .child(1, 2, &ui.kill_explorer)
                .child_item(nwg::GridLayoutItem::new(&ui.start_button, 0, 3, 1, 1))
                .build(&ui.layout)?;
            return Ok(ui);
        }
    }

    impl Drop for GameUtilUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }
    impl Deref for GameUtilUi {
        type Target = GameUtil;
        fn deref(&self) -> &GameUtil {
            &self.inner
        }
    }
}

pub fn gui_init() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _ui = GameUtil::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

fn restore(ui: Rc<GameUtil>) {
    // rename button to Start
    ui.start_button.set_text("Start");
    // disallow chaning settings while running
    ui.kill_dwm.set_enabled(true);
    ui.kill_explorer.set_enabled(true);
    if ui.disableidle_button.check_state() == nwg::CheckBoxState::Checked {
        sys::idle(0);
    }
    if ui.kill_dwm.check_state() == CheckBoxState::Checked {
        sys::resumeproc("winlogon.exe");
        sys::startproc("explorer.exe");
    } else if ui.kill_explorer.check_state() == CheckBoxState::Checked {
        sys::startproc("explorer.exe");
    }
    ui.timer_resolution.set_readonly(false);
}

fn start(ui: Rc<GameUtil>) {
    let mut config: Config = Config {
        kill_dwm: false,
        kill_explorer: false,
        disable_idle: false,
        timer_resolution: 1.0,
    };
    // rename button to Restore
    ui.start_button.set_text("Restore");
    // Lock settings so there is proper restoration
    ui.kill_dwm.set_enabled(false);
    ui.kill_explorer.set_enabled(false);
    if ui.disableidle_button.check_state() == nwg::CheckBoxState::Checked {
        sys::idle(1);
        config.disable_idle = true;
    }
    if ui.kill_dwm.check_state() == CheckBoxState::Checked {
        sys::killdwm();
        config.kill_dwm = true;
        config.kill_explorer = false;
    } else if ui.kill_explorer.check_state() == CheckBoxState::Checked {
        sys::taskkill("explorer.exe");
        config.kill_explorer = true;
        config.kill_dwm = false;
    }
    if ui.timer_resolution.text().parse::<f32>().unwrap() != 0.0 {
        let resolution: f64 = ui.timer_resolution.text().parse::<f64>().unwrap();
        sys::timerres((resolution * 10000.0) as u32);
        config.timer_resolution = resolution;
        ui.timer_resolution.set_readonly(true);
    }
    config.write().unwrap();
}

fn bool_to_checkbox_state(b: bool) -> CheckBoxState {
    match b {
        true => CheckBoxState::Checked,
        false => CheckBoxState::Unchecked,
    }
}

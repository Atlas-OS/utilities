use nwg::NativeUi;

mod sys;

#[derive(Default)]
pub struct GameUtil {
    window: nwg::Window,
    layout: nwg::GridLayout,
    timerresval: nwg::TextInput,
    start_button: nwg::Button,
    clean_button: nwg::Button,
    disableidle_button: nwg::CheckBox,
    kill_dwm: nwg::RadioButton,
    kill_exp: nwg::RadioButton,
    timerresval_label: nwg::Label,
}
mod app_gui {
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

            let mut font = nwg::Font::default();
            nwg::Font::builder()
                .family("MS Shell Dlg 2")
                .size(16)
                .build(&mut font)
                .expect("Failed to build font");
            nwg::Font::set_global_default(windows_dll::Option::Some(font));

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
                .text("5000")
                .limit(5)
                .parent(&data.window)
                .build(&mut data.timerresval)?;

            // Disable Idle Option
            nwg::CheckBox::builder()
                .text("Disable Idle")
                //.position((10, 80))
                .parent(&data.window)
                .build(&mut data.disableidle_button)?;

            nwg::Button::builder()
                .text("Start")
                .parent(&data.window)
                .build(&mut data.start_button)?;

            // radio button for kill type
            nwg::RadioButton::builder()
                .text("Kill DWM")
                .check_state(nwg::RadioButtonState::Checked)
                .parent(&data.window)
                .build(&mut data.kill_dwm)?;

            // radio button for kill type
            nwg::RadioButton::builder()
                .text("Kill Explorer")
                .check_state(nwg::RadioButtonState::Unchecked)
                .parent(&data.window)
                .build(&mut data.kill_exp)?;

            nwg::Button::builder()
                .text("Clean Memory")
                .parent(&data.window)
                .build(&mut data.clean_button)?;
            // Wrap-up
            let ui = GameUtilUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };
            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => {
                            if handle == ui.start_button {
                                if ui.start_button.text() == "Start" {
                                    // rename button to Restore
                                    ui.start_button.set_text("Restore");
                                    if ui.disableidle_button.check_state()
                                        == nwg::CheckBoxState::Checked
                                    {
                                        sys::idle(1);
                                    }
                                    if ui.kill_dwm.check_state() == nwg::RadioButtonState::Checked {
                                        sys::killdwm();
                                    } else {
                                        sys::taskkill("explorer.exe");
                                    }
                                    if ui.timerresval.text().parse::<u32>().unwrap() != 0 {
                                        //timerresval_copy = ui.timerresval.text().parse::<u32>().unwrap();
                                        sys::timerres(
                                            //timerresval_copy,
                                            ui.timerresval.text().parse::<u32>().unwrap(),
                                        );
                                    }
                                } else {
                                    // rename button to Start
                                    ui.start_button.set_text("Start");
                                    if ui.disableidle_button.check_state()
                                        == nwg::CheckBoxState::Checked
                                    {
                                        sys::idle(0);
                                    }
                                    if ui.kill_dwm.check_state() == nwg::RadioButtonState::Checked {
                                        sys::resumeproc("winlogon.exe");
                                        sys::startproc("explorer.exe");
                                    } else {
                                        sys::startproc("explorer.exe");
                                    }
                                    // change for button implementation
                                }
                            }
                            if handle == ui.clean_button {
                                sys::cleanworkingset();
                            }
                            if handle == ui.disableidle_button {
                                // prevent changing setting while running
                                if ui.start_button.text() == "Restore" {
                                    if ui.disableidle_button.check_state()
                                        == nwg::CheckBoxState::Unchecked
                                    {
                                        ui.disableidle_button
                                            .set_check_state(nwg::CheckBoxState::Checked);
                                    } else {
                                        ui.disableidle_button
                                            .set_check_state(nwg::CheckBoxState::Unchecked);
                                    }
                                }
                            }
                        }
                        E::OnWindowClose => {
                            if handle == ui.window {
                                nwg::stop_thread_dispatch();
                            }
                        }
                        E::OnTextInput => {
                            if handle == ui.timerresval {
                                /*if ui.start_button.text() == "Restore" {
                                    ui.timerresval.set_text(&timerresval_copy.to_string());
                                }
                                */
                                // make sure numbers only
                                #[allow(unused_variables)]
                                // don't check for incorrect types if input is empty
                                if !ui.timerresval.text().is_empty() {
                                    if let Err(num) = ui.timerresval.text().parse::<u32>() {
                                        // warn message
                                        //nwg::modal_info_message(&ui.window, "Error", &format!("{} is not a valid number", ui.timerresval.text()));

                                        // remove last typed character in text input
                                        //ui.timerresval.set_text(&format!("{}", ui.timerresval.text().chars().take(ui.timerresval.text().chars().count() - 1).collect::<String>()));
                                        // filter to only numbers
                                        ui.timerresval.set_text(
                                            &ui.timerresval
                                                .text()
                                                .chars()
                                                .filter(|&c| c.is_numeric())
                                                .collect::<String>(),
                                        );
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
                .child(1, 0, &ui.timerresval)
                .child(0, 1, &ui.disableidle_button)
                .child(1, 3, &ui.clean_button)
                .child(0, 2, &ui.kill_dwm)
                .child(1, 2, &ui.kill_exp)
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

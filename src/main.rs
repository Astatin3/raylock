use eframe::egui;
use egui::Key;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod input;
mod structs;
mod ui;

#[derive(Default)]
struct ExampleApp {
    auth_state: Arc<Mutex<structs::AuthState>>,
}

impl ExampleApp {
    fn name() -> &'static str {
        "raylock"
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut state = self.auth_state.lock().unwrap();
        ctx.set_pixels_per_point(1.5);

        if ctx.input(|i| i.events.len() > 0) {
            ctx.input(|i| {
                for event in &i.events {
                    if let egui::Event::Key {
                        key,
                        pressed,
                        modifiers,
                        ..
                    } = event
                    {
                        if *pressed {
                            match key {
                                Key::Enter => {
                                    state.to_be_submitted = true;
                                }
                                Key::Backspace => {
                                    let len = state.password.len();
                                    if len != 0 {
                                        state.password.remove(len - 1 as usize);
                                    }
                                }
                                _ => {
                                    let str = &input::format_key(*key, modifiers.shift);
                                    state.password += str;
                                }
                            }
                            //let mod_str = if modifiers.is_empty() {
                            //    String::new()
                            //} else {
                            //    format!(" + {:?}", modifiers)
                            //};
                        }
                    }
                }
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.add_space(200.0);
            // ui.heading("*".repeat(state.password.clone().len()));
            // ui.add_space(20.0);
            //
            ui::update(state, ctx, _frame, ui);
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((400.0, 400.0)),
        ..eframe::NativeOptions::default()
    };

    let state = Arc::new(Mutex::new(structs::AuthState {
        password: String::new(),
        to_be_submitted: false,
        failed_attempts: 0,
    }));

    let auth_state_clone = state.clone();

    // Spawn authentication thread
    thread::spawn(move || loop {
        let mut state = auth_state_clone.lock().unwrap();

        if state.to_be_submitted {
            let result = try_sudo(&state.password);
            match result {
                Ok(true) => {
                    println!("True");
                    input::sway_unlock_input();
                    std::process::exit(0);
                }
                Ok(false) => {
                    println!("False");
                    state.failed_attempts += 1;
                    state.password.clear();
                }
                Err(_) => {
                    state.password.clear();
                }
            }
            state.to_be_submitted = false;
        }
        drop(state);
        thread::sleep(Duration::from_millis(100));
    });

    input::sway_lock_input();

    eframe::run_native(
        ExampleApp::name(),
        native_options,
        Box::new(|_| Ok(Box::<ExampleApp>::new(ExampleApp { auth_state: state }))),
    )
}

fn try_sudo(password: &str) -> Result<bool, std::io::Error> {
    let mut child = Command::new("sudo")
        .args(["-kS", "true"]) // Use -S to read password from stdin
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        writeln!(stdin, "{}", password)?;
    }

    match child.wait() {
        Ok(status) => Ok(status.success()),
        Err(e) => Err(e),
    }
}

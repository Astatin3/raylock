use configs::SCREEN_HEIGHT;
use eframe::Error;
use eframe::{egui, App};
use egui::FontFamily;
use egui::Key;
// use graph::CpuGraph;
use input::is_locked;
use panes::Pane;
// use serde::de::Error;
// use rand::Error;
// use panes::{PaneConfig, PaneRenderer, SplitDirection};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
// use egui::epaint::text::{FontInsert, InsertFontFamily};

use eframe::CreationContext;
use egui::ecolor::Color32;
use egui::FontId;
use egui::Stroke;
// use egui_terminal::prelude::*;
// use egui_terminal::render::CursorType;

use egui::ecolor::HexColor;

mod configs;

mod cpugraph;
mod diskgraph;
mod graph;
mod memgraph;
mod netgraph;

mod table;

mod infopane;

mod input;
mod panes;
mod structs;
mod ui;

use panes::{load_pane_config, EXAMPLE_CONFIG};

struct ExampleApp {
    auth_state: Arc<Mutex<structs::AuthState>>,
    root_pane: panes::PaneInstance,
}

impl ExampleApp {
    fn name() -> &'static str {
        "raylock"
    }
}

impl eframe::App for ExampleApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::from_rgba_unmultiplied(0.1, 0.1, 0.1, 0.9).to_array()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut state = self.auth_state.lock().unwrap();
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
                        }
                    }
                }
            });
        }

        // let mut pane = ;
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                std::thread::sleep(Duration::from_millis(50));
                ui::update(state, ctx, _frame, ui, &mut self.root_pane);
                ctx.request_repaint();
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

    thread::spawn(move || loop {
        let mut state = auth_state_clone.lock().unwrap();

        if state.to_be_submitted {
            let result = try_sudo(&state.password);
            match result {
                Ok(true) => {
                    // println!("True");
                    input::sway_unlock_input();
                    input::remove_lock();
                    std::process::exit(0);
                }
                Ok(false) => {
                    // println!("False");
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

    if input::is_locked() {
        println!("Raylock is already running!");
        std::process::exit(1);
    }

    input::sway_lock_input();
    input::create_lock();

    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        input::sway_unlock_input();
        input::remove_lock();
        default_panic(info);
    }));

    let mut pane_config = load_pane_config(EXAMPLE_CONFIG).unwrap();
    pane_config.precalc(egui::Rect {
        min: egui::Pos2 { x: 0., y: 0. },
        max: egui::Pos2 {
            x: configs::SCREEN_WIDTH,
            y: configs::SCREEN_HEIGHT,
        },
    });

    eframe::run_native(
        ExampleApp::name(),
        native_options,
        Box::new(|_| {
            Ok(Box::<ExampleApp>::new(ExampleApp {
                auth_state: state,
                root_pane: pane_config,
                // cpu_graph: CpuGraph::new(),
            }))
        }),
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

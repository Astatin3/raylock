use eframe::{egui, App};
use egui::FontFamily;
use egui::Key;
use graph::CpuGraph;
use input::is_locked;
use panes::{PaneConfig, PaneRenderer, SplitDirection};
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
mod graph;
mod input;
mod panes;
mod structs;
mod ui;

// Demonstrates how to add a font to the existing ones
// fn add_font(ctx: &egui::Context) {
//     // Start with the default fonts (we will be adding to them rather than replacing them).
//     let mut fonts = egui::FontDefinitions::default();

//     // Install my own font (maybe supporting non-latin characters).
//     // .ttf and .otf files supported.
//     fonts.font_data.insert(
//         "my_font".to_owned(),
//         egui::FontData::from_static(include_bytes!(
//             "/home/astatin3/.fonts/UbuntuMono/UbuntuMonoNerdFontMono-Regular.ttf"
//         )),
//     );

//     // Put my font first (highest priority) for proportional text:
//     // fonts
//     //     .families
//     //     .entry(egui::FontFamily::Proportional)
//     //     .or_default()
//     //     .insert(0, "my_font".to_owned());

//     // Put my font as last fallback for monospace:
//     fonts
//         .families
//         .entry(egui::FontFamily::Monospace)
//         .or_default()
//         .push("my_font".to_owned());

//     // Tell egui to use these fonts:
//     ctx.set_fonts(fonts);
// }

// #[derive(Default)]
struct ExampleApp {
    auth_state: Arc<Mutex<structs::AuthState>>,
    cpu_graph: CpuGraph,
    // terminals: HashMap<String, TermHandler>,
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
        // let s1 = self.terminals.get_mut("0").unwrap();
        // s1.style()
        // s1.cursor_trail = false;
        // s1.cursor_trail_color = Some(HexColor::Hex8(Color32::LIGHT_BLUE.gamma_multiply(0.5)));

        // s1.default_focus_cursor = CursorType::OpenBlock(HexColor::Hex8(Color32::RED));
        // s1.default_unfocus_cursor = CursorType::None;
        // for str in egui::FontDefinitions::builtin_font_names() {
        //     println!("{}", str);
        // }
        // s1.cursor_stroke = Stroke::new(1., Color32::WHITE);

        // add_font(ctx);

        // let font = FontId {
        //     size: 9.,
        //     family: FontFamily::Name("my_font".into()),
        // };

        // s1.font = font;

        let mut state = self.auth_state.lock().unwrap();
        //ctx.set_pixels_per_point(1.5);
        // let config = ui::winconfig();

        let config: panes::LayoutConfig = serde_json::from_str(configs::EXAMPLE_CONFIG).unwrap();
        let mut pane_renderer = PaneRenderer::new(config);

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

        // let sine_points = PlotPoints::from_explicit_callback(|x| x.sin(), .., 5000);
        // let plot_a_lines = [Line::new(sine_points).name("Sine")];

        // let sine_points2 = PlotPoints::from_explicit_callback(|x| x.sin(), .., 5000);
        // let plot_b_lines = [Line::new(sine_points2).name("Sine")];

        // self.cpu_graph.update();

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                // let ht = ui.available_height();
                // for (_idx, (_id, term)) in self.terminals.iter_mut().enumerate() {
                //     ui.terminal_sized(term, egui::vec2(ui.available_width(), ht));
                // }
                // ui.add_space(200.0);
                // ui.heading("*".repeat(state.password.clone().len()));
                // ui.add_space(20.0);
                //

                // ui.terminal_sized(
                //     self.terminals.get_mut("0").unwrap(),
                //     egui::vec2(ui.available_width(), ui.available_height()),
                // );
                // Splitter::new("some_unique_id", splitter::SplitterAxis::Horizontal).show(
                //     ui,
                //     |ui_a, ui_b| {
                //         ui_a.terminal_sized(
                //             self.terminals.get_mut("0").unwrap(),
                //             egui::vec2(ui_a.available_width(), ui_a.available_height()),
                //         );

                //         ui_b.terminal_sized(
                //             self.terminals.get_mut("1h-1").unwrap(),
                //             egui::vec2(ui_a.available_width(), ui_a.available_height()),
                //         );

                //     },
                // );

                ui::update_password_viewer(state, ctx, _frame, ui, &mut pane_renderer);

                // self.cpu_graph.render(
                //     ui.painter(),
                //     egui::Rect {
                //         min: egui::Pos2 { x: 0., y: 0. },
                //         max: egui::Pos2 { x: 500., y: 500. },
                //     },
                // );

                ctx.request_repaint();
            });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((400.0, 400.0)),
        ..eframe::NativeOptions::default()
    };

    // let system_shell = std::env::var("SHELL")
    //     .expect("SHELL variable is not defined")
    //     .to_string();

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

    // let mut map = HashMap::new();
    // map.insert(String::from("0"), TermHandler::new_from_str("btop"));
    // map.insert(String::from("1h-0"), TermHandler::new_from_str("nvitop"));
    // map.insert(String::from("1h-1"), TermHandler::new_from_str("neofetch"));
    input::sway_lock_input();

    if input::is_locked() {
        println!("Raylock is already running!");
        std::process::exit(1);
    }

    let test = [1, 2, 3];

    input::create_lock();

    eframe::run_native(
        ExampleApp::name(),
        native_options,
        Box::new(|_| {
            Ok(Box::<ExampleApp>::new(ExampleApp {
                auth_state: state,
                cpu_graph: CpuGraph::new(),
                // terminals: map,
            }))
        }),
    )
}

fn test_test(test: &mut [i32]) {
    for i in 0..test.len() {
        test[i] += 1;
    }
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

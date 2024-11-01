use egui::Key;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::thread;

const LOCK_FILEPATH: &str = "/tmp/.raylock.lock";

pub fn sway_lock_input() {
    thread::spawn(move || {
        let _ = Command::new("swaymsg").args(["mode", "lock"]).spawn();
        // let _ = Command::new("swaymsg")
        //     .args(["input", "type:touchpad", "events", "disabled"])
        //     .spawn();
    });
}

pub fn sway_unlock_input() {
    thread::spawn(move || {
        let _ = Command::new("swaymsg").args(["mode", "default"]).spawn();
        // let _ = Command::new("swaymsg")
        //     .args(["input", "type:touchpad", "events", "enabled"])
        //     .spawn();
    });
}

pub fn create_lock() {
    let _ = File::create(LOCK_FILEPATH).unwrap();
}

pub fn is_locked() -> bool {
    Path::exists(Path::new(LOCK_FILEPATH))
}

pub fn remove_lock() {
    if !is_locked() {
        return;
    }

    std::fs::remove_file(Path::new(LOCK_FILEPATH));
}

pub fn format_key(key: Key, shift_pressed: bool) -> String {
    match key {
        // Letters
        Key::A
        | Key::B
        | Key::C
        | Key::D
        | Key::E
        | Key::F
        | Key::G
        | Key::H
        | Key::I
        | Key::J
        | Key::K
        | Key::L
        | Key::M
        | Key::N
        | Key::O
        | Key::P
        | Key::Q
        | Key::R
        | Key::S
        | Key::T
        | Key::U
        | Key::V
        | Key::W
        | Key::X
        | Key::Y
        | Key::Z => {
            let base_char = (key as u8 - Key::A as u8 + b'a') as char;
            if shift_pressed {
                base_char.to_uppercase().to_string()
            } else {
                base_char.to_string()
            }
        }

        // Numbers and their shift symbols
        Key::Num0 => {
            if shift_pressed {
                ")".to_string()
            } else {
                "0".to_string()
            }
        }
        Key::Num1 => {
            if shift_pressed {
                "!".to_string()
            } else {
                "1".to_string()
            }
        }
        Key::Num2 => {
            if shift_pressed {
                "@".to_string()
            } else {
                "2".to_string()
            }
        }
        Key::Num3 => {
            if shift_pressed {
                "#".to_string()
            } else {
                "3".to_string()
            }
        }
        Key::Num4 => {
            if shift_pressed {
                "$".to_string()
            } else {
                "4".to_string()
            }
        }
        Key::Num5 => {
            if shift_pressed {
                "%".to_string()
            } else {
                "5".to_string()
            }
        }
        Key::Num6 => {
            if shift_pressed {
                "^".to_string()
            } else {
                "6".to_string()
            }
        }
        Key::Num7 => {
            if shift_pressed {
                "&".to_string()
            } else {
                "7".to_string()
            }
        }
        Key::Num8 => {
            if shift_pressed {
                "*".to_string()
            } else {
                "8".to_string()
            }
        }
        Key::Num9 => {
            if shift_pressed {
                "(".to_string()
            } else {
                "9".to_string()
            }
        }

        // Special characters
        Key::Space => " ".to_string(),
        // Key::Tab => "Tab".to_string(),
        // Key::Enter => "Enter".to_string(),
        // Key::Backspace => "Backspace".to_string(),
        // Key::Escape => "Esc".to_string(),
        // Key::Delete => "Del".to_string(),

        // Arrow keys
        // Key::ArrowLeft => "←".to_string(),
        // Key::ArrowRight => "→".to_string(),
        // Key::ArrowUp => "↑".to_string(),
        // Key::ArrowDown => "↓".to_string(),

        // Punctuation and symbols
        Key::Minus => {
            if shift_pressed {
                "_".to_string()
            } else {
                "-".to_string()
            }
        }
        Key::Equals => {
            if shift_pressed {
                "+".to_string()
            } else {
                "=".to_string()
            }
        }
        Key::OpenBracket => {
            if shift_pressed {
                "{".to_string()
            } else {
                "[".to_string()
            }
        }
        Key::CloseBracket => {
            if shift_pressed {
                "}".to_string()
            } else {
                "]".to_string()
            }
        }
        Key::Backslash => {
            if shift_pressed {
                "|".to_string()
            } else {
                "\\".to_string()
            }
        }
        Key::Semicolon => {
            if shift_pressed {
                ":".to_string()
            } else {
                ";".to_string()
            }
        }
        Key::Quote => {
            if shift_pressed {
                "\"".to_string()
            } else {
                "'".to_string()
            }
        }
        Key::Comma => {
            if shift_pressed {
                "<".to_string()
            } else {
                ",".to_string()
            }
        }
        Key::Period => {
            if shift_pressed {
                ">".to_string()
            } else {
                ".".to_string()
            }
        }
        Key::Slash => {
            if shift_pressed {
                "?".to_string()
            } else {
                "/".to_string()
            }
        }
        Key::Backtick => {
            if shift_pressed {
                "~".to_string()
            } else {
                "`".to_string()
            }
        }

        // Function keys
        // Key::F1..=Key::F12 => format!("F{}", (key as u8 - Key::F1 as u8 + 1)),

        // Modifier keys
        // Key::Control => "Ctrl".to_string(),
        // Key::Alt => "Alt".to_string(),
        // Key::Shift => "Shift".to_string(),
        // Key::Insert => "Insert".to_string(),
        // Key::Home => "Home".to_string(),
        // Key::End => "End".to_string(),
        // Key::PageUp => "PgUp".to_string(),
        // Key::PageDown => "PgDn".to_string(),

        // Catch any other keys
        _ => "".to_string(),
    }
}

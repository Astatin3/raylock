use crate::structs;
use eframe::egui;
use egui::Color32;
use egui::Pos2;
use egui::Stroke;
use std::f32::consts::PI;

use std::sync::MutexGuard;

const TEXT_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
const LOGIN_CIRCLE_RADIUS: f32 = 50.;
const LOGIN_SUBCIRCLE_START_ANG: f32 = -PI / 4.;

const LOGIN_SUBCIRCLE_RADIUS: f32 = 4.;
const LOGIN_SUBCIRCLE_COLOR: Color32 = Color32::TRANSPARENT;
const LOGIN_SUBCIRCLE_STROKE: Stroke = Stroke {
    width: 2.0,
    color: Color32::from_rgb(255, 255, 255),
};
const LOGIN_CIRCLE_LINE_STROKE: Stroke = Stroke {
    width: 2.0,
    color: TEXT_COLOR,
};
const LOGIN_FAIL_COLOR: Color32 = Color32::from_rgb(184, 41, 11);
const LOGIN_FAIL_CIRCLE_STROKE: Stroke = Stroke {
    width: 5.,
    color: LOGIN_FAIL_COLOR,
};
const LOGIN_FAIL_COUNT_CIRCLE_RADIUS: f32 = 15.;
const LOGIN_FAIL_COUNT_CIRCLE_COLOR: Color32 = Color32::TRANSPARENT;
const LOGIN_FAIL_COUNT_CIRCLE_STROKE: Stroke = Stroke {
    width: 2.,
    color: LOGIN_FAIL_COLOR,
};

fn rot_circle(i: i16, center: Pos2, rad: f32, offset_ang: f32, ang_per_num: f32) -> Pos2 {
    center
        + (egui::Vec2 {
            x: rad * f32::cos(i as f32 * ang_per_num + offset_ang),
            y: rad * f32::sin(i as f32 * ang_per_num + offset_ang),
        })
}

pub fn update_password_viewer(
    wstate: MutexGuard<'_, structs::AuthState>,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut egui::Ui,
) {
    let mut state = wstate;
    let rect = ui.clip_rect();
    let center = Pos2 {
        x: rect.width() / 2.,
        y: rect.height() / 2.,
    };

    let painter = ui.painter();

    // Login Circle

    if state.failed_attempts > 0 {
        painter.circle(
            center,
            LOGIN_CIRCLE_RADIUS - LOGIN_FAIL_CIRCLE_STROKE.width,
            Color32::TRANSPARENT,
            LOGIN_FAIL_CIRCLE_STROKE,
        );
    }

    let ang_per_char = 2. * PI / state.password.len() as f32;
    let ang_per_fail = 2. * PI / state.failed_attempts as f32;
    let len: i16 = state.password.len() as i16;

    let mut last_pos = rot_circle(
        len - 1,
        center,
        LOGIN_CIRCLE_RADIUS,
        LOGIN_SUBCIRCLE_START_ANG,
        ang_per_char,
    );

    for i in 0..state.failed_attempts {
        let pos: egui::Pos2 = {
            if state.failed_attempts <= 1 {
                center
            } else {
                rot_circle(
                    i as i16,
                    center,
                    LOGIN_FAIL_COUNT_CIRCLE_RADIUS,
                    LOGIN_SUBCIRCLE_START_ANG,
                    ang_per_fail,
                )
            }
        };

        painter.circle(
            pos,
            LOGIN_SUBCIRCLE_RADIUS,
            LOGIN_FAIL_COUNT_CIRCLE_COLOR,
            LOGIN_FAIL_COUNT_CIRCLE_STROKE,
        );
    }

    for i in 0..len {
        let pos: egui::Pos2 = {
            if len <= 1 {
                center
            } else {
                rot_circle(
                    i,
                    center,
                    LOGIN_CIRCLE_RADIUS,
                    LOGIN_SUBCIRCLE_START_ANG,
                    ang_per_char,
                )
            }
        };

        painter.circle(
            pos,
            LOGIN_SUBCIRCLE_RADIUS,
            LOGIN_SUBCIRCLE_COLOR,
            LOGIN_SUBCIRCLE_STROKE,
        );

        if len > 1 {
            painter.line_segment([last_pos, pos], LOGIN_CIRCLE_LINE_STROKE);

            last_pos = pos;
        }
    }
}

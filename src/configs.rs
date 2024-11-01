use crate::structs;
use egui::{Color32, Stroke};
use std::f32::consts::PI;

// pub const windows: PaneSplit =
//     Pane::new(SplitHorisontal).new_hsplit(Pane::new(Temp1), Pane::new(Temp2));

pub const TEXT_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
pub const BACKGROUND_2: Color32 = Color32::from_rgba_premultiplied(10, 10, 10, 230);

pub const LOGIN_CIRCLE_RADIUS: f32 = 50.;
pub const LOGIN_SUBCIRCLE_START_ANG: f32 = -PI / 4.;

pub const LOGIN_SUBCIRCLE_RADIUS: f32 = 4.;
pub const LOGIN_SUBCIRCLE_COLOR: Color32 = Color32::TRANSPARENT;
pub const LOGIN_SUBCIRCLE_STROKE: Stroke = Stroke {
    width: 2.0,
    color: Color32::from_rgb(255, 255, 255),
};
pub const LOGIN_CIRCLE_LINE_STROKE: Stroke = Stroke {
    width: 2.0,
    color: TEXT_COLOR,
};
pub const LOGIN_FAIL_COLOR: Color32 = Color32::from_rgb(184, 41, 11);
pub const LOGIN_FAIL_CIRCLE_STROKE: Stroke = Stroke {
    width: 5.,
    color: LOGIN_FAIL_COLOR,
};
pub const LOGIN_FAIL_COUNT_CIRCLE_RADIUS: f32 = 15.;
pub const LOGIN_FAIL_COUNT_CIRCLE_COLOR: Color32 = Color32::TRANSPARENT;
pub const LOGIN_FAIL_COUNT_CIRCLE_STROKE: Stroke = Stroke {
    width: 2.,
    color: LOGIN_FAIL_COLOR,
};

pub const DOTS_SPACING: f32 = 25.;
pub const DOTS_RAD: f32 = 0.7;
pub const DOTS_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
// pub const DOTS_STROKE: Stroke = Stroke {0., DOTS_COLOR};

pub const CORNER_CUT: f32 = LOGIN_CIRCLE_RADIUS * 1.41421356237;
pub const PANE_GAP: f32 = 6.;

use crate::structs;
use egui::{Color32, FontId, Stroke};
use std::f32::consts::PI;

// pub const windows: PaneSplit =
//     Pane::new(SplitHorisontal).new_hsplit(Pane::new(Temp1), Pane::new(Temp2));

pub const SCREEN_WIDTH: f32 = 1920.;
pub const SCREEN_HEIGHT: f32 = 1080.;

pub const TITLE_FONT: FontId = FontId::monospace(25.);
pub const GRAPH_STROKE: Stroke = Stroke {
    width: 0.5,
    color: Color32::from_rgba_premultiplied(255, 255, 255, 1),
};

pub const TEXT_FONT: FontId = FontId::monospace(16.);

pub const UP_GRAPH_STROKE: Stroke = Stroke {
    width: 1.5,
    color: Color32::from_rgba_premultiplied(0, 255, 255, 1),
};

pub const DOWN_GRAPH_STROKE: Stroke = Stroke {
    width: 1.5,
    color: Color32::from_rgba_premultiplied(255, 64, 4, 1),
};

pub const TEXT_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
pub const BACKGROUND: Color32 = Color32::BLACK;
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

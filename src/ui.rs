use crate::configs::*;
use crate::graph::CpuGraph;
use crate::panes;
use crate::structs;
use panes::PaneRenderer;
// use crate::structs::cur_context;
// use crate::structs::windowTypes;

use eframe::egui;
use egui::Color32;
use egui::Shape;
use egui::{Pos2, Vec2};
use panes::{PaneConfig, SplitDirection};
use std::f32::consts::PI;

use std::ops::Deref;
use std::sync::MutexGuard;

fn rot_circle(i: i16, center: Pos2, rad: f32, offset_ang: f32, ang_per_num: f32) -> Pos2 {
    center
        + (egui::Vec2 {
            x: rad * f32::cos(i as f32 * ang_per_num + offset_ang),
            y: rad * f32::sin(i as f32 * ang_per_num + offset_ang),
        })
}

fn add_Pos2(add: Vec2, points: &mut Vec<Pos2>) -> &mut Vec<Pos2> {
    for i in 0..points.len() {
        points[i] = points[i] + add;
    }
    points
}

fn mult_Pos2(mult: f32, points: &mut Vec<Pos2>) -> &mut Vec<Pos2> {
    let vec = Vec2 { x: mult, y: mult };
    for i in 0..points.len() {
        points[i] = points[i] * mult;
    }
    points
}

fn rotate_90(points: &mut Vec<Pos2>) -> &mut Vec<Pos2> {
    for i in 0..points.len() {
        points[i] = Pos2 {
            x: -points[i].y,
            y: points[i].x,
        }
    }
    points
}
fn rotate_180(points: &mut Vec<Pos2>) -> &mut Vec<Pos2> {
    for i in 0..points.len() {
        points[i] = Pos2 {
            x: -points[i].x,
            y: -points[i].y,
        }
    }
    points
}
fn rotate_270(points: &mut Vec<Pos2>) -> &mut Vec<Pos2> {
    for i in 0..points.len() {
        points[i] = Pos2 {
            x: points[i].y,
            y: -points[i].x,
        }
    }
    points
}

const CORNER_SQUARE: [Pos2; 1] = [Pos2 { x: 0., y: 0. }];
const CORNER_45: [Pos2; 2] = [Pos2 { x: 0., y: 1. }, Pos2 { x: 1., y: 0. }];
const CORNER_30: [Pos2; 2] = [Pos2 { x: 0., y: 0.5 }, Pos2 { x: 1., y: 0. }];
const CORNER_60: [Pos2; 2] = [Pos2 { x: 0., y: 1. }, Pos2 { x: 0.5, y: 0. }];

fn get_corner_points(ctype: panes::CornerTypes) -> Vec<Pos2> {
    // CORNER_30.to_vec()
    match ctype {
        panes::CornerTypes::SQUARE => CORNER_SQUARE.to_vec(),
        panes::CornerTypes::Ang30 => CORNER_30.to_vec(),
        panes::CornerTypes::Ang45 => CORNER_45.to_vec(),
        panes::CornerTypes::Ang60 => CORNER_60.to_vec(),
    }
}

pub fn background_render(
    painter: &egui::Painter,
    rect: egui::Rect,
    corners: [panes::CornerTypes; 4],
) {
    let mut points: Vec<Pos2> = Vec::new();
    let left = rect.left() + PANE_GAP;
    let right = rect.right() - PANE_GAP;
    let bottom = rect.bottom() - PANE_GAP;
    let top = rect.top() + PANE_GAP;
    // let x1 = left + CORNER_CUT - PANE_GAP;
    // let x2 = right - CORNER_CUT + PANE_GAP;
    // let y1 = top + CORNER_CUT - PANE_GAP;
    // let y2 = bottom - CORNER_CUT + PANE_GAP;

    points.append(add_Pos2(
        Vec2 { x: left, y: top },
        mult_Pos2(CORNER_CUT, &mut get_corner_points(corners[0]).to_vec()),
    ));

    points.append(add_Pos2(
        Vec2 { x: right, y: top },
        mult_Pos2(
            CORNER_CUT,
            rotate_90(&mut get_corner_points(corners[1]).to_vec()),
        ),
    ));

    points.append(add_Pos2(
        Vec2 {
            x: right,
            y: bottom,
        },
        mult_Pos2(
            CORNER_CUT,
            rotate_180(&mut get_corner_points(corners[2]).to_vec()),
        ),
    ));

    points.append(add_Pos2(
        Vec2 { x: left, y: bottom },
        mult_Pos2(
            CORNER_CUT,
            rotate_270(&mut get_corner_points(corners[3]).to_vec()),
        ),
    ));

    let filled_polygon =
        Shape::convex_polygon(points, BACKGROUND_2, egui::Stroke::new(0.5, TEXT_COLOR));

    painter.add(filled_polygon);
}

pub fn update_password_viewer(
    wstate: MutexGuard<'_, structs::AuthState>,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut egui::Ui,
    winconfig: &mut PaneRenderer,
) {
    let rect: egui::Rect = ui.available_rect_before_wrap();
    // let ctx: cur_context = cur_context {
    let state: &structs::AuthState = wstate.deref();

    let center: Pos2 = Pos2 {
        x: rect.width() / 2.,
        y: rect.height() / 2.,
    };
    let painter: &egui::Painter = ui.painter();

    paint_windows(
        painter,
        rect.width() as f32,
        rect.height() as f32,
        0.,
        0.,
        // windows,
    );

    dots(painter, ui.clip_rect());
    winconfig.render(painter, &rect);
    paint_password_circle(state, ctx, frame, ui, center, painter);
}

fn dots(painter: &egui::Painter, win_rect: egui::Rect) {
    let dots_x = win_rect.right() / DOTS_SPACING;
    let dots_y = win_rect.bottom() / DOTS_SPACING;
    for x in (-dots_x as i32 / 2)..((dots_x as i32 / 2) + 1) {
        for y in (-dots_y as i32 / 2)..((dots_y as i32 / 2) + 1) {
            painter.circle(
                Pos2 {
                    x: win_rect.right() / 2. + (x as f32 * DOTS_SPACING),
                    y: win_rect.bottom() / 2. + (y as f32 * DOTS_SPACING),
                },
                DOTS_RAD,
                DOTS_COLOR,
                egui::Stroke::NONE,
            );
        }
    }
}

fn paint_password_circle(
    state: &structs::AuthState,

    ctx: &egui::Context,
    frame: &eframe::Frame,
    ui: &egui::Ui,

    center: egui::Pos2,
    painter: &egui::Painter,
) {
    let len = state.password.len();

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

    ctx.request_repaint();
}

fn paint_windows(
    painter: &egui::Painter,

    width: f32,
    height: f32,
    x: f32,
    y: f32,
    // pane: structs::PaneSplit,
) {
    // match pane.wintype {
    //     windowTypes::SplitHorisontal => {
    //         paint_windows(painter, width, height / 2., x, y, pane.sub_a);
    //         paint_windows(painter, width, height / 2., x, y + height / 2., pane.sub_b);
    //     }
    // }
}

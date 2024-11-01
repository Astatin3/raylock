use crate::configs::*;
use crate::structs;
use crate::structs::PaneRenderer;
// use crate::structs::cur_context;
// use crate::structs::windowTypes;

use eframe::egui;
use egui::Color32;
use egui::Pos2;
use egui::Shape;
use std::f32::consts::PI;
use structs::{PaneConfig, SplitDirection};

use std::ops::Deref;
use std::sync::MutexGuard;

fn rot_circle(i: i16, center: Pos2, rad: f32, offset_ang: f32, ang_per_num: f32) -> Pos2 {
    center
        + (egui::Vec2 {
            x: rad * f32::cos(i as f32 * ang_per_num + offset_ang),
            y: rad * f32::sin(i as f32 * ang_per_num + offset_ang),
        })
}

pub fn background_render(painter: &egui::Painter, rect: egui::Rect) {
    let left = rect.left() + PANE_GAP;
    let right = rect.right() - PANE_GAP;
    let bottom = rect.bottom() - PANE_GAP;
    let top = rect.top() + PANE_GAP;
    let x1 = left + CORNER_CUT - PANE_GAP;
    let x2 = right - CORNER_CUT + PANE_GAP;
    let y1 = top + CORNER_CUT - PANE_GAP;
    let y2 = bottom - CORNER_CUT + PANE_GAP;

    let points = [
        egui::Pos2 { x: x1, y: top },
        egui::Pos2 { x: x2, y: top },
        egui::Pos2 { x: right, y: y1 },
        egui::Pos2 { x: right, y: y2 },
        egui::Pos2 { x: x2, y: bottom },
        egui::Pos2 { x: x1, y: bottom },
        egui::Pos2 { x: left, y: y2 },
        egui::Pos2 { x: left, y: y1 },
    ];

    let filled_polygon = Shape::convex_polygon(
        points.to_vec(),
        BACKGROUND_2,
        egui::Stroke::new(0.5, TEXT_COLOR),
    );

    painter.add(filled_polygon);
    // painter.rect_filled(rect, 0.0, Color32::RED);
    // painter.rect_stroke(rect, 0.0, (1.0, Color32::BLACK));
}

// fn draw_polygon(ui: &mut egui::Ui, points: &[Pos2], fill_color: egui::Color32, stroke: ) {
//     if points.len() < 3 {
//         return; // Need at least 3 points for a polygon
//     }

//     // Create the filled polygon
//     let filled_polygon = Shape::convex_polygon(
//         points.to_vec(),
//         fill_color,
//         Stroke::new(0.0, fill_color), // No stroke for fill
//     );

//     // Create the outline
//     let outline = Shape::line(
//         points.iter().chain(std::iter::once(&points[0])).cloned().collect(),
//         Stroke::new(stroke_width, outline_color),
//     );

//     // Get the painter and draw both shapes
//     let painter = ui.painter();
//     painter.add(filled_polygon);
//     painter.add(outline);
// }

pub fn winconfig() -> PaneConfig {
    PaneConfig::new("root")
        .with_split(SplitDirection::Horizontal, 0.5)
        .with_callback(|painter: &egui::Painter, rect: egui::Rect| background_render(painter, rect))
        .with_children(vec![
            PaneConfig::new("left")
                .with_split(SplitDirection::Vertical, 0.5)
                .with_children(vec![
                    PaneConfig::new("left_top").with_callback(
                        |painter: &egui::Painter, rect: egui::Rect| {
                            background_render(painter, rect)
                        },
                    ),
                    PaneConfig::new("left_bottom")
                        // .with_split(SplitDirection::Horizontal, 0.5)
                        .with_callback(|painter: &egui::Painter, rect: egui::Rect| {
                            background_render(painter, rect)
                        }),
                ]),
            PaneConfig::new("right")
                .with_split(SplitDirection::Vertical, 0.5)
                .with_callback(|painter: &egui::Painter, rect: egui::Rect| {
                    background_render(painter, rect)
                })
                .with_children(vec![
                    PaneConfig::new("right_top")
                        .with_split(SplitDirection::Horizontal, 0.5)
                        .with_children(vec![
                            PaneConfig::new("right_top_1").with_callback(
                                |painter: &egui::Painter, rect: egui::Rect| {
                                    background_render(painter, rect)
                                },
                            ),
                            PaneConfig::new("right_top_2").with_callback(
                                |painter: &egui::Painter, rect: egui::Rect| {
                                    background_render(painter, rect)
                                },
                            ),
                        ]),
                    PaneConfig::new("right_bottom").with_callback(
                        |painter: &egui::Painter, rect: egui::Rect| {
                            background_render(painter, rect)
                        },
                    ),
                ]),
        ])
}

pub fn update_password_viewer(
    wstate: MutexGuard<'_, structs::AuthState>,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut egui::Ui,
    winconfig: PaneRenderer,
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
    winconfig.render(painter, rect);
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

fn paint_window(painter: &egui::Painter, width: f32, height: f32, x: f32, y: f32) {}

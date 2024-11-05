use crate::configs::*;
use crate::panes;
use crate::panes::PaneInstance;
use crate::structs;
// use panes::PaneRenderer;
// use crate::structs::cur_context;
// use crate::structs::windowTypes;

use eframe::egui;
use egui::Color32;
use egui::Shape;
use egui::{Pos2, Rect, Vec2};
use panes::Pane;
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

fn add_Pos2(add: Vec2, points: Vec<Pos2>) -> Vec<Pos2> {
    let mut points = points;
    for i in 0..points.len() {
        points[i] = points[i] + add;
    }
    points
}

fn mult_Pos2(mult: f32, points: Vec<Pos2>) -> Vec<Pos2> {
    let mut points = points;
    let vec = Vec2 { x: mult, y: mult };
    for i in 0..points.len() {
        points[i] = points[i] * mult;
    }
    points
}

fn rotate_90(points: Vec<Pos2>) -> Vec<Pos2> {
    let mut points = points;
    for i in 0..points.len() {
        points[i] = Pos2 {
            x: -points[i].y,
            y: points[i].x,
        }
    }
    points
}
fn rotate_180(points: Vec<Pos2>) -> Vec<Pos2> {
    let mut points = points;
    for i in 0..points.len() {
        points[i] = Pos2 {
            x: -points[i].x,
            y: -points[i].y,
        }
    }
    points
}
fn rotate_270(points: Vec<Pos2>) -> Vec<Pos2> {
    let mut points = points;
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

pub fn get_corners(rect: egui::Rect, corners: [panes::CornerTypes; 4]) -> Vec<Pos2> {
    let mut points: Vec<Pos2> = Vec::new();
    let left = rect.left() + PANE_GAP;
    let right = rect.right() - PANE_GAP;
    let bottom = rect.bottom() - PANE_GAP;
    let top = rect.top() + PANE_GAP;

    let rot_point_tl = get_corner_points(corners[0]).to_vec();
    let rot_point_tr = rotate_90(get_corner_points(corners[1]).to_vec());
    let rot_point_bl = rotate_180(get_corner_points(corners[2]).to_vec());
    let rot_point_br = rotate_270(get_corner_points(corners[3]).to_vec());

    points.append(
        add_Pos2(
            Vec2 { x: left, y: top },
            mult_Pos2(CORNER_CUT, rot_point_tl),
        )
        .as_mut(),
    );

    points.append(
        add_Pos2(
            Vec2 { x: right, y: top },
            mult_Pos2(CORNER_CUT, rot_point_tr),
        )
        .as_mut(),
    );

    points.append(
        add_Pos2(
            Vec2 {
                x: right,
                y: bottom,
            },
            mult_Pos2(CORNER_CUT, rot_point_bl),
        )
        .as_mut(),
    );

    points.append(
        add_Pos2(
            Vec2 { x: left, y: bottom },
            mult_Pos2(CORNER_CUT, rot_point_br),
        )
        .as_mut(),
    );

    // let largest_rect = find_largest_rectangle(&points).unwrap();

    // let filled_polygon =

    return points;
}

pub fn update(
    wstate: MutexGuard<'_, structs::AuthState>,
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    ui: &mut egui::Ui,
    root_pane: &mut PaneInstance,
) {
    let rect: egui::Rect = ui.available_rect_before_wrap();
    // let ctx: cur_context = cur_context {
    let state: &structs::AuthState = wstate.deref();

    let center: Pos2 = Pos2 {
        x: rect.width() / 2.,
        y: rect.height() / 2.,
    };
    let painter: &egui::Painter = ui.painter();

    // paint_windows(
    //     painter,
    //     rect.width() as f32,
    //     rect.height() as f32,
    //     0.,
    //     0.,
    //     // windows,
    // );

    // dots(painter, ui.clip_rect());
    root_pane.render(ui.painter());
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

pub fn find_largest_rectangle(points: &[Pos2]) -> Option<Rect> {
    if points.len() < 4 {
        return None;
    }

    // Helper function to calculate area of a rectangle
    fn calculate_area(rect: &Rect) -> f32 {
        rect.width() * rect.height()
    }

    // Helper function to determine if a point is inside a polygon using ray casting
    fn is_point_in_polygon(point: &Pos2, polygon: &[Pos2]) -> bool {
        let mut inside = false;
        let mut j = polygon.len() - 1;

        for i in 0..polygon.len() {
            if ((polygon[i].y >= point.y) != (polygon[j].y >= point.y))
                && (point.x
                    <= (polygon[j].x - polygon[i].x) * (point.y - polygon[i].y)
                        / (polygon[j].y - polygon[i].y)
                        + polygon[i].x)
            {
                inside = !inside;
            }
            j = i;
        }

        inside
    }

    // Helper function to check if a rectangle is completely inside the polygon
    fn is_rect_in_polygon(rect: &Rect, polygon: &[Pos2]) -> bool {
        // Check all four corners of the rectangle
        let corners = [
            rect.min,
            Pos2::new(rect.max.x, rect.min.y),
            rect.max,
            Pos2::new(rect.min.x, rect.max.y),
        ];

        corners
            .iter()
            .all(|corner| is_point_in_polygon(corner, polygon))
    }

    // Helper function to check if a set of points forms a valid rectangle
    fn is_valid_rectangle(p1: &Pos2, p2: &Pos2, p3: &Pos2, p4: &Pos2) -> bool {
        // Sort points by x coordinate
        let mut pts = vec![p1, p2, p3, p4];
        pts.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        // For a valid rectangle:
        // - The first two points should have the same x-coordinate
        // - The last two points should have the same x-coordinate
        // - Two points should have the min y-coordinate
        // - Two points should have the max y-coordinate
        let x_eps = 0.;
        let y_eps = 0.;

        // // Check x-coordinates are properly paired
        // if (pts[0].x - pts[1].x).abs() >= x_eps || (pts[2].x - pts[3].x).abs() >= x_eps {
        //     return false;
        // }

        // // Sort points by y coordinate
        // pts.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

        // // Check y-coordinates are properly paired
        // if (pts[0].y - pts[1].y).abs() >= y_eps || (pts[2].y - pts[3].y).abs() >= y_eps {
        //     return false;
        // }

        true
    }

    // Sort points to ensure they form a proper polygon
    let mut polygon_points = points.to_vec();
    let center = Pos2::new(
        polygon_points.iter().map(|p| p.x).sum::<f32>() / polygon_points.len() as f32,
        polygon_points.iter().map(|p| p.y).sum::<f32>() / polygon_points.len() as f32,
    );

    // Sort points clockwise around the center
    polygon_points.sort_by(|a, b| {
        let a_angle = (a.y - center.y).atan2(a.x - center.x);
        let b_angle = (b.y - center.y).atan2(b.x - center.x);
        b_angle.partial_cmp(&a_angle).unwrap()
    });

    let mut largest_rect = None;
    let mut max_area = 0.0;

    // Try all combinations of 4 points
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for k in j + 1..points.len() {
                for l in k + 1..points.len() {
                    let p1 = points[i];
                    let p2 = points[j];
                    let p3 = points[k];
                    let p4 = points[l];

                    // Skip if these points don't form a valid rectangle
                    if !is_valid_rectangle(&p1, &p2, &p3, &p4) {
                        continue;
                    }

                    // Create rectangle from these points
                    let min_x = p1.x.min(p2.x).min(p3.x).min(p4.x);
                    let max_x = p1.x.max(p2.x).max(p3.x).max(p4.x);
                    let min_y = p1.y.min(p2.y).min(p3.y).min(p4.y);
                    let max_y = p1.y.max(p2.y).max(p3.y).max(p4.y);

                    let rect = Rect {
                        min: Pos2 {
                            x: min_x + 1.,
                            y: min_y + 1.,
                        },
                        max: Pos2 {
                            x: max_x - 1.,
                            y: max_y - 1.,
                        },
                    };

                    // Verify the rectangle is inside the polygon
                    if !is_rect_in_polygon(&rect, &polygon_points) {
                        continue;
                    }

                    let area = calculate_area(&rect);
                    if area > max_area {
                        max_area = area;
                        largest_rect = Some(rect);
                    }
                }
            }
        }
    }

    largest_rect
}

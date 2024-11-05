use egui::{Color32, Pos2, Rect, Stroke};
use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::time::Instant;
use sysinfo::System;

use crate::configs;

pub const HISTORY_SIZE: usize = 25;
pub const ANIMATION_DURATION: f32 = 0.2; // seconds
pub const UPDATES_PER_SECOND: f32 = 1.5;

#[derive(Clone)]
struct DataPoint {
    value: f32,
    timestamp: Instant,
}

#[derive(Clone)]
struct AnimatedValue {
    current: f32,
    target: f32,
    last_update: Instant,
}

impl AnimatedValue {
    fn new(initial: f32) -> Self {
        Self {
            current: initial,
            target: initial,
            last_update: Instant::now(),
        }
    }

    fn update(&mut self, new_target: f32) {
        self.target = new_target;
        self.last_update = Instant::now();
    }

    fn get_current_value(&mut self) -> f32 {
        let elapsed = self.last_update.elapsed().as_secs_f32();
        let progress = (elapsed / ANIMATION_DURATION).min(1.0);
        self.current = self.current + (self.target - self.current) * progress;
        self.current
    }
}

#[derive(Clone)]
pub struct GraphLine {
    label: String,
    stroke: Stroke,
    history: VecDeque<DataPoint>,
    animated_value: AnimatedValue,
}

impl GraphLine {
    fn new(label: String, stroke: Stroke) -> Self {
        Self {
            label,
            stroke,
            history: VecDeque::with_capacity(HISTORY_SIZE),
            animated_value: AnimatedValue::new(0.0),
        }
    }

    fn update(&mut self, value: f32) {
        self.animated_value.update(value);
        self.history.push_back(DataPoint {
            value,
            timestamp: Instant::now(),
        });

        while self.history.len() > HISTORY_SIZE {
            self.history.pop_front();
        }
    }
}

pub struct ResourceGraph {
    lines: Vec<GraphLine>,
    // last_update: Instant,
    title: String,
    y_axis_label: String,
    min_value: f32,
    max_value: f32,
}

impl ResourceGraph {
    pub fn new(title: String, y_axis_label: String, min_value: f32, max_value: f32) -> Self {
        Self {
            lines: Vec::new(),
            // last_update: Instant::now(),
            title,
            y_axis_label,
            min_value,
            max_value,
        }
    }

    pub fn redo_max(&mut self) {
        let mut max = 0.5;
        for i in 0..self.lines.len() {
            let history = self.lines.get_mut(i).unwrap();

            for j in 0..history.history.len() {
                let val = history.history.get(j).unwrap();
                if val.value > max {
                    max = val.value;
                }
            }
        }
        self.max_value = max;
    }

    pub fn add_line(&mut self, label: String, stroke: Stroke) {
        self.lines.push(GraphLine::new(label, stroke));
    }

    pub fn update_line(&mut self, index: usize, value: f32) {
        if let Some(line) = self.lines.get_mut(index) {
            line.update(value);
        }
    }

    pub fn render(&mut self, painter: &egui::Painter, rect: Rect) {
        // let now = Instant::now();
        // Draw background
        // painter.rect_filled(rect, 0.0, Color32::from_gray(20));

        // Draw title
        // painter.text(
        //     Pos2::new(rect.min.x + 5.0, rect.min.y + 5.0),
        //     egui::Align2::LEFT_TOP,
        //     &self.title,
        //     egui::FontId::proportional(16.0),
        //     Color32::WHITE,
        // );

        // Calculate graph area (leaving space for labels)
        let label_width = 65.0;
        let graph_rect = Rect::from_min_max(
            Pos2::new(rect.min.x + label_width, rect.min.y + 30.0),
            rect.max,
        );

        // Draw grid lines and labels
        for i in 0..=4 {
            let y = graph_rect.min.y + graph_rect.height() * (i as f32 / 4.0);
            painter.line_segment(
                [
                    Pos2::new(graph_rect.min.x, y),
                    Pos2::new(graph_rect.max.x, y),
                ],
                Stroke::new(1.0, Color32::from_gray(40)),
            );

            let value = self.max_value - (i as f32 / 4.0) * (self.max_value - self.min_value);
            painter.text(
                Pos2::new(rect.min.x + 5.0, y - 8.0),
                egui::Align2::LEFT_CENTER,
                format!("{:.1}{}", value, self.y_axis_label),
                egui::FontId::proportional(12.0),
                Color32::LIGHT_GRAY,
            );
        }

        // Draw lines
        for (_, line) in self.lines.iter_mut().enumerate() {
            if line.history.is_empty() {
                continue;
            }

            let points: Vec<Pos2> = line
                .history
                .iter()
                .enumerate()
                .map(|(i, point)| {
                    let x = graph_rect.min.x
                        + graph_rect.width() * (i as f32 / (HISTORY_SIZE - 1) as f32);

                    let normalized_value =
                        (point.value - self.min_value) / (self.max_value - self.min_value);
                    let y = graph_rect.max.y - graph_rect.height() * normalized_value;
                    Pos2::new(x, y)
                })
                .collect();

            // Get the current animated value for the last point
            let current_value = line.animated_value.get_current_value();
            let mut animated_points = points;
            if let Some(last) = animated_points.last_mut() {
                let normalized_value =
                    (current_value - self.min_value) / (self.max_value - self.min_value);
                last.y = graph_rect.max.y - graph_rect.height() * normalized_value;
            }

            painter.add(egui::Shape::line(animated_points, line.stroke));
        }
        // self.last_update = now;
    }
}

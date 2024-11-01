use egui::{Color32, Pos2, Rect, Stroke, Vec2};
use std::collections::VecDeque;
use std::time::Instant;
use sysinfo::System;

const HISTORY_SIZE: usize = 100;
const ANIMATION_DURATION: f32 = 0.2; // seconds
const UPDATES_PER_SECOND: f32 = 2.0;

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
// #[derive(Clone)]
//
// #[derive(Debug)]
pub struct CpuGraph {
    sys: System,
    history: Vec<VecDeque<DataPoint>>,
    animated_values: Vec<AnimatedValue>,
    last_update: Instant,
    colors: Vec<Color32>,
}

impl CpuGraph {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_cpu_all();

        let cpu_count = sys.cpus().len();
        let history = vec![VecDeque::with_capacity(HISTORY_SIZE); cpu_count];
        let animated_values = vec![AnimatedValue::new(0.0); cpu_count];

        // Generate distinct colors for each CPU core
        let colors = (0..cpu_count)
            .map(|i| {
                let hue = (i as f32 / cpu_count as f32) * 360.0;
                Color32::from_rgb(
                    ((1.0 + (hue.sin())) * 127.5) as u8,
                    ((1.0 + ((hue + 120.0).sin())) * 127.5) as u8,
                    ((1.0 + ((hue + 240.0).sin())) * 127.5) as u8,
                )
            })
            .collect();

        Self {
            sys,
            history,
            animated_values,
            last_update: Instant::now(),
            colors,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update).as_secs_f32() < 1.0 / UPDATES_PER_SECOND {
            return;
        }

        self.sys.refresh_cpu_all();

        for (i, cpu) in self.sys.cpus().iter().enumerate() {
            let usage = cpu.cpu_usage() / 100.0;

            self.animated_values[i].update(usage);

            self.history[i].push_back(DataPoint {
                value: usage,
                timestamp: now,
            });

            while self.history[i].len() > HISTORY_SIZE {
                self.history[i].pop_front();
            }
        }

        self.last_update = now;
    }

    pub fn render(&mut self, painter: &egui::Painter, rect: Rect) {
        let stroke_width = 2.0;

        // Draw background
        painter.rect_filled(rect, 0.0, Color32::from_gray(20));

        // Draw grid lines
        for i in 0..=4 {
            let y = rect.min.y + rect.height() * (i as f32 / 4.0);
            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                Stroke::new(1.0, Color32::from_gray(40)),
            );
        }

        // Draw CPU usage lines
        for (core_idx, (history, color)) in self.history.iter().zip(self.colors.iter()).enumerate()
        {
            if history.is_empty() {
                continue;
            }

            let points: Vec<Pos2> = history
                .iter()
                .enumerate()
                .map(|(i, point)| {
                    let x = rect.min.x + rect.width() * (i as f32 / (HISTORY_SIZE - 1) as f32);
                    let y = rect.max.y - rect.height() * point.value;
                    Pos2::new(x, y)
                })
                .collect();

            // Get the current animated value for the last point
            let current_value = self.animated_values[core_idx].get_current_value();
            let mut animated_points = points;
            if let Some(last) = animated_points.last_mut() {
                last.y = rect.max.y - rect.height() * current_value;
            }

            // Draw the line
            painter.add(egui::Shape::line(
                animated_points,
                Stroke::new(stroke_width, *color),
            ));
        }

        // Draw labels
        for (i, color) in self.colors.iter().enumerate() {
            let text = format!("CPU {}", i);
            let pos = Pos2::new(rect.min.x + 5.0, rect.min.y + 15.0 + (i as f32 * 20.0));
            painter.text(
                pos,
                egui::Align2::LEFT_TOP,
                text,
                egui::FontId::proportional(14.0),
                *color,
            );
        }
    }
}

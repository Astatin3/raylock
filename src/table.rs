use egui::{Color32, Pos2, Rect, Response, Stroke, Vec2};
use std::time::Instant;
use sysinfo::{Pid, System};

const UPDATE_INTERVAL: f32 = 0.5; // seconds
pub const BAR_HEIGHT: f32 = 16.0;
pub const ROW_HEIGHT: f32 = 24.0;
const COLUMN_PADDING: f32 = 10.0;
const SWAP_INTERVAL: f32 = 5.; // seconds

#[derive(Clone, Copy)]
pub enum SortColumn {
    Cpu,
    Memory,
}

struct ProcessInfo {
    pid: Pid,
    name: String,
    command: String,
    cpu_usage: f32,
    memory_bytes: u64,
    memory_percent: f32,
}

pub struct ProcessTable {
    sys: System,
    last_update: Instant,
    last_swap: Instant,
    processes: Vec<ProcessInfo>,
    sort_by: SortColumn,
    name_width: usize,
    command_width: usize,
    pub row_count: usize,
}

fn rem_first_and_last(value: String, num: usize) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.take(num).collect::<String>()
}

impl ProcessTable {
    pub fn new(name_width: usize, command_width: usize, row_count: usize) -> Self {
        Self {
            sys: System::new_all(),
            last_update: Instant::now(),
            last_swap: Instant::now(),
            processes: Vec::new(),
            sort_by: SortColumn::Cpu,
            name_width,
            command_width,
            row_count,
        }
    }

    pub fn set_sort(&mut self, sort_by: SortColumn) {
        self.sort_by = sort_by;
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update).as_secs_f32() < UPDATE_INTERVAL {
            return;
        }
        if now.duration_since(self.last_swap).as_secs_f32() > SWAP_INTERVAL {
            self.sort_by = match self.sort_by {
                SortColumn::Cpu => SortColumn::Memory,
                SortColumn::Memory => SortColumn::Cpu,
            };
            self.last_swap = now;
        }

        self.sys.refresh_all();

        let total_memory = self.sys.total_memory() as f64;

        // Collect process information
        self.processes = self
            .sys
            .processes()
            .iter()
            .map(|(pid, proc)| ProcessInfo {
                pid: *pid,
                name: proc.name().to_str().unwrap().to_string(),
                command: format!("{:?}", proc.cmd()),
                cpu_usage: proc.cpu_usage(),
                memory_bytes: proc.memory(),
                memory_percent: (proc.memory() as f64 / total_memory * 100.0) as f32,
            })
            .collect();

        // Sort processes based on selected criterion
        match self.sort_by {
            SortColumn::Cpu => {
                self.processes
                    .sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
            }
            SortColumn::Memory => {
                self.processes
                    .sort_by_key(|p| std::cmp::Reverse(p.memory_bytes));
            }
        }

        // Keep only top N processes
        self.processes.truncate(self.row_count);

        self.last_update = now;
    }

    fn draw_bar(&self, painter: &egui::Painter, rect: Rect, percentage: f32, color: Color32) {
        // Background
        painter.rect_filled(rect, 0.0, Color32::from_gray(40));

        // Foreground bar
        let bar_width = rect.width() * (percentage / 100.0).min(1.0);
        let bar_rect = Rect::from_min_max(rect.min, Pos2::new(rect.min.x + bar_width, rect.max.y));
        painter.rect_filled(bar_rect, 0.0, color);

        // Percentage text
        let text = format!("{:.1}%", percentage);
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(12.0),
            Color32::WHITE,
        );
    }

    fn draw_header(&self, painter: &egui::Painter, rect: Rect) -> f32 {
        let text_color = Color32::LIGHT_GRAY;
        let header_height = 24.0;
        let header_rect =
            Rect::from_min_max(rect.min, Pos2::new(rect.max.x, rect.min.y + header_height));

        // Background
        painter.rect_filled(header_rect, 0.0, Color32::from_gray(30));

        let mut x = rect.min.x + COLUMN_PADDING;

        // PID header
        painter.text(
            Pos2::new(x, header_rect.center().y),
            egui::Align2::LEFT_CENTER,
            "PID",
            egui::FontId::proportional(14.0),
            text_color,
        );
        x += rect.width() * 0.1;

        // Name header
        painter.text(
            Pos2::new(x, header_rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Name",
            egui::FontId::proportional(14.0),
            text_color,
        );
        x += rect.width() * 0.2;

        // Command header
        painter.text(
            Pos2::new(x, header_rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Command",
            egui::FontId::proportional(14.0),
            text_color,
        );
        x += rect.width() * 0.5;

        // CPU header
        painter.text(
            Pos2::new(x, header_rect.center().y),
            egui::Align2::LEFT_CENTER,
            "CPU %",
            egui::FontId::proportional(14.0),
            text_color,
        );
        x += rect.width() * 0.1;

        // Memory header
        painter.text(
            Pos2::new(x, header_rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Memory %",
            egui::FontId::proportional(14.0),
            text_color,
        );

        header_height
    }

    pub fn render(&mut self, painter: &egui::Painter, rect: Rect) {
        let header_height = self.draw_header(painter, rect);
        let content_rect =
            Rect::from_min_max(Pos2::new(rect.min.x, rect.min.y + header_height), rect.max);

        for (i, process) in self.processes.iter().enumerate() {
            let row_min_y = content_rect.min.y + (i as f32 * ROW_HEIGHT);
            let row_rect = Rect::from_min_max(
                Pos2::new(content_rect.min.x, row_min_y),
                Pos2::new(content_rect.max.x, row_min_y + ROW_HEIGHT),
            );

            // Background (alternating)
            painter.rect_filled(
                row_rect,
                0.0,
                if i % 2 == 0 {
                    Color32::from_gray(25)
                } else {
                    Color32::from_gray(20)
                },
            );

            let mut x = row_rect.min.x + COLUMN_PADDING;
            let text_y = row_rect.center().y;

            // PID
            painter.text(
                Pos2::new(x, text_y),
                egui::Align2::LEFT_CENTER,
                process.pid.to_string(),
                egui::FontId::proportional(14.0),
                Color32::WHITE,
            );
            x += rect.width() * 0.1;

            // Name (truncated)
            painter.text(
                Pos2::new(x, text_y),
                egui::Align2::LEFT_CENTER,
                process
                    .name
                    .chars()
                    .take(self.name_width)
                    .collect::<String>(),
                egui::FontId::proportional(14.0),
                Color32::WHITE,
            );
            x += rect.width() * 0.2;

            // let cmd = rem_first_and_last(
            //     process.command.split("\", \"").collect(),
            //     self.command_width,
            // );

            // Command (truncated)
            painter.text(
                Pos2::new(x, text_y),
                egui::Align2::LEFT_CENTER,
                process
                    .command
                    .chars()
                    .take(self.row_count)
                    .collect::<String>(),
                // .chars()
                // .take(self.command_width).collect::<String>(),
                egui::FontId::proportional(14.0),
                Color32::WHITE,
            );
            x += rect.width() * 0.5;

            let cpu_bar_rect = Rect::from_min_max(
                Pos2::new(x, row_rect.min.y + (ROW_HEIGHT - BAR_HEIGHT) / 2.0),
                Pos2::new(
                    x + rect.width() * 0.08,
                    row_rect.min.y + (ROW_HEIGHT + BAR_HEIGHT) / 2.0,
                ),
            );
            self.draw_bar(
                painter,
                cpu_bar_rect,
                process.cpu_usage,
                Color32::from_rgb(46, 194, 126),
            );
            x += rect.width() * 0.1;

            // Memory bar
            let mem_bar_rect = Rect::from_min_max(
                Pos2::new(x, row_rect.min.y + (ROW_HEIGHT - BAR_HEIGHT) / 2.0),
                Pos2::new(
                    x + rect.width() * 0.08,
                    row_rect.min.y + (ROW_HEIGHT + BAR_HEIGHT) / 2.0,
                ),
            );
            self.draw_bar(
                painter,
                mem_bar_rect,
                process.memory_percent,
                Color32::from_rgb(194, 137, 46),
            );
        }
    }
}

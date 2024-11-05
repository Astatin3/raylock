use egui::{Color32, Pos2, Rect, Stroke};
use std::collections::VecDeque;
use std::time::Instant;
use sysinfo::System;

use crate::configs;
use crate::graph::*;

pub struct CpuGraph {
    sys: System,
    cpu_graph: ResourceGraph,
    last_update: Instant,
}

impl CpuGraph {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_cpu_all();

        let mut cpu_graph =
            ResourceGraph::new("CPU Usage".to_string(), "%".to_string(), 0.0, 100.0);

        let cpu_count = sys.cpus().len();
        for i in 0..cpu_count {
            let hue = (i as f32 / cpu_count as f32) * 360.0;
            cpu_graph.add_line(format!("CPU {}", i), configs::GRAPH_STROKE);
        }

        Self {
            sys,
            cpu_graph,
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update).as_secs_f32() < 1.0 / UPDATES_PER_SECOND {
            return;
        }

        self.sys.refresh_cpu_all();
        // self.sys.refresh_memory();

        // Update CPU usage
        for (i, cpu) in self.sys.cpus().iter().enumerate() {
            self.cpu_graph.update_line(i, cpu.cpu_usage());
        }

        self.last_update = now;
    }

    pub fn render(&mut self, painter: &egui::Painter, rect: Rect) {
        self.cpu_graph.render(painter, rect);
    }
}

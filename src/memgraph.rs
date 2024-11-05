use egui::Rect;
use std::time::Instant;
use sysinfo::System;

use crate::configs;
use crate::graph::*;

pub struct MemGraph {
    sys: System,
    mem_graph: ResourceGraph,
    last_update: Instant,
}

impl MemGraph {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_memory();

        let mut memory_graph = ResourceGraph::new(
            "Memory Usage".to_string(),
            "GB".to_string(),
            0.0,
            sys.total_memory() as f32 / 1024.0 / 1024.0 / 1024.0,
        );

        // Add memory lines
        memory_graph.add_line("Used".to_string(), configs::GRAPH_STROKE);
        memory_graph.add_line("Cached".to_string(), configs::GRAPH_STROKE);

        Self {
            sys,
            mem_graph: memory_graph,
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update).as_secs_f32() < 1.0 / UPDATES_PER_SECOND {
            return;
        }

        // self.sys.refresh_cpu_all();
        self.sys.refresh_memory();

        // Update memory usage
        let total_gb = self.sys.total_memory() as f32 / 1024.0 / 1024.0 / 1024.0;
        let used_gb = self.sys.used_memory() as f32 / 1024.0 / 1024.0 / 1024.0;
        let cached_gb = (self.sys.total_memory() - self.sys.available_memory()) as f32
            / 1024.0
            / 1024.0
            / 1024.0;

        self.mem_graph.update_line(0, used_gb);
        self.mem_graph.update_line(1, cached_gb);

        self.last_update = now;
    }

    pub fn render(&mut self, painter: &egui::Painter, rect: Rect) {
        // self.cpu_graph.render(painter, cpu_rect);
        self.mem_graph.render(painter, rect);
    }
}

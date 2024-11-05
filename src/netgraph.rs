use egui::Rect;
use std::time::Instant;
use sysinfo::Process;
use sysinfo::ProcessRefreshKind;
use sysinfo::RefreshKind;
use sysinfo::System;

use crate::configs;
use crate::graph::*;

pub struct NetGraph {
    net: sysinfo::Networks,

    mem_graph: ResourceGraph,
    last_update: Instant,
}

impl NetGraph {
    pub fn new() -> Self {
        let mut net = sysinfo::Networks::new_with_refreshed_list();

        let mut memory_graph =
            ResourceGraph::new("Disk Usage".to_string(), " KB/s".to_string(), 0.0, 10.0);

        // Add memory lines
        memory_graph.add_line("U".to_string(), configs::UP_GRAPH_STROKE);
        memory_graph.add_line("D".to_string(), configs::DOWN_GRAPH_STROKE);

        Self {
            net,
            mem_graph: memory_graph,
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update).as_secs_f32() < 1.0 / UPDATES_PER_SECOND {
            return;
        }

        // self.mem_graph.

        // self.networks.refresh_cpu_all();

        let mut transmitted: f32 = 0.;
        let mut recieved: f32 = 0.;

        for (_, network) in &self.net {
            // println!("in: {} B", network.received());
            recieved += (network.received() as f32 / 1024.);
            transmitted += (network.transmitted() as f32 / 1024.);
        }

        self.net.refresh();

        self.mem_graph
            .update_line(0, transmitted * UPDATES_PER_SECOND);
        self.mem_graph.update_line(1, recieved * UPDATES_PER_SECOND);

        self.mem_graph.redo_max();

        self.last_update = now;
    }

    pub fn render(&mut self, painter: &egui::Painter, rect: Rect) {
        // self.cpu_graph.render(painter, cpu_rect);
        self.mem_graph.render(painter, rect);
    }
}

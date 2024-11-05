use egui::Rect;
use std::time::Instant;
use sysinfo::ProcessRefreshKind;
use sysinfo::RefreshKind;
use sysinfo::System;

use crate::configs;
use crate::graph::*;
use crate::table::ProcessTable;

pub struct DiskGraph {
    sys: sysinfo::System,
    mem_graph: ResourceGraph,
    last_update: Instant,

    proc_refresh: ProcessRefreshKind,
}

impl DiskGraph {
    pub fn new() -> Self {
        let proc_refresh = ProcessRefreshKind::new().with_disk_usage();

        let mut sys =
            sysinfo::System::new_with_specifics(RefreshKind::new().with_processes(proc_refresh));

        let mut memory_graph =
            ResourceGraph::new("Net Usage".to_string(), " MB/s".to_string(), 0.0, 10.0);

        // Add memory lines
        memory_graph.add_line("U".to_string(), configs::UP_GRAPH_STROKE);
        memory_graph.add_line("D".to_string(), configs::DOWN_GRAPH_STROKE);

        Self {
            sys,
            mem_graph: memory_graph,
            last_update: Instant::now(),
            proc_refresh,
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

        for (_, proc) in self.sys.processes() {
            // println!("in: {} B", network.received());
            let disk = proc.disk_usage();
            recieved += (disk.read_bytes as f32 / 1024. / 1024.) * UPDATES_PER_SECOND;

            transmitted += (disk.written_bytes as f32 / 1024. / 1024.) * UPDATES_PER_SECOND;
        }

        self.sys.refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::All,
            false,
            self.proc_refresh,
        );

        self.mem_graph.update_line(0, transmitted);
        self.mem_graph.update_line(1, recieved);

        self.mem_graph.redo_max();

        self.last_update = now;
    }

    pub fn render(&mut self, painter: &egui::Painter, rect: Rect) {
        // self.cpu_graph.render(painter, cpu_rect);
        self.mem_graph.render(painter, rect);
    }
}

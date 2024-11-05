use egui::ecolor::linear_u8_from_linear_f32;
use egui::emath::Rot2;
use egui::epaint::TextShape;
use egui::{Color32, FontId, Galley, Image, Painter, Pos2, Rect, Rounding};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::default::Default;
use std::f32::consts::PI;

use crate::cpugraph::CpuGraph;
use crate::diskgraph::DiskGraph;
use crate::infopane::InfoPane;
use crate::memgraph::MemGraph;
use crate::netgraph::NetGraph;
use crate::table::{ProcessTable, BAR_HEIGHT, ROW_HEIGHT};
use crate::ui::get_corners;
use crate::{configs::*, ui};

// use crate::{graph::CpuGraph, ui};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CornerTypes {
    SQUARE,
    Ang45,
    Ang30,
    Ang60,
}

#[derive(Clone)]
pub enum TitleFormats {
    TOP { text: Option<String> },
    SIDE { text: Option<String> },
}

impl Default for TitleFormats {
    fn default() -> Self {
        TitleFormats::TOP { text: None }
    }
}

const DEFAULT_CORNERS: [CornerTypes; 4] = [
    CornerTypes::Ang30,
    CornerTypes::Ang60,
    CornerTypes::Ang60,
    CornerTypes::Ang30,
];

// Enum to represent different types of panes
#[derive(Deserialize, Serialize, Clone)]
// #[serde(tag = "type")]
pub enum PaneType {
    Info,
    CpuGraph,
    MemGraph,
    NetGraph,
    DiskGraph,
    ProcTable,
    No,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum SplitType {
    H,
    V,
}

// Structure to store runtime data for different pane types
pub enum PaneData {
    Info { info_man: InfoPane },
    CpuGraph { cpu_graph: CpuGraph },
    MemGraph { mem_graph: MemGraph },
    NetGraph { net_graph: NetGraph },
    DiskGraph { disk_graph: DiskGraph },
    ProcTable { proc_table: ProcessTable },
    No {},
}

impl Default for PaneData {
    fn default() -> Self {
        PaneData::No {}
    }
}

impl PaneData {
    pub fn new(pane: &Pane) -> Self {
        match pane {
            Pane::Split { .. } => PaneData::No {},

            Pane::Leaf { pane_type, .. } => match pane_type {
                PaneType::Info => PaneData::Info {
                    info_man: InfoPane::new(),
                },
                PaneType::CpuGraph => PaneData::CpuGraph {
                    cpu_graph: CpuGraph::new(),
                },
                PaneType::MemGraph => PaneData::MemGraph {
                    mem_graph: MemGraph::new(),
                },
                PaneType::NetGraph => PaneData::NetGraph {
                    net_graph: NetGraph::new(),
                },
                PaneType::DiskGraph => PaneData::DiskGraph {
                    disk_graph: DiskGraph::new(),
                },
                PaneType::ProcTable => PaneData::ProcTable {
                    proc_table: ProcessTable::new(30, 50, 0),
                },
                PaneType::No {} => PaneData::No {},
            },
        }
    }
}

// Main pane structure that represents either a split or leaf node
#[derive(Deserialize, Serialize)]
// #[default]
#[serde(tag = "kind")]
pub enum Pane {
    Leaf {
        pane_type: PaneType,
        corners: [CornerTypes; 4],
        #[serde(skip, default = "get_default_rect")]
        rect: Rect,
        #[serde(skip, default = "get_default_rect")]
        inner_rect: Rect,
        #[serde(skip)]
        container_points: Vec<Pos2>,
        #[serde(skip)]
        title_type: TitleFormats,
    },
    Split {
        direction: SplitType,
        bias: f32,
        #[serde(skip)]
        first: Box<PaneInstance>,
        #[serde(skip)]
        second: Box<PaneInstance>,
        a: Box<Pane>,
        b: Box<Pane>,
    },
}

const DEFAULT_RECT: Rect = Rect {
    min: Pos2 { x: 0., y: 0. },
    max: Pos2 { x: 0., y: 0. },
};
const DEFAULT_POINTS: Vec<Pos2> = Vec::new();

fn get_default_rect() -> Rect {
    DEFAULT_RECT
}

impl Default for Pane {
    fn default() -> Self {
        Pane::Leaf {
            pane_type: PaneType::No {},
            corners: DEFAULT_CORNERS,
            rect: DEFAULT_RECT,
            inner_rect: DEFAULT_RECT,
            container_points: DEFAULT_POINTS,
            title_type: TitleFormats::default(),
        }
    }
}
impl Default for PaneInstance {
    fn default() -> Self {
        PaneInstance {
            config: Pane::default(),
            runtime_data: PaneData::default(),
        }
    }
}

// impl Pane {
//     fn
// }
// Runtime instance of a pane that includes both config and runtime data
#[derive(Deserialize, Serialize)]
pub struct PaneInstance {
    config: Pane,
    #[serde(skip)]
    runtime_data: PaneData,
}

impl PaneInstance {
    // Create a new instance from configuration
    fn from_config(config: Pane) -> Self {
        PaneInstance {
            runtime_data: PaneData::new(&config),
            config: config,
        }
    }

    pub fn precalc(&mut self, rect: Rect) {
        match &mut self.config {
            Pane::Split {
                direction,
                bias,
                first,
                second,
                ..
            } => {
                let (first_rect, second_rect) = if *direction == SplitType::V {
                    let split_x = rect.min.x + rect.width() * bias.clone();
                    (
                        Rect::from_min_max(rect.min, egui::pos2(split_x, rect.max.y)),
                        Rect::from_min_max(egui::pos2(split_x, rect.min.y), rect.max),
                    )
                } else {
                    let mut split_y = rect.min.y + rect.height() * bias.clone();
                    (
                        Rect::from_min_max(rect.min, egui::pos2(rect.max.x, split_y)),
                        Rect::from_min_max(egui::pos2(rect.min.x, split_y), rect.max),
                    )
                };

                first.precalc(first_rect);
                second.precalc(second_rect);
            }
            Pane::Leaf {
                pane_type,
                corners,
                rect: rect2,
                inner_rect,
                container_points,
                title_type: title_type,
            } => {
                container_points.clone_from(&get_corners(rect, corners.to_owned()));
                rect2.clone_from(&rect);
                inner_rect.clone_from(&ui::find_largest_rectangle(container_points).unwrap());

                let text = Some(
                    match pane_type {
                        PaneType::Info => "INFO",
                        PaneType::CpuGraph => "CPU",
                        PaneType::MemGraph => "MEM",
                        PaneType::ProcTable => "PROC",
                        PaneType::NetGraph => "NET",
                        PaneType::DiskGraph => "DISK",
                        _ => "ERR",
                    }
                    .to_string(),
                );

                title_type.clone_from(&if ((inner_rect.left() - rect2.left())
                    + (rect2.right() - inner_rect.right()))
                    > ((inner_rect.top() - rect2.top()) + (rect2.bottom() - inner_rect.bottom()))
                {
                    TitleFormats::SIDE { text: text }
                } else {
                    TitleFormats::TOP { text: text }
                });

                match pane_type {
                    PaneType::ProcTable { .. } => {
                        if let PaneData::ProcTable { proc_table } = self.runtime_data.borrow_mut() {
                            proc_table.row_count =
                                ((inner_rect.height() - BAR_HEIGHT) / ROW_HEIGHT).floor() as usize;
                            // cpu_proc_table.row_count = 10;
                        }
                    }
                    _ => {}
                }
                // let adj_rect = background_render(painter, rect, *corners);
            }
        }
    }

    // Render the pane and its children
    pub fn render(&mut self, painter: &Painter) {
        match &mut self.config {
            Pane::Split { first, second, .. } => {
                first.render(painter);
                second.render(painter);
            }
            Pane::Leaf {
                pane_type,
                rect,
                inner_rect,
                container_points,
                title_type,
                ..
            } => {
                painter.add(egui::Shape::convex_polygon(
                    container_points.to_owned(),
                    BACKGROUND_2,
                    egui::Stroke::new(0.5, TEXT_COLOR),
                ));

                match title_type {
                    TitleFormats::SIDE { text } => {
                        let galley = painter.layout_no_wrap(
                            text.as_ref().unwrap().to_owned(),
                            TITLE_FONT,
                            TEXT_COLOR,
                        );
                        let size = galley.size();
                        let mid_x = rect.min.x + (f32::abs(rect.min.x - inner_rect.min.x) / 2.)
                            - (size.y / 2.)
                            + PANE_GAP;
                        let mid_y = (inner_rect.min.y
                            + f32::abs(inner_rect.min.y - inner_rect.max.y) / 2.)
                            + size.x / 2.;
                        painter.add(
                            TextShape::new(Pos2 { x: mid_x, y: mid_y }, galley, Color32::WHITE)
                                .with_angle(-PI / 2.),
                        );
                    }
                    TitleFormats::TOP { text } => {
                        let galley = painter.layout_no_wrap(
                            text.as_ref().unwrap().to_owned(),
                            TITLE_FONT,
                            TEXT_COLOR,
                        );
                        let size = galley.size();
                        let mid_x = rect.min.x
                            + (f32::abs(inner_rect.min.x - inner_rect.max.x) / 2.)
                            - (size.x / 2.);
                        let mid_y = (rect.min.y + f32::abs(rect.min.y - inner_rect.min.y) / 2.)
                            - (size.y / 2.)
                            + PANE_GAP;
                        painter.add(TextShape::new(
                            Pos2 { x: mid_x, y: mid_y },
                            galley,
                            Color32::WHITE,
                        ));
                    }
                }

                painter.rect_stroke(
                    inner_rect.to_owned(),
                    0.0,
                    egui::Stroke::new(0.25, TEXT_COLOR),
                );

                match pane_type {
                    PaneType::Info => {
                        render_info(painter, inner_rect.to_owned(), &mut self.runtime_data);
                    }
                    PaneType::CpuGraph => {
                        render_cpu_graph(painter, inner_rect.to_owned(), &mut self.runtime_data);
                    }
                    PaneType::MemGraph => {
                        render_mem_graph(painter, inner_rect.to_owned(), &mut self.runtime_data);
                    }
                    PaneType::NetGraph => {
                        render_net_graph(painter, inner_rect.to_owned(), &mut self.runtime_data);
                    }
                    PaneType::DiskGraph => {
                        render_disk_graph(painter, inner_rect.to_owned(), &mut self.runtime_data);
                    }
                    PaneType::ProcTable => {
                        render_proc_table(painter, inner_rect.to_owned(), &mut self.runtime_data);
                    }
                    PaneType::No => {}
                }
            }
        }
    }
}

pub fn render_info(painter: &Painter, rect: Rect, data: &mut PaneData) {
    if let PaneData::Info { info_man } = data {
        info_man.update();
        info_man.render(painter, rect);
    }
}

// Example rendering functions for different pane types
pub fn render_cpu_graph(painter: &Painter, rect: Rect, data: &mut PaneData) {
    if let PaneData::CpuGraph { cpu_graph } = data {
        cpu_graph.update();
        cpu_graph.render(painter, rect);
    }
}

pub fn render_mem_graph(painter: &Painter, rect: Rect, data: &mut PaneData) {
    if let PaneData::MemGraph { mem_graph } = data {
        mem_graph.update();
        mem_graph.render(painter, rect);
    }
}

pub fn render_net_graph(painter: &Painter, rect: Rect, data: &mut PaneData) {
    if let PaneData::NetGraph { net_graph } = data {
        net_graph.update();
        net_graph.render(painter, rect);
    }
}

pub fn render_disk_graph(painter: &Painter, rect: Rect, data: &mut PaneData) {
    if let PaneData::DiskGraph { disk_graph } = data {
        disk_graph.update();
        disk_graph.render(painter, rect);
    }
}

pub fn render_proc_table(painter: &Painter, rect: Rect, data: &mut PaneData) {
    if let PaneData::ProcTable { proc_table } = data {
        proc_table.update();
        proc_table.render(painter, rect);
    }
}

// Function to load pane configuration from JSON
pub fn load_pane_config(json: &str) -> Result<PaneInstance, serde_json::Error> {
    let config: Pane = serde_json::from_str(json)?;

    // Create the pane hierarchy with runtime data
    Ok(create_pane_instance(config))
}

// Helper function to create the pane hierarchy
pub fn create_pane_instance(config: Pane) -> PaneInstance {
    match config {
        Pane::Split {
            direction,
            bias,
            a,
            b,
            ..
        } => {
            let first = Box::new(create_pane_instance(*a));
            let second = Box::new(create_pane_instance(*b));

            PaneInstance {
                config: Pane::Split {
                    direction,
                    bias,
                    first,
                    second,
                    a: Box::new(Pane::default()),
                    b: Box::new(Pane::default()),
                },
                runtime_data: PaneData::default(),
            }
        }
        leaf => PaneInstance::from_config(leaf),
    }
}

// Example JSON configuration
pub const EXAMPLE_CONFIG: &str = r#"
{
    "kind": "Split",
    "direction": "V",
    "bias": 0.5,
    "a": {
        "kind": "Split",
        "direction": "H",
        "bias": 0.2,
        "a": {
            "kind": "Leaf",
            "corners": ["Ang60", "Ang30", "Ang60", "Ang30"],
            "pane_type": "Info"
        },
        "b": {
            "kind": "Leaf",
            "corners": ["Ang30", "Ang60", "SQUARE", "SQUARE"],
            "pane_type": "ProcTable"
        }
    },
    "b": {
        "kind": "Split",
        "direction": "H",
        "bias": 0.5,
        "a": {
            "kind": "Split",
            "direction": "H",
            "bias": 0.5,
            "a": {
                "kind": "Leaf",
                "corners": ["Ang60", "SQUARE", "SQUARE", "Ang30"],
                "pane_type": "CpuGraph"
            },
            "b": {
                "kind": "Leaf",
                "corners": ["Ang60", "SQUARE", "SQUARE", "Ang30"],
                "pane_type": "MemGraph"
            }
        },
        "b": {
            "kind": "Split",
            "direction": "H",
            "bias": 0.5,
            "a": {
                "kind": "Leaf",
                "corners": ["Ang60", "SQUARE", "SQUARE", "Ang30"],
                "pane_type": "NetGraph"
            },
            "b": {
                "kind": "Leaf",
                "corners": ["Ang60", "SQUARE", "SQUARE", "Ang30"],
                "pane_type": "DiskGraph"
            }
        }
    }
}
"#;

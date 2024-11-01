use egui::{Color32, Painter, Pos2, Rect};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{graph::CpuGraph, ui};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CornerTypes {
    SQUARE,
    Ang45,
    Ang30,
    Ang60,
}

// Serializable configuration types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum PaneTypeConfig {
    Solid(SolidPaneConfig),
    Text(TextPaneConfig),
    Gradient(GradientPaneConfig),
}

// Runtime-only pane type that includes non-serializable data
// #[derive(Clone)]
pub enum PaneType {
    Solid(SolidPane),
    Text(TextPane),
    Gradient(GradientPane),
}

// Serializable configs
#[derive(Clone, Serialize, Deserialize)]
pub struct SolidPaneConfig {
    pub color: [u8; 3],
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TextPaneConfig {
    pub text: String,
    pub font_size: f32,
    pub color: [u8; 3],
    pub background_color: Option<[u8; 3]>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GradientPaneConfig {
    pub start_color: [u8; 3],
    pub end_color: [u8; 3],
    pub horizontal: bool,
}

// Runtime types with additional non-serializable data
#[derive(Clone)]
pub struct SolidPane {
    config: SolidPaneConfig,
    // Runtime-only fields
    cached_color: Color32,
}

#[derive(Clone)]
pub struct TextPane {
    config: TextPaneConfig,
    // Runtime-only fields
    cached_color: Color32,
    cached_bg_color: Option<Color32>,
    cached_font: egui::FontId,
}

// #[derive(Clone)]
pub struct GradientPane {
    config: GradientPaneConfig,
    // Runtime-only fields
    cached_start_color: Color32,
    cached_end_color: Color32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PaneConfig {
    pub id: String,
    #[serde(default)]
    pub split: Option<SplitConfig>,
    #[serde(default)]
    pub pane_type: Option<PaneTypeConfig>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SplitConfig {
    pub direction: SplitDirection,
    pub ratio: f32,
    pub children: Vec<PaneConfig>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub root: PaneConfig,
    #[serde(default)]
    pub default_pane_type: Option<PaneTypeConfig>,
}

// Runtime representation of the layout
pub struct RuntimePane {
    pub id: String,
    pub split: Option<RuntimeSplit>,
    pub pane_type: Option<PaneType>,
}

pub struct RuntimeSplit {
    pub direction: SplitDirection,
    pub ratio: f32,
    pub children: Vec<RuntimePane>,
}

// Conversion implementations
impl From<PaneTypeConfig> for PaneType {
    fn from(config: PaneTypeConfig) -> Self {
        match config {
            PaneTypeConfig::Solid(config) => PaneType::Solid(SolidPane {
                cached_color: Color32::from_rgb(config.color[0], config.color[1], config.color[2]),
                config,
            }),
            PaneTypeConfig::Text(config) => PaneType::Text(TextPane {
                cached_color: Color32::from_rgb(config.color[0], config.color[1], config.color[2]),
                cached_bg_color: config
                    .background_color
                    .map(|c| Color32::from_rgb(c[0], c[1], c[2])),
                cached_font: egui::FontId::proportional(config.font_size),
                config,
            }),
            PaneTypeConfig::Gradient(config) => PaneType::Gradient(GradientPane {
                cached_start_color: Color32::from_rgb(
                    config.start_color[0],
                    config.start_color[1],
                    config.start_color[2],
                ),
                cached_end_color: Color32::from_rgb(
                    config.end_color[0],
                    config.end_color[1],
                    config.end_color[2],
                ),

                config,
            }),
        }
    }
}

impl PaneType {
    fn render(&self, painter: &Painter, rect: Rect) {
        ui::background_render(
            painter,
            rect,
            [
                CornerTypes::Ang30,
                CornerTypes::Ang60,
                CornerTypes::Ang30,
                CornerTypes::Ang60,
            ],
        );
        match self {
            PaneType::Solid(pane) => {
                // painter.rect_filled(rect, 0.0, pane.cached_color);
                // painter.rect_stroke(rect, 0.0, (1.0, Color32::BLACK));
            }
            PaneType::Text(pane) => {
                // if let Some(bg_color) = pane.cached_bg_color {
                //     painter.rect_filled(rect, 0.0, bg_color);
                // }

                // painter.text(
                //     rect.center(),
                //     egui::Align2::CENTER_CENTER,
                //     &pane.config.text,
                //     pane.cached_font.clone(),
                //     pane.cached_color,
                // );
            }
            PaneType::Gradient(pane) => {
                let mut pane2 = pane;
                // pane.cpu_graph.update();
                // pane.cpu_graph.render(painter, rect);
                // if pane.config.horizontal {
                //     painter.rect_filled(rect, 0.0, pane.cached_start_color); // Simplified
                // } else {
                //     painter.rect_filled(rect, 0.0, pane.cached_end_color); // Simplified
                // }
            }
        }
    }
}

pub struct PaneRenderer {
    runtime_config: RuntimePane,
    default_pane_type: Option<PaneType>,
}

impl PaneRenderer {
    pub fn new(config: LayoutConfig) -> Self {
        Self {
            runtime_config: Self::convert_config(&config.root),
            default_pane_type: config.default_pane_type.map(Into::into),
        }
    }

    fn convert_config(config: &PaneConfig) -> RuntimePane {
        RuntimePane {
            id: config.id.clone(),
            split: config.split.as_ref().map(|split| RuntimeSplit {
                direction: split.direction,
                ratio: split.ratio,
                children: split.children.iter().map(Self::convert_config).collect(),
            }),
            pane_type: config.pane_type.clone().map(Into::into),
        }
    }

    pub fn render(&self, painter: &Painter, rect: &Rect) {
        self.render_pane(painter, &rect, &self.runtime_config);
    }

    fn render_pane(&self, painter: &Painter, rect: &Rect, pane: &RuntimePane) {
        if let Some(split) = &pane.split {
            if !split.children.is_empty() {
                let rects =
                    self.split_rect(rect, split.direction, split.ratio, split.children.len());

                for (child, child_rect) in split.children.iter().zip(&rects) {
                    self.render_pane(painter, child_rect, child);
                }

                // Draw split lines
                let split_line_color = Color32::from_gray(128);
                match split.direction {
                    SplitDirection::Horizontal => {
                        for rect in rects.windows(2) {
                            let x = rect[0].max.x;
                            painter.line_segment(
                                [Pos2::new(x, rect[0].min.y), Pos2::new(x, rect[0].max.y)],
                                (1.0, split_line_color),
                            );
                        }
                    }
                    SplitDirection::Vertical => {
                        for rect in rects.windows(2) {
                            let y = rect[0].max.y;
                            painter.line_segment(
                                [Pos2::new(rect[0].min.x, y), Pos2::new(rect[0].max.x, y)],
                                (1.0, split_line_color),
                            );
                        }
                    }
                }
            }
        } else {
            // Render leaf pane
            let pane_type = &mut pane.pane_type.as_ref();

            if let Some(pane_type) = pane_type {
                pane_type.render(painter, *rect);
            }
        }
    }

    fn split_rect(
        &self,
        rect: &Rect,
        direction: SplitDirection,
        ratio: f32,
        count: usize,
    ) -> Vec<Rect> {
        let mut rects = Vec::with_capacity(count);
        let size = match direction {
            SplitDirection::Horizontal => rect.width(),
            SplitDirection::Vertical => rect.height(),
        };

        let first_size = size * ratio;
        let remaining_size = size - first_size;
        let size_per_remaining = if count > 1 {
            remaining_size / (count as f32 - 1.0)
        } else {
            0.0
        };

        for i in 0..count {
            let (start, end) = match direction {
                SplitDirection::Horizontal => {
                    let start = if i == 0 {
                        rect.min.x
                    } else {
                        rect.min.x + first_size + size_per_remaining * (i as f32 - 1.0)
                    };
                    let end = if i == 0 {
                        rect.min.x + first_size
                    } else {
                        start + size_per_remaining
                    };
                    (Pos2::new(start, rect.min.y), Pos2::new(end, rect.max.y))
                }
                SplitDirection::Vertical => {
                    let start = if i == 0 {
                        rect.min.y
                    } else {
                        rect.min.y + first_size + size_per_remaining * (i as f32 - 1.0)
                    };
                    let end = if i == 0 {
                        rect.min.y + first_size
                    } else {
                        start + size_per_remaining
                    };
                    (Pos2::new(rect.min.x, start), Pos2::new(rect.max.x, end))
                }
            };
            rects.push(Rect::from_min_max(start, end));
        }
        rects
    }
}

// // Example JSON configuration:
// const EXAMPLE_CONFIG: &str = r#"
// {
//     "root": {
//         "id": "root",
//         "split": {
//             "direction": "Horizontal",
//             "ratio": 0.3,
//             "children": [
//                 {
//                     "id": "left",
//                     "pane_type": {
//                         "type": "Solid",
//                         "config": {
//                             "color": [100, 150, 200]
//                         }
//                     }
//                 },
//                 {
//                     "id": "right",
//                     "split": {
//                         "direction": "Vertical",
//                         "ratio": 0.6,
//                         "children": [
//                             {
//                                 "id": "right_top",
//                                 "pane_type": {
//                                     "type": "Text",
//                                     "config": {
//                                         "text": "Hello World",
//                                         "font_size": 24.0,
//                                         "color": [255, 255, 255],
//                                         "background_color": [50, 50, 150]
//                                     }
//                                 }
//                             },
//                             {
//                                 "id": "right_bottom",
//                                 "pane_type": {
//                                     "type": "Gradient",
//                                     "config": {
//                                         "start_color": [200, 100, 100],
//                                         "end_color": [100, 200, 100],
//                                         "horizontal": true
//                                     }
//                                 }
//                             }
//                         ]
//                     }
//                 }
//             ]
//         }
//     },
//     "default_pane_type": {
//         "type": "Solid",
//         "config": {
//             "color": [200, 200, 200]
//         }
//     }
// }
// "#;

// // Example usage
// pub fn example_usage(ctx: &egui::Context) {
//     // Parse configuration
//     let config: LayoutConfig = serde_json::from_str(EXAMPLE_CONFIG).unwrap();
//     let pane_renderer = PaneRenderer::new(config);

//     // egui::CentralPanel::default().show(ctx, |ui| {
//     //     let painter = ui.painter();

//     // });
// }

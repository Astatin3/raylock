use egui::{Color32, Painter, Pos2, Rect, Vec2};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::MutexGuard;
// use serde::

#[derive(Default)]
pub struct AuthState {
    pub password: String,
    pub to_be_submitted: bool,
    pub failed_attempts: u16,
}

// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// pub enum SplitDirection {
//     Horizontal,
//     Vertical,
// }

// // #[derive(Sized)]
// pub struct PaneConfig {
//     pub id: String,
//     pub split: Option<(SplitDirection, f32)>,
//     pub children: Option<Vec<PaneConfig>>,
//     pub color: Option<Color32>, // Store color in config for consistency
//     pub callback: Option<Box<dyn Fn(&Painter, Rect, [CornerTypes; 4])>>,
//     pub corners: [CornerTypes; 4],
// }

// // #[derive(Debug)]
// pub struct PaneRenderer {
//     config: PaneConfig,
// }

// impl PaneConfig {
//     pub fn new(id: impl Into<String>) -> Self {
//         Self {
//             id: id.into(),
//             split: None,
//             children: None,
//             color: None,
//             callback: None,
//             corners: [
//                 CornerTypes::Ang60,
//                 CornerTypes::Ang30,
//                 CornerTypes::SQUARE,
//                 CornerTypes::Ang45,
//             ],
//         }
//     }

//     pub fn with_split(mut self, direction: SplitDirection, ratio: f32) -> Self {
//         self.split = Some((direction, ratio.clamp(0.0, 1.0)));
//         self
//     }

//     pub fn with_children(mut self, children: Vec<PaneConfig>) -> Self {
//         self.children = Some(children);
//         self
//     }

//     pub fn with_callback(
//         mut self,
//         callback: impl Fn(&Painter, Rect, [CornerTypes; 4]) + 'static,
//     ) -> Self {
//         self.callback = Some(Box::new(callback));
//         self
//     }

//     pub fn with_corners(mut self, corners: [CornerTypes; 4]) -> Self {
//         self.corners = corners;
//         self
//     }

//     pub fn with_corner_tl(mut self, corner: CornerTypes) -> Self {
//         self.corners[0] = corner;
//         self
//     }
//     pub fn with_corner_tr(mut self, corner: CornerTypes) -> Self {
//         self.corners[1] = corner;
//         self
//     }
//     pub fn with_corner_br(mut self, corner: CornerTypes) -> Self {
//         self.corners[2] = corner;
//         self
//     }
//     pub fn with_corner_bl(mut self, corner: CornerTypes) -> Self {
//         self.corners[3] = corner;
//         self
//     }

//     pub fn with_random_color(mut self) -> Self {
//         let mut rng = rand::thread_rng();
//         self.color = Some(Color32::from_rgb(
//             rng.gen_range(0..255),
//             rng.gen_range(0..255),
//             rng.gen_range(0..255),
//         ));
//         self
//     }
// }

// impl PaneRenderer {
//     pub fn new(config: PaneConfig) -> Self {
//         Self { config }
//     }

//     pub fn render(&self, painter: &Painter, rect: Rect) {
//         self.render_pane(painter, rect, &self.config);
//     }

//     fn render_pane(&self, painter: &Painter, rect: Rect, pane: &PaneConfig) {
//         if let Some((direction, ratio)) = &pane.split {
//             if let Some(children) = &pane.children {
//                 if children.len() >= 2 {
//                     let (first_rect, second_rect) = self.split_rect(rect, *direction, *ratio);

//                     // Render first child
//                     self.render_pane(painter, first_rect, &children[0]);

//                     // Render second child
//                     self.render_pane(painter, second_rect, &children[1]);

//                     // Render any additional children (will be stacked after the split)
//                     let remaining_rect = match direction {
//                         SplitDirection::Horizontal => Rect::from_min_size(
//                             Pos2::new(second_rect.max.x, rect.min.y),
//                             Vec2::new(rect.max.x - second_rect.max.x, rect.height()),
//                         ),
//                         SplitDirection::Vertical => Rect::from_min_size(
//                             Pos2::new(rect.min.x, second_rect.max.y),
//                             Vec2::new(rect.width(), rect.max.y - second_rect.max.y),
//                         ),
//                     };

//                     for child in children.iter().skip(2) {
//                         self.render_pane(painter, remaining_rect, child);
//                     }

//                     // Draw split line
//                     let split_line_color = Color32::from_gray(128);
//                     match direction {
//                         SplitDirection::Horizontal => {
//                             let x = first_rect.max.x;
//                             // painter.line_segment(
//                             //     [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
//                             //     (1.0, split_line_color),
//                             // );
//                         }
//                         SplitDirection::Vertical => {
//                             let y = first_rect.max.y;
//                             // painter.line_segment(
//                             //     [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
//                             //     (1.0, split_line_color),
//                             // );
//                         }
//                     }
//                 }
//             }
//         } else {
//             if !pane.callback.is_none() {
//                 // (&self.config).corners[0] = CornerTypes::Ang30;
//                 pane.callback.as_ref().unwrap()(painter, rect, self.config.corners);
//                 return;
//             }
//         }
//     }

//     fn split_rect(&self, rect: Rect, direction: SplitDirection, ratio: f32) -> (Rect, Rect) {
//         match direction {
//             SplitDirection::Horizontal => {
//                 let split_x = rect.min.x + rect.width() * ratio;
//                 let first = Rect::from_min_max(rect.min, Pos2::new(split_x, rect.max.y));
//                 let second = Rect::from_min_max(Pos2::new(split_x, rect.min.y), rect.max);
//                 (first, second)
//             }
//             SplitDirection::Vertical => {
//                 let split_y = rect.min.y + rect.height() * ratio;
//                 let first = Rect::from_min_max(rect.min, Pos2::new(rect.max.x, split_y));
//                 let second = Rect::from_min_max(Pos2::new(rect.min.x, split_y), rect.max);
//                 (first, second)
//             }
//         }
//     }
// }

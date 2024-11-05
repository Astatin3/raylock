use battery::units::electric_potential::volt;
use battery::units::energy::joule;
use battery::units::power::watt;
use battery::units::time::minute;
use battery::units::time::second;
use battery::Manager;
use chrono::Local;
use egui::Align2;
use egui::Galley;
use egui::Pos2;
use egui::Rect;
use std::env;
use std::time::Instant;

use crate::configs::*;
use crate::graph::*;

pub struct InfoPane {
    manager: Manager,
    unamea: String,
    state: BatteryState,
    last_update: Instant,
}

struct BatteryState {
    percent: f32,
    rate: f32,
    time: f32,
    discharging: bool,
}

impl InfoPane {
    pub fn new() -> Self {
        Self {
            manager: Manager::new().unwrap(),
            unamea: os_info::get().to_string(),
            state: BatteryState {
                percent: 0.,
                rate: 0.,
                time: 0.,
                discharging: false,
            },
            last_update: Instant::now(),
        }
    }

    // fn get_unamea() ->

    fn get_percent(&mut self) -> Result<BatteryState, battery::errors::Error> {
        // let mut percent = 1.;
        // let mut rate = 0.;
        // let mut min_time;

        for (idx, unsafe_battery) in self.manager.batteries()?.enumerate() {
            let battery = unsafe_battery.unwrap();
            let percent = battery.energy().get::<joule>() / battery.energy_full().get::<joule>();
            let rate = battery.energy_rate().get::<watt>();

            let time_to_empty = battery.time_to_empty();

            let time: f32 = if !time_to_empty.is_none() {
                time_to_empty.unwrap().get::<minute>()
            } else if !battery.time_to_full().is_none() {
                battery.time_to_full().unwrap().get::<minute>()
            } else {
                0.
            };

            return Ok((BatteryState {
                percent: percent * 100.0,
                rate: rate,
                time: time,
                discharging: !time_to_empty.is_none(),
            }));

            // percent *= battery
            //     .unwrap()
            //     .state_of_charge()
            //     .get::<battery::units::ratio::percent>();
        }

        Ok((BatteryState {
            percent: 0.,
            rate: 0.,
            time: 0.,
            discharging: false,
        }))
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update).as_secs_f32() < 1.0 / UPDATES_PER_SECOND {
            return;
        }

        self.state = self.get_percent().unwrap();
        self.last_update = Instant::now();
    }

    pub fn render(&mut self, painter: &egui::Painter, rect: Rect) {
        let now = Local::now();

        let mut y = 0.;

        painter.text(
            Pos2 {
                x: rect.min.x,
                y: rect.min.y,
            },
            Align2::LEFT_TOP,
            format!("TIME: {}", now.format("UTC%Z %H:%M:%S%.f (%Y-%d-%m)")),
            TEXT_FONT,
            TEXT_COLOR,
        );

        y += TEXT_FONT.size;

        // let galley = painter.layout_no_wrap(format!("{}%", self.percent), TITLE_FONT, TEXT_COLOR);
        //

        painter.text(
            Pos2 {
                x: rect.min.x,
                y: rect.min.y + y,
            },
            Align2::LEFT_TOP,
            format!(
                "BAT: {}% {} ({} min) ({} W)",
                self.state.percent,
                {
                    if self.state.discharging {
                        "discharging"
                    } else {
                        "charging"
                    }
                },
                self.state.time,
                self.state.rate,
            ),
            TEXT_FONT,
            TEXT_COLOR,
        );

        y += TEXT_FONT.size;

        painter.text(
            Pos2 {
                x: rect.min.x,
                y: rect.min.y + y,
            },
            Align2::LEFT_TOP,
            format!("OS: {}", self.unamea,),
            TEXT_FONT,
            TEXT_COLOR,
        );
    }
}

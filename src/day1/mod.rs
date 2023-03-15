use std::time::Duration;

use eframe::egui;
use egui::{Color32, RichText, Sense, Slider, Stroke, TextEdit, Ui};

use crate::View;

mod parse;
mod solution;

pub struct Screen {
    input: Input,
    animation: Animation,
}

pub struct Input {
    enable_custom_input: bool,
    custom_input: String,
    is_input_valid: bool,
    current_input: String,
    solution: Option<(i32, i32)>,
}

pub struct Animation {
    data: Option<Vec<(i32, i32)>>,
    drawn_steps: Vec<(i32, i32)>,
    path_crossed: Option<(i32, i32)>,
    frame: usize,
    play: bool,
    step: bool,
    speed: u32,
}

impl Animation {
    fn new() -> Self {
        Animation {
            data: None,
            drawn_steps: Vec::new(),
            path_crossed: None,
            step: false,
            play: false,
            speed: 1,
            frame: 0,
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            input: Input {
                enable_custom_input: false,
                is_input_valid: true,
                custom_input: String::from(""),
                current_input: String::from(""),
                solution: None,
            },
            animation: Animation::new(),
        }
    }
}

impl Screen {
    fn update_frame(&mut self) {
        let animation_data = self.animation.data.as_ref().unwrap();
        let new_frame = animation_data.get(self.animation.frame);

        if let Some(_) = new_frame {
            self.animation.frame = self.animation.frame + 1;
            self.animation.drawn_steps = animation_data
                .get(0..self.animation.frame)
                .unwrap()
                .to_vec();
        } else {
            self.animation.play = false;
        }

        self.animation.step = false;
    }

    fn update_input(&mut self, input: &str) {
        if input != self.input.current_input {
            self.input.current_input = input.to_string();

            self.input.solution = None;
            self.animation = Animation::new();
            self.input.is_input_valid = solution::is_input_valid(input);

            if self.input.is_input_valid {
                self.input.solution = solution::solve(input);
                let (data, repetition) = solution::animation_data(input);

                self.animation.data = Some(data);
                self.animation.path_crossed = repetition;
            }
        }
    }
}

impl View for Screen {
    fn get_summary(&self) -> String {
        r#"You get input passed as a line of instructions:
Ex.
    R2, L13, R7
    R20, R200, L12

Your character starts at (0, 0) looking North and move according to the instructions (L10 = Turn left and move 10 tiles).

Part 1: Execute the instructions and return your position in Taxi Cab distance
Part 2: Return Taxi Cab distance to the first place where your paths crossed."#.to_string()
    }

    fn input(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        if self.input.enable_custom_input {
            self.update_input(&self.input.custom_input.clone());
        } else {
            let default_string = include_str!("input.txt");
            self.update_input(default_string);
        }

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.input.enable_custom_input, "Custom input");

            if self.input.enable_custom_input {
                ui.add(
                    egui::TextEdit::singleline(&mut self.input.custom_input)
                        .hint_text("Paste your input here..."),
                );
            }
        });

        ui.horizontal(|ui| {
            if self.input.is_input_valid {
                ui.label("Input is VALID.");
            } else {
                ui.label(RichText::new("Your input is INVALID!").color(Color32::DARK_RED));
                ui.label("(Example input: R1, L2, E10)");
            }
        });

        ui.horizontal(|ui| {
            ui.set_enabled(false);
            ui.set_max_width(ui.available_width());
            ui.add_sized(
                ui.available_size(),
                TextEdit::multiline(&mut self.input.current_input),
            );
        });
    }

    fn solution(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        if let Some((part1, part2)) = self.input.solution {
            ui.heading("Solution");

            ui.label(format!("Part 1: {}", part1));
            ui.label(format!("Part 2: {}", part2));

            ui.separator();
        }

        if self.animation.step || self.animation.play {
            for _ in 0..self.animation.speed {
                self.update_frame();
            }
        }

        if let Some(data) = &self.animation.data {
            ui.heading("Animation");

            ui.horizontal(|ui| {
                if ui.button("Step").clicked() {
                    self.animation.step = true;
                }

                if self.animation.play {
                    if ui.button("Pause").clicked() {
                        self.animation.play = false;
                    }
                } else {
                    if ui.button("Play").clicked() {
                        if self.animation.frame >= self.animation.data.as_ref().unwrap().len() {
                            self.animation.frame = 0;
                        }
                        self.animation.play = true;
                    }
                }

                if ui.button("Reset").clicked() {
                    self.animation.frame = 0;
                    self.animation.play = false;
                    self.animation.step = true;
                }

                ui.label("Speed: ");
                ui.add(Slider::new(&mut self.animation.speed, 1..=20).prefix("x"));
            });

            // Paint

            let max_value = data
                .iter()
                .fold(0, |max_value, (x, y)| max_value.max(x.abs()).max(y.abs()))
                + 10;

            let painter_max_size = ui.available_width().min(ui.available_height()).min(500.0);
            let painter_size = egui::vec2(painter_max_size, painter_max_size);

            let (res, painter) =
                ui.allocate_painter(painter_size, Sense::focusable_noninteractive());

            let center = res.rect.center().to_vec2();

            let side: f32 = painter_max_size / (max_value as f32 * 2.0);

            let to_panel_pos = |(x, y): (i32, i32)| {
                (egui::vec2(x as f32 * side, y as f32 * -side) + center).to_pos2()
            };

            // for x in (-max_value..max_value).step_by(10) {
            //     for y in (-max_value..max_value).step_by(10) {
            //         let dot = (x, y);

            //         let is_zero = x == 0 && y == 0;

            //         let color = if is_zero {
            //             Color32::DARK_RED
            //         } else {
            //             Color32::LIGHT_GRAY
            //         };

            //         painter.circle_stroke(to_panel_pos(dot), 0.5, Stroke::new(0.5, color));
            //     }
            // }

            for d in &self.animation.drawn_steps {
                painter.circle_stroke(to_panel_pos(*d), 1.0, Stroke::new(1.0, Color32::WHITE));
            }

            for d in &self.animation.path_crossed {
                painter.circle_stroke(to_panel_pos(*d), 5.0, Stroke::new(5.0, Color32::DARK_GREEN));
            }

            if let Some(last_step) = self.animation.drawn_steps.last() {
                painter.circle_stroke(
                    to_panel_pos(*last_step),
                    2.0,
                    Stroke::new(2.0, Color32::RED),
                );
            }

            ctx.request_repaint_after(Duration::from_millis(25));
        }
    }
}

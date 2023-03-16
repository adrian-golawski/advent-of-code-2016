use eframe::egui;
use egui::{Color32, RichText, Sense, Slider};

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
    solution: Option<(String, String)>,
}

pub struct Animation {
    data: Option<(Vec<Vec<(i32, i32)>>, Vec<Vec<(i32, i32)>>)>,
    frame: usize,
    play: bool,
    step: bool,
    speed: u32,
}

impl Animation {
    fn new() -> Self {
        Animation {
            data: None,
            frame: 0,
            speed: 1,
            step: false,
            play: false,
        }
    }
}

impl Screen {
    pub fn default() -> Self {
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

    fn update_frame(&mut self) {}

    fn update_input(&mut self, input: &str) {
        if input != self.input.current_input {
            self.input.current_input = input.to_string();

            self.input.solution = None;
            self.animation = Animation::new();
            self.input.is_input_valid = solution::is_input_valid(input);

            if self.input.is_input_valid {
                self.input.solution = Some(solution::solve(input));
                self.animation.data = Some(solution::animation_data(input));
            }
        }
    }
}

impl View for Screen {
    fn get_summary(&self) -> String {
        r#"A quick summary of the daily task here..."#.to_string()
    }

    fn input(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        if self.input.enable_custom_input {
            self.update_input(&self.input.custom_input.clone());
        } else {
            let default_string = include_str!("input.txt");
            self.update_input(default_string);
        }

        ui.checkbox(&mut self.input.enable_custom_input, "Custom input");

        if self.input.enable_custom_input {
            ui.add(
                egui::TextEdit::multiline(&mut self.input.custom_input)
                    .hint_text("Paste your input here..."),
            );
        }

        ui.horizontal(|ui| {
            if self.input.is_input_valid {
                ui.label("Input is VALID.");
            } else {
                ui.label(RichText::new("Your input is INVALID!").color(Color32::DARK_RED));
                ui.label("(Example input: RLUDLL)");
            }
        });
    }

    fn solution(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        if let Some((part1, part2)) = &self.input.solution {
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
                        if self.animation.frame >= self.animation.data.as_ref().unwrap().0.len() {
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
            let painter_max_size = ui.available_width().min(ui.available_height()).min(500.0);

            let painter_width =
                ui.available_width() / self.animation.data.as_ref().unwrap().0.len() as f32;

            let painter_size = egui::vec2(painter_width, painter_width);

            let (res, painter) =
                ui.allocate_painter(painter_size, Sense::focusable_noninteractive());
        }
    }
}

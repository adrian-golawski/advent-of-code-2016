use eframe::egui;
use egui::{Color32, RichText, TextEdit};

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
    solution: Option<()>,
}

pub struct Animation {
    data: Option<()>,
}

impl Animation {
    fn new() -> Self {
        Animation { data: None }
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

    fn update_input(&mut self, input: &str) {
        if input != self.input.current_input {
            self.input.current_input = input.to_string();

            self.input.solution = None;
            self.animation = Animation::new();
            self.input.is_input_valid = solution::is_input_valid(input);

            if self.input.is_input_valid {
                self.input.solution = solution::solve(input);
                let data = solution::animation_data(input);

                self.animation.data = data
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

    fn solution(&mut self, _ctx: &egui::Context, _ui: &mut egui::Ui) {
        todo!()
    }
}

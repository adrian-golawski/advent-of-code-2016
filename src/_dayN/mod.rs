use eframe::egui;
use egui::{Color32, RichText, TextEdit, Ui};

use crate::View;

mod parse;
mod solution;

pub struct Screen {
    enable_custom_input: bool,
    custom_input: String,
    solution_in_progress: bool,
    solution: Option<(i32, i32)>,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            enable_custom_input: false,
            custom_input: String::from(""),
            solution: None,
            solution_in_progress: false,
        }
    }
}

impl View for Screen {
    fn ui(&mut self, ui: &mut Ui) {
        let mut input = if self.enable_custom_input {
            self.custom_input.clone()
        } else {
            include_str!("input.txt").to_string()
        };

        let is_input_valid = solution::is_input_valid(&input);

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.enable_custom_input, "Custom input");

            if self.enable_custom_input {
                ui.add(
                    egui::TextEdit::singleline(&mut self.custom_input)
                        .hint_text("Paste your input here..."),
                );
            }
        });

        ui.horizontal(|ui| {
            if self.enable_custom_input && self.custom_input.len() > 0 {
                if is_input_valid {
                    ui.label("Input is VALID.");
                } else {
                    ui.label(RichText::new("Your input is INVALID!").color(Color32::DARK_RED));
                    ui.label("(Example input: R1, L2, E10)");
                }
            }
        });

        ui.separator();

        ui.heading("Input");

        ui.horizontal(|ui| {
            ui.set_enabled(false);
            ui.set_max_width(ui.available_width());
            ui.add_sized(ui.available_size(), TextEdit::multiline(&mut input));
        });

        if is_input_valid && !self.solution_in_progress {
            if ui.button("Solve for current input").clicked() {
                self.solution_in_progress = true;
                self.solution = solution::solve(&input);
                self.solution_in_progress = false;
            }
        }

        ui.separator();

        if let Some((part1, part2)) = self.solution {
            ui.label(format!("Part 1: {}", part1));
            ui.label(format!("Part 2: {}", part2));
        }
    }
}

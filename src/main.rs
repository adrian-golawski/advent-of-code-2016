use eframe::egui;
use egui::RichText;

mod _day_n;
mod day1;
mod day2;

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            // this is the id of the `<canvas>` element we have
            // in our `index.html`
            "canvas",
            web_options,
            Box::new(|_cc| Box::new(MyApp::default())),
        )
        .await
        .expect("failed to start eframe");
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Advent of Code 2016 by Adrian Goławski",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    show_side_panel: bool,
    days: Vec<Day>,
    active_ui: usize,
}

pub trait View {
    fn get_summary(&self) -> String;
    fn input(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);
    fn solution(&mut self, ctx: &egui::Context, ui: &mut egui::Ui);
}

struct Day {
    num: u8,
    ui: Option<Box<dyn View>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            show_side_panel: true,
            active_ui: 0,
            days: vec![
                Day {
                    num: 0,
                    ui: Some(Box::new(_day_n::Screen::default())),
                },
                Day {
                    num: 1,
                    ui: Some(Box::new(day1::Screen::default())),
                },
                Day {
                    num: 2,
                    ui: Some(Box::new(day2::Screen::default())),
                },
                Day { num: 3, ui: None },
                Day { num: 4, ui: None },
                Day { num: 5, ui: None },
                Day { num: 6, ui: None },
                Day { num: 7, ui: None },
                Day { num: 8, ui: None },
                Day { num: 9, ui: None },
                Day { num: 10, ui: None },
                Day { num: 11, ui: None },
                Day { num: 12, ui: None },
                Day { num: 13, ui: None },
                Day { num: 14, ui: None },
                Day { num: 15, ui: None },
                Day { num: 16, ui: None },
                Day { num: 17, ui: None },
                Day { num: 18, ui: None },
                Day { num: 19, ui: None },
                Day { num: 20, ui: None },
                Day { num: 21, ui: None },
                Day { num: 22, ui: None },
                Day { num: 23, ui: None },
                Day { num: 24, ui: None },
                Day { num: 25, ui: None },
            ],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            egui::TopBottomPanel::top("top").show(ctx, |ui| {
                if ui.button("Show days 📅").clicked() {
                    self.show_side_panel = !self.show_side_panel;
                }
            });

            egui::SidePanel::left("days_panel")
                .resizable(true)
                .show_animated(ctx, self.show_side_panel, |ui| {
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            egui::Grid::new("Day grid")
                                .num_columns(1)
                                .striped(true)
                                .show(ui, |ui| {
                                    for (i, d) in self.days.iter().enumerate() {
                                        if let Some(_day_ui) = &d.ui {
                                            if ui
                                                .add(egui::SelectableLabel::new(
                                                    self.active_ui == i,
                                                    RichText::new(if i != 0 {
                                                        format!("Day {}", d.num)
                                                    } else {
                                                        String::from("Intro")
                                                    })
                                                    .strong(),
                                                ))
                                                .clicked()
                                            {
                                                self.active_ui = i;
                                            }
                                        } else {
                                            ui.add(egui::Label::new(
                                                RichText::new(format!("Day {}", d.num))
                                                    .strikethrough(),
                                            ));
                                        }

                                        ui.end_row();
                                    }
                                })
                        });
                });

            egui::CentralPanel::default().show(ctx, |ui| {
                let active_day = self.days.get_mut(self.active_ui).unwrap();

                match &mut active_day.ui {
                    Some(view) => {
                        if active_day.num == 0 {
                            ui.heading("Welcome to my Advent of Code 2016 solution demo.");

                            ui.add_space(10.0);
                            ui.label("Advent of Code is a yearly programming exercise where you can solve daily algorithmic challenges together with thousands of programmers. You can learn more about it here: ");
                            ui.hyperlink("https://adventofcode.com/");

                            ui.add_space(10.0);
                            ui.label("This page is my solution to 2016 edition with visualisations written in Rust and compiled to WebAssembly for easy Web availability :)");

                            ui.add_space(10.0);
                            ui.label("You can check my repositiory here: ");
                            ui.hyperlink("https://github.com/adrian-golawski/advent-of-code-2016")
                                .on_hover_text("Advent of Code 2016 by Adrian Goławski");

                            ui.separator();

                            ui.heading("👈 To see a solution, use the picker from the left");
                        } else {
                            ui.heading(format!("Day {}", active_day.num));
                            ui.hyperlink(format!(
                                "https://adventofcode.com/2016/day/{}",
                                active_day.num
                            ));
                            ui.separator();

                            ui.heading("Summary");
                            ui.label(view.get_summary());

                            ui.separator();

                            ui.heading("Input");
                            view.input(ctx, ui);

                            ui.separator();

                            view.solution(ctx, ui);
                        }
                    }
                    None => self.active_ui = 0,
                }
            });
        });
    }
}

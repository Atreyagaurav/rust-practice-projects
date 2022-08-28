use eframe::{egui, epaint::Vec2};
use std::time::{Duration, Instant};

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(Vec2::new(300.0, 100.0));
    options.always_on_top = true;
    options.resizable = false;
    eframe::run_native("Timer", options, Box::new(|_cc| Box::new(MyApp::default())));
}

struct MyApp {
    label: String,
    time: Duration,
    start: Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            label: "Timer".to_owned(),
            time: Duration::from_secs(60),
            start: Instant::now(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // To force a repaint, call egui::Context::request_repaint at
        // any time (e.g. from another thread).
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("{}", self.label));
            ui.horizontal(|ui| {
                ui.label("Label: ");
                ui.text_edit_singleline(&mut self.label);
            });
            if ui.button("Add 10 secs").clicked() {
                self.time += Duration::from_secs(10);
            }
            ui.label(format!(
                "'{}' in {} secs",
                self.label,
                self.time.as_secs() - self.start.elapsed().as_secs()
            ));
        });
    }
}

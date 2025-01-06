use eframe::egui;

use crate::dispatcher::Dispatcher;

pub struct UIApp {}

impl Default for UIApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for UIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.label("Hello");
            if ui.button("Dispatch sample").clicked() {
                Dispatcher::handle_dispatch_sample();
            }
        });
    }
}

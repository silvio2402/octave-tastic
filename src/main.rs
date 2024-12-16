#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{sync::Arc, thread};

use eframe::egui;

mod app_service;
mod network_handler;
mod network_dispatcher;
mod sound_engine;
mod protocol;

use crate::app_service::AppService;

fn main() -> eframe::Result {
    let app_service = Arc::new(AppService::new());

    let app_service_listener = Arc::clone(&app_service);
    thread::spawn(move || {
        app_service_listener.listen();
    });

    app_service.dispatch_sample();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let app_ui = UIApp {
        name: "Arthur".to_owned(),
        age: 42,
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(app_ui))
        }),
    )
}

struct UIApp {
    name: String,
    age: u32,
}

impl eframe::App for UIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

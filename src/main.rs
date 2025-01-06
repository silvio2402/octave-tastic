#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::{sync::Arc, thread};

mod app_service;
mod network_dispatcher;
mod network_handler;
mod protocol;
mod sound_engine;

use crate::app_service::AppService;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<UIApp>::default())
        }),
    )
}

struct UIApp {
    app_service: Arc<AppService>,
}

impl Default for UIApp {
    fn default() -> Self {
        let app_service = Arc::new(AppService::new());

        let app_service_listener = Arc::clone(&app_service);
        thread::spawn(move || {
            app_service_listener.listen();
        });

        Self {
            app_service: app_service,
        }
    }
}

impl eframe::App for UIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.label("Hello");
        });
    }
}

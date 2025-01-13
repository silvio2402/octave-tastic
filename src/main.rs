#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use client_handler::ClientHandler;
use std::thread;
use ui_app::UIApp;

use eframe::egui;

mod client_handler;
mod dispatcher;
mod protocol;
mod sound_player;
mod sound_scheduler;
mod ui_app;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 400.0]),
        ..Default::default()
    };

    thread::spawn(move || {
        ClientHandler::listen();
    });

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

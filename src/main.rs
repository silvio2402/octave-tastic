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
        let result = ClientHandler::listen(3000);
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to bind: {}", e.message);
                // Try with port 3001
                let result = ClientHandler::listen(3001);
                match result {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to bind again: {}", e.message);
                    }
                }
            }
        }
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

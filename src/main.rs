#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // Hide console window on Windows in release mode

// Import necessary modules and crates
use client_handler::{Listener, NetworkListener};
use std::thread;
use ui_app::UIApp;

use eframe::egui;

// Declare the modules used in this project
mod client_handler;
mod dispatcher;
mod protocol;
mod sound_player;
mod sound_scheduler;
mod ui_app;

fn main() -> eframe::Result {
    // Define options for the eframe application
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 400.0]), // Set initial window size
        ..Default::default()
    };

    // Spawn a new thread to handle network listening
    thread::spawn(move || {
        // Attempt to listen on port 3000
        let result = NetworkListener::listen(3000);
        match result {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to bind: {}", e.message); // Print error message if binding fails
                                                            // Try with port 3001 if port 3000 fails
                let result = NetworkListener::listen(3001);
                match result {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to bind again: {}", e.message); // Print error message if binding fails again
                    }
                }
            }
        }
    });

    // Run the eframe application
    eframe::run_native(
        "My egui App", // Application title
        options,       // Application options
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx); // Install image loaders for egui context

            Ok(Box::<UIApp>::default()) // Initialize the UI application
        }),
    )
}

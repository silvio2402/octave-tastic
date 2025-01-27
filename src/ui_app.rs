use crate::dispatcher::{Dispatcher, NetworkDispatcher}; // Importing necessary modules and traits
use eframe::egui; // Importing eframe's egui module for UI
use rfd::FileDialog; // Importing rfd's FileDialog for file picking
use std::path::PathBuf; // Importing PathBuf for handling file paths

// Struct representing the UI application
pub struct UIApp {
    picked_file: Option<PathBuf>, // Optional field to store the picked file path
    addresses: Vec<String>,       // Vector to store target device addresses
}

// Implementing the Default trait for UIApp
impl Default for UIApp {
    fn default() -> Self {
        Self {
            picked_file: None,     // Initially no file is picked
            addresses: Vec::new(), // Initially no addresses are added
        }
    }
}

// Implementing the eframe::App trait for UIApp
impl eframe::App for UIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // If no addresses are present, add a default address
        if self.addresses.is_empty() {
            self.addresses.push("localhost:3000".to_owned());
        }

        // Creating the central panel for the UI
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut to_remove = Vec::new(); // Vector to store indices of addresses to be removed

            // Iterating over addresses to create UI elements for each
            for (i, address) in self.addresses.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label("Enter IP of target device: "); // Label for the address input
                    ui.text_edit_singleline(address); // Text input for the address

                    // Button to play sound on the target device
                    if ui.button("Play sound").clicked() {
                        if let Some(path) = &self.picked_file {
                            NetworkDispatcher::handle_dispatch(
                                vec![address.clone()],
                                path.to_str().unwrap().to_owned(),
                            );
                        }
                    }

                    // Button to remove the address
                    if ui.button("Remove").clicked() {
                        to_remove.push(i);
                    }
                });
            }

            // Removing addresses that were marked for removal
            for i in to_remove.iter().rev() {
                self.addresses.remove(*i);
            }

            // Button to add a new address
            if ui.button("Add address").clicked() {
                self.addresses.push("localhost:3000".to_owned());
            }

            ui.add_space(25.0); // Adding space in the UI
            ui.separator(); // Adding a separator line
            ui.add_space(25.0); // Adding more space

            // Button to pick a file
            if ui.button("Pick file").clicked() {
                let file = FileDialog::new().pick_file();
                if !file.is_none() {
                    self.picked_file = file;
                }
            }

            // Displaying the picked file path
            let picked_file_label = match &self.picked_file {
                Some(path) => format!("Picked file: {:?}", path),
                None => "No file picked".to_owned(),
            };
            ui.label(picked_file_label);

            ui.add_space(25.0); // Adding space in the UI
            ui.separator(); // Adding a separator line
            ui.add_space(25.0); // Adding more space

            // Button to play sound on all target devices
            if ui.button("Play sound").clicked() {
                if let Some(path) = &self.picked_file {
                    NetworkDispatcher::handle_dispatch(
                        self.addresses.clone(),
                        path.to_str().unwrap().to_owned(),
                    );
                }
            }
        });
    }
}

use crate::dispatcher::Dispatcher;
use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;

pub struct UIApp {
    picked_file: Option<PathBuf>,
    addresses: Vec<String>,
}

impl Default for UIApp {
    fn default() -> Self {
        Self {
            picked_file: None,
            addresses: Vec::new(),
        }
    }
}

impl eframe::App for UIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.addresses.is_empty() {
            self.addresses.push("localhost:3000".to_owned());
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut to_remove = Vec::new();
            for (i, address) in self.addresses.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label("Enter IP of target device: ");
                    ui.text_edit_singleline(address);
                    if ui.button("Play sound").clicked() {
                        if let Some(path) = &self.picked_file {
                            Dispatcher::handle_dispatch_sample(
                                vec![address.clone()],
                                path.to_str().unwrap().to_owned(),
                            );
                        }
                    }
                    if ui.button("Remove").clicked() {
                        to_remove.push(i);
                    }
                });
            }
            for i in to_remove.iter().rev() {
                self.addresses.remove(*i);
            }
            if ui.button("Add address").clicked() {
                self.addresses.push("localhost:3000".to_owned());
            }
            ui.add_space(25.0);
            ui.separator();
            ui.add_space(25.0);

            if ui.button("Pick file").clicked() {
                let file = FileDialog::new().pick_file();
                if !file.is_none() {
                    self.picked_file = file;
                }
            }

            ui.label(format!("Picked file: {:?}", self.picked_file));

            ui.add_space(25.0);
            ui.separator();
            ui.add_space(25.0);

            if ui.button("Play sound").clicked() {
                if let Some(path) = &self.picked_file {
                    Dispatcher::handle_dispatch_sample(
                        self.addresses.clone(),
                        path.to_str().unwrap().to_owned(),
                    );
                }
            }
        });
    }
}

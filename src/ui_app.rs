use eframe::egui;
use std::path::PathBuf;
use egui_file_dialog::FileDialog;

use crate::dispatcher;

pub struct UIApp {
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,            
    ip_address: String,
}

impl Default for UIApp {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new(),
            picked_file: None,
            ip_address: "127.0.0.1".to_string(),
        }
    }
}

impl eframe::App for UIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui|{
                ui.label("Enter IP of target device: ");
                ui.text_edit_singleline(&mut  self.ip_address);          
            });
           if ui.button("Pick file").clicked() {
                self.file_dialog.pick_file();
            }
            ui.label(format!("Picked file: {:?}", self.picked_file));
            self.file_dialog.update(ctx);
            if let Some(path) = self.file_dialog.take_picked() {
                self.picked_file = Some(path.to_path_buf());
            }
            if ui.button("Play sound").clicked() {
                dispatcher::Dispatcher::handle_dispatch_sample();
            }
        });
    }
}

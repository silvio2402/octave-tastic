#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{sync::Arc, thread};

mod app_service;
mod frontend;
mod network_dispatcher;
mod network_handler;
mod protocol;
mod sound_engine;

use crate::app_service::AppService;

fn main() {
    yew::Renderer::<frontend::App>::new().render();

    let app_service = Arc::new(AppService::new());

    let app_service_listener = Arc::clone(&app_service);
    thread::spawn(move || {
        app_service_listener.listen();
    });
}

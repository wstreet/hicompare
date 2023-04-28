// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod tray;
mod resolve;
use tauri::{SystemTray, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .system_tray(SystemTray::new())
        .setup(|app| tray::Tray::update_system_tray(&app.app_handle()))
        .on_system_tray_event(tray::Tray::on_system_tray_event)
        .invoke_handler(tauri::generate_handler![greet]);

    builder
        .run(tauri::generate_context!())
        .expect("error building app");
}

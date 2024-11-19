// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  let app = embroidery_studio::setup_app(tauri::Builder::default());
  app.run(|_, _| {});
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, sync::RwLock};

use tauri::Manager;

use embroidery_studio::{commands, events, logger, state};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // Create the Embroidery Studio directory in the user's document directory
      // and copy the sample patterns there if it doesn't exist.
      let studio_dir = app.path().document_dir()?.join("Embroidery Studio");
      if !studio_dir.exists() {
        log::debug!("Creating Embroidery Studio document directory at {:?}", studio_dir);
        fs::create_dir(&studio_dir)?;
        log::debug!("Copying sample patterns to Embroidery Studio document directory");
        let resource_path = app.path().resource_dir()?;
        for pattern in fs::read_dir(resource_path)? {
          let pattern = pattern?.path();
          fs::copy(pattern.clone(), studio_dir.join(pattern.file_name().unwrap()))?;
        }
      }

      events::pattern::setup_event_handlers(&app.get_webview_window("main").unwrap(), app.handle());

      Ok(())
    })
    .manage(RwLock::new(state::AppState::new()))
    .plugin(logger::setup_logger().build())
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![
      commands::pattern::load_pattern,
      commands::pattern::create_pattern,
      commands::pattern::save_pattern,
      commands::pattern::close_pattern,
    ])
    .run(tauri::generate_context!())
    .expect("Error while running Embroidery Studio");
}

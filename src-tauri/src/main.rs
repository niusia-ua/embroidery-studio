// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, sync::RwLock};

use tauri::Manager;

use embroidery_studio::{
  logger::setup_logger,
  pattern::{self, events::setup_pattern_event_handlers},
  state,
};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // Create the Embroidery Studio directory in the user's document directory
      // and copy the sample patterns there if it doesn't exist.
      if let Some(document_dir) = tauri::api::path::document_dir() {
        let studio_dir = document_dir.join("Embroidery Studio");
        if !studio_dir.exists() {
          log::debug!("Creating Embroidery Studio document directory at {:?}", studio_dir);
          fs::create_dir(&studio_dir)?;
          log::debug!("Copying sample patterns to Embroidery Studio document directory");
          let resource_path = app.path_resolver().resolve_resource("resources/patterns").unwrap();
          for pattern in fs::read_dir(resource_path)? {
            let pattern = pattern?.path();
            fs::copy(pattern.clone(), studio_dir.join(pattern.file_name().unwrap()))?;
          }
        }
      }

      setup_pattern_event_handlers(app.get_window("main").unwrap(), app.handle());

      Ok(())
    })
    .manage(RwLock::new(state::AppState::new()))
    .plugin(setup_logger().build())
    .invoke_handler(tauri::generate_handler![
      pattern::load_pattern,
      pattern::create_pattern,
      pattern::save_pattern,
      pattern::close_pattern
    ])
    .run(tauri::generate_context!())
    .expect("Error while running Embroidery Studio");
}

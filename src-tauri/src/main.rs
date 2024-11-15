// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, sync::RwLock};

use tauri::Manager;

use embroidery_studio::{commands, events, logger, state, utils};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      // Create the Embroidery Studio directory in the user's document directory
      // and copy the sample patterns there if it doesn't exist.
      let app_document_dir = utils::path::app_document_dir(app.handle())?;
      if !app_document_dir.exists() {
        log::debug!("Creating an app document directory",);
        fs::create_dir(&app_document_dir)?;
        log::debug!("Copying sample patterns to the app document directory");
        let resource_path = app.path().resource_dir()?;
        for pattern in fs::read_dir(resource_path)? {
          let pattern = pattern?.path();
          fs::copy(pattern.clone(), app_document_dir.join(pattern.file_name().unwrap()))?;
        }
      }

      events::pattern::setup_event_handlers(&app.get_webview_window("main").unwrap(), app.handle());

      Ok(())
    })
    .manage(RwLock::new(state::AppState::default()))
    .plugin(logger::setup_logger().build())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .invoke_handler(tauri::generate_handler![
      commands::path::get_app_document_dir,
      commands::pattern::load_pattern,
      commands::pattern::create_pattern,
      commands::pattern::save_pattern,
      commands::pattern::close_pattern,
      commands::pattern::get_pattern_file_path,
      commands::pattern::add_palette_item,
      commands::stitches::add_stitch,
    ])
    .run(tauri::generate_context!())
    .expect("Error while running Embroidery Studio");
}

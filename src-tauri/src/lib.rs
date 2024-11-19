use std::{collections::HashMap, sync::RwLock};

use state::HistoryStateInner;
use tauri::Manager;

pub mod commands;
pub mod state;

mod core;
mod utils;

mod error;
mod logger;

pub fn setup_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
  builder
    .setup(|app| {
      let app_document_dir = utils::path::app_document_dir(app.handle())?;
      if !cfg!(test) && !app_document_dir.exists() {
        // Create the Embroidery Studio directory in the user's document directory
        // and copy the sample patterns there if it doesn't exist.
        log::debug!("Creating an app document directory",);
        std::fs::create_dir(&app_document_dir)?;
        log::debug!("Copying sample patterns to the app document directory");
        let resource_path = app.path().resource_dir()?;
        for pattern in std::fs::read_dir(resource_path)? {
          let pattern = pattern?.path();
          std::fs::copy(pattern.clone(), app_document_dir.join(pattern.file_name().unwrap()))?;
        }
      }
      Ok(())
    })
    .manage(RwLock::new(
      HashMap::<state::PatternKey, core::pattern::PatternProject>::new(),
    ))
    .manage(RwLock::new(HistoryStateInner::<R>::default()))
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
      commands::palette::add_palette_item,
      commands::stitches::add_stitch,
      commands::stitches::remove_stitch,
      commands::history::undo,
      commands::history::redo,
    ])
    .build(tauri::generate_context!())
    .expect("Failed to build Embroidery Studio")
}

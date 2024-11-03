use std::path::PathBuf;

use tauri::Manager;

pub fn app_document_dir<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) -> anyhow::Result<PathBuf> {
  let app_name = app_handle.config().product_name.clone().unwrap();
  let dir_path = if cfg!(test) {
    std::env::temp_dir()
  } else {
    let path_resolver = app_handle.path();
    path_resolver
      .document_dir()
      // We expect the home directory to always be available.
      .unwrap_or_else(|_| path_resolver.home_dir().unwrap())
  };
  Ok(dir_path.join(app_name))
}

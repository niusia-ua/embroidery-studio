use std::str::FromStr;

use log::Level;
use tauri_plugin_log::{Target, TargetKind};

pub fn setup_logger() -> tauri_plugin_log::Builder {
  let log_level = std::env::var("RUST_LOG").unwrap_or(String::from("INFO"));
  let log_level = Level::from_str(&log_level).unwrap_or(Level::Info);
  let max_file_size = match log_level {
    Level::Error | Level::Warn | Level::Info => 48 * 1024, // 48 KiB
    _ => 256 * 1024,                                       // 256 KiB
  };
  tauri_plugin_log::Builder::default()
    .clear_targets()
    .targets([
      #[cfg(debug_assertions)]
      Target::new(TargetKind::Stderr),
      #[cfg(not(debug_assertions))]
      Target::new(TargetKind::LogDir { file_name: None }),
    ])
    .level_for("embroidery_studio", log_level.to_level_filter())
    .max_file_size(max_file_size)
}

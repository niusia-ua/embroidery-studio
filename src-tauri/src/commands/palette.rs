use crate::core::pattern::PaletteItem;
use crate::state::{PatternKey, PatternsState};

#[tauri::command]
pub fn add_palette_item(pattern_key: PatternKey, palette_item: PaletteItem, patterns: tauri::State<PatternsState>) {
  let mut patterns = patterns.write().unwrap();
  let patproj = patterns.get_mut(&pattern_key).unwrap();
  patproj.pattern.palette.push(palette_item);
}

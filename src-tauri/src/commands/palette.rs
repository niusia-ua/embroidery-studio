use crate::{
  core::pattern::PaletteItem,
  state::{AppStateType, PatternKey},
};

#[tauri::command]
pub fn add_palette_item(pattern_key: PatternKey, palette_item: PaletteItem, state: tauri::State<AppStateType>) {
  let mut state = state.write().unwrap();
  let patproj = state.patterns.get_mut(&pattern_key).unwrap();
  patproj.pattern.palette.push(palette_item);
}

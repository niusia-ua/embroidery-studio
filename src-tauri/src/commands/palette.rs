use crate::core::actions::{Action, AddPaletteItemAction, RemovePaletteItemAction};
use crate::core::pattern::PaletteItem;
use crate::error::CommandResult;
use crate::state::{HistoryState, PatternKey, PatternsState};

#[tauri::command]
pub fn add_palette_item<R: tauri::Runtime>(
  pattern_key: PatternKey,
  palette_item: PaletteItem,
  window: tauri::WebviewWindow<R>,
  history: tauri::State<HistoryState<R>>,
  patterns: tauri::State<PatternsState>,
) -> CommandResult<()> {
  let mut patterns = patterns.write().unwrap();
  let patproj = patterns.get_mut(&pattern_key).unwrap();
  if !patproj.pattern.palette.contains(&palette_item) {
    let mut history = history.write().unwrap();
    let action = AddPaletteItemAction::new(palette_item);
    action.perform(&window, patproj)?;
    history.get_mut(&pattern_key).push(Box::new(action));
  }
  Ok(())
}

#[tauri::command]
pub fn remove_palette_item<R: tauri::Runtime>(
  pattern_key: PatternKey,
  palette_item: PaletteItem,
  window: tauri::WebviewWindow<R>,
  history: tauri::State<HistoryState<R>>,
  patterns: tauri::State<PatternsState>,
) -> CommandResult<()> {
  let mut patterns = patterns.write().unwrap();
  let mut history = history.write().unwrap();
  let action = RemovePaletteItemAction::new(palette_item);
  action.perform(&window, patterns.get_mut(&pattern_key).unwrap())?;
  history.get_mut(&pattern_key).push(Box::new(action));
  Ok(())
}

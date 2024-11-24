use tauri::WebviewWindow;

use crate::{
  error::CommandResult,
  state::{HistoryState, PatternKey, PatternsState},
};

#[tauri::command]
pub fn undo<R: tauri::Runtime>(
  pattern_key: PatternKey,
  window: WebviewWindow<R>,
  history: tauri::State<HistoryState<R>>,
  patterns: tauri::State<PatternsState>,
) -> CommandResult<()> {
  let mut history = history.write().unwrap();
  let mut patterns = patterns.write().unwrap();
  if let Some(action) = history.get_mut(&pattern_key).undo() {
    action.perform(&window, patterns.get_mut(&pattern_key).unwrap())?;
  }
  Ok(())
}

#[tauri::command]
pub fn redo<R: tauri::Runtime>(
  pattern_key: PatternKey,
  window: WebviewWindow<R>,
  history: tauri::State<HistoryState<R>>,
  patterns: tauri::State<PatternsState>,
) -> CommandResult<()> {
  let mut history = history.write().unwrap();
  let mut patterns = patterns.write().unwrap();
  if let Some(action) = history.get_mut(&pattern_key).redo() {
    action.perform(&window, patterns.get_mut(&pattern_key).unwrap())?;
  }
  Ok(())
}

use crate::{
  core::{
    actions::{Action, AddStitchAction, RemoveStitchAction},
    pattern::Stitch,
  },
  error::CommandResult,
  state::{HistoryState, PatternKey, PatternsState},
};

#[tauri::command]
pub fn add_stitch<R: tauri::Runtime>(
  pattern_key: PatternKey,
  stitch: Stitch,
  window: tauri::WebviewWindow<R>,
  history: tauri::State<HistoryState<R>>,
  patterns: tauri::State<PatternsState>,
) -> CommandResult<()> {
  let mut history = history.write().unwrap();
  let mut patterns = patterns.write().unwrap();
  let action = AddStitchAction::new(stitch);
  action.perform(&window, patterns.get_mut(&pattern_key).unwrap())?;
  history.get_mut(&pattern_key).push(Box::new(action));
  Ok(())
}

#[tauri::command]
pub fn remove_stitch<R: tauri::Runtime>(
  pattern_key: PatternKey,
  stitch: Stitch,
  window: tauri::WebviewWindow<R>,
  history: tauri::State<HistoryState<R>>,
  patterns: tauri::State<PatternsState>,
) -> CommandResult<()> {
  let mut history = history.write().unwrap();
  let mut patterns = patterns.write().unwrap();
  let action = RemoveStitchAction::new(stitch);
  action.perform(&window, patterns.get_mut(&pattern_key).unwrap())?;
  history.get_mut(&pattern_key).push(Box::new(action));
  Ok(())
}

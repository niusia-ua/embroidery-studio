use crate::{
  core::{
    commands::{AddStitchCommand, Command, RemoveStitchCommand},
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
  let command = AddStitchCommand::new(stitch);
  command.execute(&window, patterns.get_mut(&pattern_key).unwrap())?;
  history.get_mut(&pattern_key).push(Box::new(command));
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
  let command = RemoveStitchCommand::new(stitch);
  command.execute(&window, patterns.get_mut(&pattern_key).unwrap())?;
  history.get_mut(&pattern_key).push(Box::new(command));
  Ok(())
}

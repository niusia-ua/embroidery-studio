use tauri::WebviewWindow;

use crate::{
  error::CommandResult,
  state::{AppStateType, PatternKey},
};

#[tauri::command]
pub fn undo(window: WebviewWindow, state: tauri::State<AppStateType>, pattern_key: PatternKey) -> CommandResult<()> {
  let mut state = state.write().unwrap();
  let history = state.history.get_mut(&pattern_key).unwrap();
  if let Some(command) = history.undo() {
    command.revoke(&window, state.patterns.get_mut(&pattern_key).unwrap())?;
  }
  Ok(())
}

#[tauri::command]
pub fn redo(window: WebviewWindow, state: tauri::State<AppStateType>, pattern_key: PatternKey) -> CommandResult<()> {
  let mut state = state.write().unwrap();
  let history = state.history.get_mut(&pattern_key).unwrap();
  if let Some(command) = history.redo() {
    command.execute(&window, state.patterns.get_mut(&pattern_key).unwrap())?;
  }
  Ok(())
}

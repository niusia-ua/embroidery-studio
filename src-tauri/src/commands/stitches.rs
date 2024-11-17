use crate::{
  core::{
    commands::{AddStitchCommand, Command, RemoveStitchCommand},
    pattern::Stitch,
  },
  error::CommandResult,
  state::{AppStateType, PatternKey},
};

#[tauri::command]
pub fn add_stitch<R: tauri::Runtime>(
  pattern_key: PatternKey,
  stitch: Stitch,
  window: tauri::WebviewWindow<R>,
  state: tauri::State<AppStateType>,
) -> CommandResult<()> {
  let mut state = state.write().unwrap();
  let command = AddStitchCommand::new(stitch);
  command.execute(&window, state.patterns.get_mut(&pattern_key).unwrap())?;
  state.history.get_mut(&pattern_key).unwrap().push(Box::new(command));
  Ok(())
}

#[tauri::command]
pub fn remove_stitch<R: tauri::Runtime>(
  pattern_key: PatternKey,
  stitch: Stitch,
  window: tauri::WebviewWindow<R>,
  state: tauri::State<AppStateType>,
) -> CommandResult<()> {
  let mut state = state.write().unwrap();
  let command = RemoveStitchCommand::new(stitch);
  command.execute(&window, state.patterns.get_mut(&pattern_key).unwrap())?;
  state.history.get_mut(&pattern_key).unwrap().push(Box::new(command));
  Ok(())
}

use tauri::Manager;

use crate::{
  error::CommandResult,
  parser::{self, PatternFormat},
  pattern::{display::DisplaySettings, print::PrintSettings, Pattern, PatternProject},
  state::{AppStateType, PatternKey},
};

#[tauri::command]
pub fn load_pattern(file_path: std::path::PathBuf, state: tauri::State<AppStateType>) -> CommandResult<Vec<u8>> {
  log::trace!("Loading pattern");
  let mut state = state.write().unwrap();
  let pattern_key = PatternKey::from(file_path.clone());
  let pattern = match state.patterns.get(&pattern_key) {
    Some(pattern) => {
      log::trace!("Pattern has been already loaded");
      pattern.to_owned()
    }
    None => {
      let mut new_file_path = file_path.clone();
      new_file_path.set_extension("oxs");

      let mut pattern = match PatternFormat::try_from(file_path.extension())? {
        PatternFormat::Xsd => parser::xsd::parse_pattern(file_path)?,
        PatternFormat::Oxs => parser::oxs::parse_pattern(file_path)?,
        PatternFormat::EmbProj => todo!(),
        // PatternFormat::EmbProj => {
        //   let mut reader = std::fs::File::open(file_path)?;
        //   borsh::from_reader(&mut reader)?
        // }
      };
      pattern.file_path = new_file_path;

      state.patterns.insert(pattern_key, pattern.clone());
      pattern
    }
  };
  log::trace!("Pattern loaded");
  Ok(borsh::to_vec(&pattern)?)
}

#[tauri::command]
pub fn create_pattern<R: tauri::Runtime>(
  app_handle: tauri::AppHandle<R>,
  state: tauri::State<AppStateType>,
) -> CommandResult<(PatternKey, Vec<u8>)> {
  log::trace!("Creating new pattern");
  let mut state = state.write().unwrap();
  let file_path = app_handle
    .path()
    .document_dir()?
    .join(app_handle.config().product_name.clone().unwrap())
    .join("Untitled.oxs");
  let pattern_key = PatternKey::from(file_path.clone());
  let pattern = PatternProject {
    file_path,
    pattern: Pattern::default(),
    display_settings: DisplaySettings::new(2),
    print_settings: PrintSettings::default(),
  };
  state.patterns.insert(pattern_key.clone(), pattern.clone());
  log::trace!("Pattern has been created");
  // It is safe to unwrap here, because the pattern is always serializable.
  Ok((pattern_key, borsh::to_vec(&pattern).unwrap()))
}

#[tauri::command]
pub fn save_pattern(
  pattern_key: PatternKey,
  file_path: std::path::PathBuf,
  state: tauri::State<AppStateType>,
) -> CommandResult<()> {
  log::trace!("Saving pattern");
  let mut state = state.write().unwrap();
  let patproj = state.patterns.get_mut(&pattern_key).unwrap();
  patproj.file_path = file_path;
  match PatternFormat::try_from(patproj.file_path.extension())? {
    PatternFormat::Xsd => Err(anyhow::anyhow!("The XSD format is not supported for saving.")),
    PatternFormat::Oxs => parser::oxs::save_pattern(patproj),
    PatternFormat::EmbProj => todo!(),
    // PatternFormat::EmbProj => {
    //   let mut reader = std::fs::File::open(file_path)?;
    //   borsh::from_reader(&mut reader)?
    // }
  }?;
  log::trace!("Pattern saved");
  Ok(())
}

#[tauri::command]
pub fn close_pattern(pattern_key: PatternKey, state: tauri::State<AppStateType>) {
  log::trace!("Closing pattern {:?}", pattern_key);
  state.write().unwrap().patterns.remove(&pattern_key);
  log::trace!("Pattern closed");
}

#[tauri::command]
pub fn get_pattern_file_path(pattern_key: PatternKey, state: tauri::State<AppStateType>) -> String {
  let state = state.read().unwrap();
  let patproj = state.patterns.get(&pattern_key).unwrap();
  patproj.file_path.to_string_lossy().to_string()
}

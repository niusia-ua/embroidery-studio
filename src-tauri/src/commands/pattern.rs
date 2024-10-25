use crate::{
  error::CommandResult,
  parser::{self, PatternFormat},
  pattern::{display::DisplaySettings, print::PrintSettings, Pattern, PatternProject},
  state::{AppStateType, PatternKey},
};

#[tauri::command]
pub fn load_pattern(file_path: std::path::PathBuf, state: tauri::State<AppStateType>) -> CommandResult<Vec<u8>> {
  log::trace!("Loading pattern from {:?}", file_path);
  let mut state = state.write().unwrap();
  let pattern_key = PatternKey::from(file_path.clone());
  let pattern = match state.patterns.get(&pattern_key) {
    Some(pattern) => {
      log::trace!("Pattern has been already loaded");
      pattern.to_owned()
    }
    None => {
      let pattern_format = PatternFormat::try_from(file_path.extension())?;
      let pattern = match pattern_format {
        PatternFormat::Xsd => parser::xsd::parse_pattern(file_path)?,
        PatternFormat::Oxs => parser::oxs::parse_pattern(file_path)?,
        // PatternFormat::EmbProj => {
        //   let mut reader = std::fs::File::open(file_path)?;
        //   borsh::from_reader(&mut reader)?
        // }
        PatternFormat::EmbProj => todo!(),
      };
      state.patterns.insert(pattern_key, pattern.clone());
      pattern
    }
  };
  log::trace!("Pattern loaded");
  Ok(borsh::to_vec(&pattern)?)
}

#[tauri::command]
pub fn create_pattern(state: tauri::State<AppStateType>) -> (PatternKey, Vec<u8>) {
  log::trace!("Creating new pattern");
  let mut state = state.write().unwrap();
  let file_path = std::path::PathBuf::from(format!("Untitled-{:?}.json", std::time::Instant::now()));
  let pattern_key = PatternKey::from(file_path.clone());
  let pattern = PatternProject {
    file_path: Some(file_path),
    pattern: Pattern::default(),
    display_settings: DisplaySettings::new(2),
    print_settings: PrintSettings::default(),
  };
  state.patterns.insert(pattern_key.clone(), pattern.clone());
  log::trace!("Pattern has been created");
  // It is safe to unwrap here, because the pattern is always serializable.
  (pattern_key, borsh::to_vec(&pattern).unwrap())
}

// TODO: Use a custom or different pattern format, but not the JSON.
#[tauri::command]
pub fn save_pattern(
  pattern_key: PatternKey,
  file_path: std::path::PathBuf,
  state: tauri::State<AppStateType>,
) -> CommandResult<()> {
  log::trace!("Saving pattern to {:?}", file_path);
  let state = state.read().unwrap();
  let pattern = state.patterns.get(&pattern_key).unwrap();
  std::fs::write(file_path, borsh::to_vec(pattern)?)?;
  log::trace!("Pattern saved");
  Ok(())
}

#[tauri::command]
pub fn close_pattern(pattern_key: PatternKey, state: tauri::State<AppStateType>) {
  log::trace!("Closing pattern {:?}", pattern_key);
  state.write().unwrap().patterns.remove(&pattern_key);
  log::trace!("Pattern closed");
}

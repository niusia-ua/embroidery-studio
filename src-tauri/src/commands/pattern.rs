use crate::{
  error::Result,
  parser::{self, PatternFormat},
  pattern::Pattern,
  state::{AppStateType, PatternKey},
};

#[tauri::command]
pub fn load_pattern(file_path: std::path::PathBuf, state: tauri::State<AppStateType>) -> Result<Vec<u8>> {
  log::trace!("Loading pattern from {:?}", file_path);
  let mut state = state.write().unwrap();
  let pattern_key = PatternKey::from(file_path.clone());
  let pattern = match state.patterns.get(&pattern_key) {
    Some(pattern) => {
      log::trace!("Pattern already loaded");
      pattern.to_owned()
    }
    None => {
      let pattern_format = PatternFormat::try_from(file_path.extension())?;
      let pattern = match pattern_format {
        PatternFormat::Xsd => parser::xsd::parse_pattern(file_path)?,
        PatternFormat::Oxs => parser::oxs::parse_pattern(file_path)?,
        PatternFormat::Json => {
          let content = std::fs::read_to_string(file_path)?;
          serde_json::from_str(&content).unwrap()
        }
      };
      state.patterns.insert(pattern_key, pattern.clone());
      pattern
    }
  };
  log::trace!("Pattern loaded");
  Ok(borsh::to_vec(&pattern).unwrap())
}

#[tauri::command]
pub fn create_pattern(state: tauri::State<AppStateType>) -> (PatternKey, Vec<u8>) {
  log::trace!("Creating new pattern");
  let mut state = state.write().unwrap();
  let file_path = std::path::PathBuf::from(format!("Untitled-{:?}.json", std::time::Instant::now()));
  let pattern_key = PatternKey::from(file_path);
  let pattern = Pattern::default();
  state.patterns.insert(pattern_key.clone(), pattern.clone());
  log::trace!("Pattern created");
  (pattern_key, borsh::to_vec(&pattern).unwrap())
}

// TODO: Use a custom or different pattern format, but not the JSON.
#[tauri::command]
pub fn save_pattern(
  pattern_key: PatternKey,
  file_path: std::path::PathBuf,
  state: tauri::State<AppStateType>,
) -> Result<()> {
  log::trace!("Saving pattern to {:?}", file_path);
  let state = state.read().unwrap();
  let pattern = state.patterns.get(&pattern_key).unwrap();
  std::fs::write(file_path, serde_json::to_string(pattern).unwrap())?;
  log::trace!("Pattern saved");
  Ok(())
}

#[tauri::command]
pub fn close_pattern(pattern_key: PatternKey, state: tauri::State<AppStateType>) {
  log::trace!("Closing pattern {:?}", pattern_key);
  state.write().unwrap().patterns.remove(&pattern_key);
  log::trace!("Pattern closed");
}

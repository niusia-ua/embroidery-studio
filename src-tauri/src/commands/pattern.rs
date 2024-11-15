use crate::{
  error::CommandResult,
  parser::{self, PatternFormat},
  pattern::{display::DisplaySettings, print::PrintSettings, PaletteItem, Pattern, PatternProject},
  state::{AppStateType, PatternKey},
  utils::path::app_document_dir,
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
      new_file_path.set_extension(PatternFormat::default().to_string());

      let mut pattern = match PatternFormat::try_from(file_path.extension())? {
        PatternFormat::Xsd => parser::xsd::parse_pattern(file_path)?,
        PatternFormat::Oxs => parser::oxs::parse_pattern(file_path)?,
        PatternFormat::EmbProj => parser::embproj::parse_pattern(file_path)?,
      };
      pattern.file_path = new_file_path;

      pattern
    }
  };
  let result = borsh::to_vec(&pattern)?;

  state.patterns.insert(pattern_key, pattern.clone());
  log::trace!("Pattern loaded");

  Ok(result)
}

#[tauri::command]
pub fn create_pattern<R: tauri::Runtime>(
  app_handle: tauri::AppHandle<R>,
  state: tauri::State<AppStateType>,
) -> CommandResult<(PatternKey, Vec<u8>)> {
  log::trace!("Creating new pattern");
  let mut state = state.write().unwrap();

  let pattern = Pattern::default();
  let patproj = PatternProject {
    file_path: app_document_dir(&app_handle)?.join(format!("{}.{}", pattern.info.title, PatternFormat::default())),
    pattern,
    display_settings: DisplaySettings::new(2),
    print_settings: PrintSettings::default(),
  };

  let pattern_key = PatternKey::from(patproj.file_path.clone());
  // It is safe to unwrap here, because the pattern is always serializable.
  let result = (pattern_key.clone(), borsh::to_vec(&patproj).unwrap());

  state.patterns.insert(pattern_key, patproj);
  log::trace!("Pattern has been created");

  Ok(result)
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
    PatternFormat::EmbProj => parser::embproj::save_pattern(patproj),
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

#[tauri::command]
pub fn add_palette_item(pattern_key: PatternKey, palette_item: PaletteItem, state: tauri::State<AppStateType>) {
  let mut state = state.write().unwrap();
  let patproj = state.patterns.get_mut(&pattern_key).unwrap();
  patproj.pattern.palette.push(palette_item);
}

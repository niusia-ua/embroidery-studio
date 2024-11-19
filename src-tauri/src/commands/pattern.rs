use crate::{
  core::{
    parser::{self, PatternFormat},
    pattern::{display::DisplaySettings, print::PrintSettings, Pattern, PatternProject},
  },
  error::CommandResult,
  state::{PatternKey, PatternsState},
  utils::path::app_document_dir,
};

#[tauri::command]
pub fn load_pattern(file_path: std::path::PathBuf, patterns: tauri::State<PatternsState>) -> CommandResult<Vec<u8>> {
  log::trace!("Loading pattern");
  let mut patterns = patterns.write().unwrap();
  let pattern_key = PatternKey::from(file_path.clone());
  let result = match patterns.get(&pattern_key) {
    Some(pattern) => borsh::to_vec(pattern)?,
    None => {
      let mut new_file_path = file_path.clone();
      new_file_path.set_extension(PatternFormat::default().to_string());

      let mut patproj = match PatternFormat::try_from(file_path.extension())? {
        PatternFormat::Xsd => parser::xsd::parse_pattern(file_path)?,
        PatternFormat::Oxs => parser::oxs::parse_pattern(file_path)?,
        PatternFormat::EmbProj => parser::embproj::parse_pattern(file_path)?,
      };
      patproj.file_path = new_file_path;

      let result = borsh::to_vec(&patproj)?;
      patterns.insert(pattern_key, patproj);
      result
    }
  };
  log::trace!("Pattern loaded");
  Ok(result)
}

#[tauri::command]
pub fn create_pattern<R: tauri::Runtime>(
  app_handle: tauri::AppHandle<R>,
  patterns: tauri::State<PatternsState>,
) -> CommandResult<(PatternKey, Vec<u8>)> {
  log::trace!("Creating new pattern");
  let mut patterns = patterns.write().unwrap();

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

  patterns.insert(pattern_key, patproj);
  log::trace!("Pattern has been created");

  Ok(result)
}

#[tauri::command]
pub fn save_pattern(
  pattern_key: PatternKey,
  file_path: std::path::PathBuf,
  patterns: tauri::State<PatternsState>,
) -> CommandResult<()> {
  log::trace!("Saving pattern");
  let mut patterns = patterns.write().unwrap();
  let patproj = patterns.get_mut(&pattern_key).unwrap();
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
pub fn close_pattern(pattern_key: PatternKey, patterns: tauri::State<PatternsState>) {
  log::trace!("Closing pattern {:?}", pattern_key);
  patterns.write().unwrap().remove(&pattern_key);
  log::trace!("Pattern closed");
}

#[tauri::command]
pub fn get_pattern_file_path(pattern_key: PatternKey, patterns: tauri::State<PatternsState>) -> String {
  let patterns = patterns.read().unwrap();
  let patproj = patterns.get(&pattern_key).unwrap();
  patproj.file_path.to_string_lossy().to_string()
}

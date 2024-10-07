use tauri::Manager;

use embroidery_studio::{
  commands,
  state::{AppStateType, PatternKey},
};

mod utils;

#[test]
fn parses_supported_pattern_formats() {
  let app = utils::setup_app();
  let handle = app.handle();
  let state = handle.state::<AppStateType>();

  let resources = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/patterns");
  let paths = std::fs::read_dir(resources).unwrap();
  for path in paths {
    let path = path.unwrap().path();
    assert!(commands::pattern::load_pattern(path.clone(), state.clone()).is_ok());
    assert!(state.read().unwrap().patterns.contains_key(&PatternKey::from(path)));
  }
}

#[test]
fn creates_new_pattern() {
  let app = utils::setup_app();
  let handle = app.handle();
  let state = handle.state::<AppStateType>();

  let (pattern_key, _) = commands::pattern::create_pattern(state.clone());
  assert!(state.read().unwrap().patterns.contains_key(&pattern_key));
}

#[test]
fn saves_pattern() {
  let app = utils::setup_app();
  let handle = app.handle();
  let state = handle.state::<AppStateType>();

  let temp_dir = std::env::temp_dir();
  let file_path = temp_dir.join("pattern.json");

  let (pattern_key, _) = commands::pattern::create_pattern(state.clone());
  assert!(commands::pattern::save_pattern(pattern_key, file_path, state.clone()).is_ok());
}

#[test]
fn closes_pattern() {
  let app = utils::setup_app();
  let handle = app.handle();
  let state = handle.state::<AppStateType>();

  let (pattern_key, _) = commands::pattern::create_pattern(state.clone());
  commands::pattern::close_pattern(pattern_key.clone(), state.clone());
  assert!(state.read().unwrap().patterns.get(&pattern_key).is_none());
}

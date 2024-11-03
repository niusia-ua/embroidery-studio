use tauri::Manager;

use embroidery_studio::{
  commands,
  state::{AppStateType, PatternKey},
};

mod utils;

fn get_all_test_patterns() -> Vec<std::io::Result<std::fs::DirEntry>> {
  let sample_patterns = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/patterns");
  let test_patterns = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("testdata/patterns");
  std::fs::read_dir(sample_patterns)
    .unwrap()
    .chain(std::fs::read_dir(test_patterns).unwrap())
    .collect()
}

#[test]
fn parses_supported_pattern_formats() {
  let app = utils::setup_app();
  let app_handle = app.handle();
  let state = app_handle.state::<AppStateType>();

  for file_path in get_all_test_patterns().into_iter() {
    let file_path = file_path.unwrap().path();
    assert!(commands::pattern::load_pattern(file_path.clone(), state.clone()).is_ok());
    assert!(state
      .read()
      .unwrap()
      .patterns
      .contains_key(&PatternKey::from(file_path)));
  }
}

#[test]
fn creates_new_pattern() {
  let app = utils::setup_app();
  let app_handle = app.handle();
  let state = app_handle.state::<AppStateType>();

  let (pattern_key, _) = commands::pattern::create_pattern(app_handle.clone(), state.clone()).unwrap();
  assert!(state.read().unwrap().patterns.contains_key(&pattern_key));
}

#[test]
fn saves_pattern() {
  let app = utils::setup_app();
  let app_handle = app.handle();
  let state = app_handle.state::<AppStateType>();

  for file_path in get_all_test_patterns().into_iter() {
    let file_path = file_path.unwrap().path();
    commands::pattern::load_pattern(file_path.clone(), state.clone()).unwrap();
    let pattern_key = PatternKey::from(file_path);

    for extension in ["oxs", "embproj"] {
      let file_path = std::env::temp_dir().join(format!("pattern.{}", extension));
      // If we can save the pattern and then parse it back, we can consider it a success.
      assert!(commands::pattern::save_pattern(pattern_key.clone(), file_path.clone(), state.clone()).is_ok());
      assert!(commands::pattern::load_pattern(file_path.clone(), state.clone()).is_ok());
    }
  }
}

#[test]
fn closes_pattern() {
  let app = utils::setup_app();
  let app_handle = app.handle();
  let state = app_handle.state::<AppStateType>();

  let (pattern_key, _) = commands::pattern::create_pattern(app_handle.clone(), state.clone()).unwrap();
  commands::pattern::close_pattern(pattern_key.clone(), state.clone());
  assert!(state.read().unwrap().patterns.get(&pattern_key).is_none());
}

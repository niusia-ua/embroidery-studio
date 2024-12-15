use tauri::{
  test::{mock_builder, MockRuntime},
  Manager,
};

use embroidery_studio::{
  commands, setup_app,
  state::{PatternKey, PatternsState},
};

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
  let app = setup_app::<MockRuntime>(mock_builder());
  let app_handle = app.handle();
  let patterns_state = app_handle.state::<PatternsState>();

  for file_path in get_all_test_patterns().into_iter() {
    let file_path = file_path.unwrap().path();
    assert!(commands::pattern::load_pattern(file_path.clone(), patterns_state.clone()).is_ok());
    assert!(patterns_state
      .read()
      .unwrap()
      .contains_key(&PatternKey::from(&file_path)));
  }
}

#[test]
fn creates_new_pattern() {
  let app = setup_app::<MockRuntime>(mock_builder());
  let app_handle = app.handle();
  let patterns_state = app_handle.state::<PatternsState>();

  assert!(patterns_state.read().unwrap().is_empty());
  commands::pattern::create_pattern(app_handle.clone(), patterns_state.clone()).unwrap();
  assert_eq!(patterns_state.read().unwrap().len(), 1);
}

#[test]
fn saves_pattern() {
  let app = setup_app::<MockRuntime>(mock_builder());
  let app_handle = app.handle();
  let patterns_state = app_handle.state::<PatternsState>();

  for file_path in get_all_test_patterns().into_iter() {
    let file_path = file_path.unwrap().path();
    commands::pattern::load_pattern(file_path.clone(), patterns_state.clone()).unwrap();
    let pattern_key = PatternKey::from(&file_path);

    for extension in ["oxs", "embproj"] {
      let file_path = std::env::temp_dir().join(format!("pattern.{}", extension));
      // If we can save the pattern and then parse it back, we can consider it a success.
      assert!(commands::pattern::save_pattern(pattern_key.clone(), file_path.clone(), patterns_state.clone()).is_ok());
      assert!(commands::pattern::load_pattern(file_path.clone(), patterns_state.clone()).is_ok());
    }
  }
}

#[test]
fn closes_pattern() {
  let app = setup_app::<MockRuntime>(mock_builder());
  let app_handle = app.handle();
  let patterns_state = app_handle.state::<PatternsState>();

  assert!(patterns_state.read().unwrap().is_empty());
  commands::pattern::create_pattern(app_handle.clone(), patterns_state.clone()).unwrap();
  assert_eq!(patterns_state.read().unwrap().len(), 1);

  let pattern_key = patterns_state
    .read()
    .unwrap()
    .keys()
    .cloned()
    .collect::<Vec<PatternKey>>()
    .first()
    .unwrap()
    .to_owned();
  commands::pattern::close_pattern(pattern_key, patterns_state.clone());
  assert!(patterns_state.read().unwrap().is_empty());
}

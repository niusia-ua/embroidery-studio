use tauri::{
  generate_context,
  test::{mock_builder, MockRuntime},
  App,
};

use embroidery_studio::state::AppState;

pub fn setup_app() -> App<MockRuntime> {
  mock_builder()
    .manage(std::sync::RwLock::new(AppState::new()))
    .build(generate_context!())
    .unwrap()
}

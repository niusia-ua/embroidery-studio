use crate::pattern::Pattern;

pub struct AppState {
  pub pattern: Option<Pattern>,
}

impl AppState {
  pub fn new() -> Self {
    Self { pattern: None }
  }
}

pub type AppStateType = std::sync::RwLock<AppState>;

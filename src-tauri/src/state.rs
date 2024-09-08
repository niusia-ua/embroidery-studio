use std::collections::HashMap;

use crate::pattern::{Pattern, PatternKey};

pub struct AppState {
  pub patterns: HashMap<PatternKey, Pattern>,
}

impl AppState {
  pub fn new() -> Self {
    Self {
      patterns: HashMap::new(),
    }
  }
}

pub type AppStateType = std::sync::RwLock<AppState>;

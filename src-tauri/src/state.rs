use std::{collections::HashMap, path::PathBuf};

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use crate::pattern::Pattern;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[repr(transparent)]
pub struct PatternKey(String);

impl From<PathBuf> for PatternKey {
  fn from(value: PathBuf) -> Self {
    Self(value.to_string_lossy().to_string())
  }
}

pub struct AppState {
  pub patterns: HashMap<PatternKey, Pattern>,
}

impl AppState {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    Self { patterns: HashMap::new() }
  }
}

pub type AppStateType = std::sync::RwLock<AppState>;

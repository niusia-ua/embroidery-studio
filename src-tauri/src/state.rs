use std::{collections::HashMap, path::PathBuf};

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use crate::core::{commands::Command, pattern::PatternProject};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[repr(transparent)]
pub struct PatternKey(String);

impl From<PathBuf> for PatternKey {
  fn from(value: PathBuf) -> Self {
    Self(value.to_string_lossy().to_string())
  }
}

#[derive(Default)]
pub struct AppState {
  pub patterns: HashMap<PatternKey, PatternProject>,
  pub history: HashMap<PatternKey, Vec<Box<dyn Command>>>,
}

pub type AppStateType = std::sync::RwLock<AppState>;

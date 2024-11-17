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

/// A history of commands.
#[derive(Default)]
pub struct History {
  undo_stack: Vec<Box<dyn Command>>,
  redo_stack: Vec<Box<dyn Command>>,
}

impl History {
  /// Add a command to the history.
  /// This pushes the command to the undo stack and clears the redo stack.
  pub fn push(&mut self, command: Box<dyn Command>) {
    self.undo_stack.push(command);
    self.redo_stack.clear();
  }

  /// Get the last command from the undo stack.
  /// This pops the command from the undo stack and pushes it to the redo stack, then returns it.
  pub fn undo(&mut self) -> Option<Box<dyn Command>> {
    self.undo_stack.pop().inspect(|command| {
      self.redo_stack.push(command.clone());
    })
  }

  /// Get the last command from the redo stack.
  /// This pops the command from the redo stack and pushes it to the undo stack, then returns it.
  pub fn redo(&mut self) -> Option<Box<dyn Command>> {
    self.redo_stack.pop().inspect(|command| {
      self.undo_stack.push(command.clone());
    })
  }
}

#[derive(Default)]
pub struct AppState {
  pub patterns: HashMap<PatternKey, PatternProject>,
  pub history: HashMap<PatternKey, History>,
}

impl AppState {
  /// Insert a pattern into the state.
  /// This also initializes the history for the pattern.
  pub fn insert_pattern(&mut self, key: PatternKey, patproj: PatternProject) {
    self.patterns.insert(key.clone(), patproj);
    self.history.insert(key, History::default());
  }

  /// Remove a pattern from the state.
  /// This also removes the history for the pattern.
  pub fn remove_pattern(&mut self, key: &PatternKey) {
    self.patterns.remove(key);
    self.history.remove(key);
  }
}

pub type AppStateType = std::sync::RwLock<AppState>;

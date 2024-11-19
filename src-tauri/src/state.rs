use std::{collections::HashMap, path::PathBuf};

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use crate::core::{actions::Action, pattern::PatternProject};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[repr(transparent)]
pub struct PatternKey(String);

impl From<PathBuf> for PatternKey {
  fn from(value: PathBuf) -> Self {
    Self(value.to_string_lossy().to_string())
  }
}

/// A history of actions.
pub struct History<R: tauri::Runtime> {
  undo_stack: Vec<Box<dyn Action<R>>>,
  redo_stack: Vec<Box<dyn Action<R>>>,
}

impl<R: tauri::Runtime> History<R> {
  /// Add an action object to the history.
  /// This pushes the action object to the undo stack and clears the redo stack.
  pub fn push(&mut self, action: Box<dyn Action<R>>) {
    self.undo_stack.push(action);
    self.redo_stack.clear();
  }

  /// Get the last action object from the undo stack.
  /// This pops the action object from the undo stack and pushes it to the redo stack, then returns it.
  pub fn undo(&mut self) -> Option<Box<dyn Action<R>>> {
    self.undo_stack.pop().inspect(|action| {
      self.redo_stack.push(action.clone());
    })
  }

  /// Get the last action object from the redo stack.
  /// This pops the action object from the redo stack and pushes it to the undo stack, then returns it.
  pub fn redo(&mut self) -> Option<Box<dyn Action<R>>> {
    self.redo_stack.pop().inspect(|action| {
      self.undo_stack.push(action.clone());
    })
  }
}

impl<R: tauri::Runtime> Default for History<R> {
  fn default() -> Self {
    Self {
      undo_stack: Vec::new(),
      redo_stack: Vec::new(),
    }
  }
}

pub struct HistoryStateInner<R: tauri::Runtime> {
  inner: HashMap<PatternKey, History<R>>,
}

impl<R: tauri::Runtime> HistoryStateInner<R> {
  pub fn get(&self, key: &PatternKey) -> Option<&History<R>> {
    self.inner.get(key)
  }

  pub fn get_mut(&mut self, key: &PatternKey) -> &mut History<R> {
    self.inner.entry(key.clone()).or_insert_with(History::default)
  }
}

impl<R: tauri::Runtime> Default for HistoryStateInner<R> {
  fn default() -> Self {
    Self { inner: HashMap::new() }
  }
}

pub type PatternsState = std::sync::RwLock<HashMap<PatternKey, PatternProject>>;
pub type HistoryState<R> = std::sync::RwLock<HistoryStateInner<R>>;

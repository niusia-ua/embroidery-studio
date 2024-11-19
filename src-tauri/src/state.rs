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
pub struct History<R: tauri::Runtime> {
  undo_stack: Vec<Box<dyn Command<R>>>,
  redo_stack: Vec<Box<dyn Command<R>>>,
}

impl<R: tauri::Runtime> History<R> {
  /// Add a command to the history.
  /// This pushes the command to the undo stack and clears the redo stack.
  pub fn push(&mut self, command: Box<dyn Command<R>>) {
    self.undo_stack.push(command);
    self.redo_stack.clear();
  }

  /// Get the last command from the undo stack.
  /// This pops the command from the undo stack and pushes it to the redo stack, then returns it.
  pub fn undo(&mut self) -> Option<Box<dyn Command<R>>> {
    self.undo_stack.pop().inspect(|command| {
      self.redo_stack.push(command.clone());
    })
  }

  /// Get the last command from the redo stack.
  /// This pops the command from the redo stack and pushes it to the undo stack, then returns it.
  pub fn redo(&mut self) -> Option<Box<dyn Command<R>>> {
    self.redo_stack.pop().inspect(|command| {
      self.undo_stack.push(command.clone());
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

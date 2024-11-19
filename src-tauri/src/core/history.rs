//! This module contains the definition of a history of actions.
//! The history is stored per pattern project.

use super::actions::Action;

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

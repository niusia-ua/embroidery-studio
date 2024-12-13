//! This module contains the definition of actions that can be performed on a pattern project.
//! These actions include operations like adding or removing stitches or palette items, updating pattern information, etc.
//!
//! Actually, the actions implements the `Command` pattern.
//! Hovewer we named it `Action` to avoid confusion with the `commands` from Tauri.
//!
//! Each method of the `Action` accepts a reference to the `WebviewWindow` and a mutable reference to the `PatternProject`.
//! The `WebviewWindow` is used to emit events to the frontend.
//! The reason for this is that the `Action` can affects many aspects of the `PatternProject` so it is easier to emit an event for each change.

use anyhow::Result;
use tauri::WebviewWindow;

use super::pattern::PatternProject;

mod stitches;
pub use stitches::*;

/// An action that can be executed and revoked.
pub trait Action<R: tauri::Runtime>: Send + Sync + dyn_clone::DynClone {
  /// Perform the action.
  fn perform(&self, window: &WebviewWindow<R>, patproj: &mut PatternProject) -> Result<()>;

  /// Revoke (undo) the action.
  fn revoke(&self, window: &WebviewWindow<R>, patproj: &mut PatternProject) -> Result<()>;
}

dyn_clone::clone_trait_object!(<R: tauri::Runtime> Action<R>);

#[cfg(debug_assertions)]
pub mod mock {
  use super::*;

  #[derive(Clone)]
  pub struct MockAction;

  impl<R: tauri::Runtime> Action<R> for MockAction {
    fn perform(&self, _window: &WebviewWindow<R>, _patproj: &mut PatternProject) -> Result<()> {
      Ok(())
    }

    fn revoke(&self, _window: &WebviewWindow<R>, _patproj: &mut PatternProject) -> Result<()> {
      Ok(())
    }
  }
}

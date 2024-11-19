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

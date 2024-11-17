use anyhow::Result;
use tauri::WebviewWindow;

use super::pattern::PatternProject;

mod stitches;
pub use stitches::*;

/// A command that can be executed and revoked.
pub trait Command: Send + Sync + dyn_clone::DynClone {
  /// Execute the command.
  ///
  /// The `window` parameter is the webview window that the command should use to emit events.
  /// The `patproj` parameter is the pattern project that the command should modify.
  fn execute(&self, window: &WebviewWindow, patproj: &mut PatternProject) -> Result<()>;

  /// Revoke the command.
  ///
  /// The `window` parameter is the webview window that the command should use to emit events.
  /// The `patproj` parameter is the pattern project that the command should modify.
  fn revoke(&self, window: &WebviewWindow, patproj: &mut PatternProject) -> Result<()>;
}

dyn_clone::clone_trait_object!(Command);

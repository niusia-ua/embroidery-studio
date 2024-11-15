use anyhow::Result;
use tauri::WebviewWindow;

use super::pattern::PatternProject;

mod stitches;
pub use stitches::*;

pub trait Command: Send + Sync {
  fn execute(&self, window: &WebviewWindow, patproj: &mut PatternProject) -> Result<()>;
  fn revoke(&self, window: &WebviewWindow, patproj: &mut PatternProject) -> Result<()>;
}

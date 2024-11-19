use std::sync::OnceLock;

use anyhow::Result;
use tauri::{Emitter, WebviewWindow};

use crate::core::pattern::{PatternProject, Stitch, StitchConflicts};

use super::Action;

#[cfg(test)]
#[path = "stitches.test.rs"]
mod tests;

#[derive(Clone)]
pub struct AddStitchAction {
  stitch: Stitch,
  conflicts: OnceLock<StitchConflicts>,
}

impl AddStitchAction {
  pub fn new(stitch: Stitch) -> Self {
    Self {
      stitch,
      conflicts: OnceLock::new(),
    }
  }
}

impl<R: tauri::Runtime> Action<R> for AddStitchAction {
  fn perform(&self, window: &WebviewWindow<R>, patproj: &mut PatternProject) -> Result<()> {
    let conflicts = patproj.pattern.add_stitch(self.stitch);
    if self.conflicts.get().is_none() {
      self.conflicts.set(conflicts.clone()).unwrap();
    }
    window.emit("stitches:add_one", self.stitch)?;
    window.emit("stitches:remove_many", conflicts)?;
    Ok(())
  }

  fn revoke(&self, window: &WebviewWindow<R>, patproj: &mut PatternProject) -> Result<()> {
    let conflicts = self.conflicts.get().unwrap();
    patproj.pattern.remove_stitch(self.stitch);
    for stitch in conflicts.chain() {
      patproj.pattern.add_stitch(stitch);
    }
    window.emit("stitches:remove_one", self.stitch)?;
    window.emit("stitches:add_many", conflicts)?;
    Ok(())
  }
}

#[derive(Clone)]
pub struct RemoveStitchAction {
  stitch: Stitch,
}

impl RemoveStitchAction {
  pub fn new(stitch: Stitch) -> Self {
    Self { stitch }
  }
}

impl<R: tauri::Runtime> Action<R> for RemoveStitchAction {
  fn perform(&self, window: &WebviewWindow<R>, patproj: &mut PatternProject) -> Result<()> {
    patproj.pattern.remove_stitch(self.stitch);
    window.emit("stitches:remove_one", self.stitch)?;
    Ok(())
  }

  fn revoke(&self, window: &WebviewWindow<R>, patproj: &mut PatternProject) -> Result<()> {
    patproj.pattern.add_stitch(self.stitch);
    window.emit("stitches:add_one", self.stitch)?;
    Ok(())
  }
}

use std::sync::OnceLock;

use anyhow::Result;
use tauri::{Emitter, WebviewWindow};

use crate::core::pattern::{PatternProject, Stitch, StitchConflicts};

use super::Command;

#[derive(Clone)]
pub struct AddStitchCommand {
  stitch: Stitch,
  conflicts: OnceLock<StitchConflicts>,
}

impl AddStitchCommand {
  pub fn new(stitch: Stitch) -> Self {
    Self {
      stitch,
      conflicts: OnceLock::new(),
    }
  }
}

impl Command for AddStitchCommand {
  fn execute(&self, window: &WebviewWindow, patproj: &mut PatternProject) -> Result<()> {
    let conflicts = patproj.pattern.add_stitch(self.stitch);
    if self.conflicts.get().is_none() {
      self.conflicts.set(conflicts.clone()).unwrap();
    }
    window.emit("stitches:add_one", self.stitch)?;
    window.emit("stitches:remove_many", conflicts)?;
    Ok(())
  }

  fn revoke(&self, window: &WebviewWindow, patproj: &mut PatternProject) -> Result<()> {
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

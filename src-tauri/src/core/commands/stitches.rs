use std::sync::OnceLock;

use anyhow::Result;
use tauri::{Emitter, WebviewWindow};

use crate::core::pattern::{PatternProject, Stitch, StitchConflicts};

use super::Command;

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
    let conflicts = patproj.pattern.add_stitch(self.stitch.clone());
    self.conflicts.set(conflicts.clone()).unwrap();
    window.emit("stitches:remove_many", conflicts)?;
    Ok(())
  }

  fn revoke(&self, window: &WebviewWindow, patproj: &mut PatternProject) -> Result<()> {
    let conflicts = self.conflicts.get().unwrap();

    debug_assert!(patproj.pattern.remove_stitch(self.stitch.clone()));

    for stitch in conflicts.fullstitches.iter() {
      debug_assert!(patproj.pattern.add_stitch(Stitch::Full(stitch.clone())).is_empty());
    }

    for stitch in conflicts.partstitches.iter() {
      debug_assert!(patproj.pattern.add_stitch(Stitch::Part(stitch.clone())).is_empty());
    }

    if let Some(node) = &conflicts.node {
      debug_assert!(patproj.pattern.add_stitch(Stitch::Node(node.clone())).is_empty());
    }

    if let Some(line) = &conflicts.line {
      debug_assert!(patproj.pattern.add_stitch(Stitch::Line(line.clone())).is_empty());
    }

    window.emit("stitches:add_many", conflicts.clone())?;
    Ok(())
  }
}

use std::sync::OnceLock;

use anyhow::Result;
use serde::Serialize;
use tauri::{Emitter, WebviewWindow};

use super::Action;
use crate::core::pattern::display::{Formats, Symbols};
use crate::core::pattern::{PaletteItem, PatternProject, Stitch};

#[cfg(test)]
#[path = "palette.test.rs"]
mod tests;

#[derive(Clone, Serialize)]
pub struct AddPaletteItemAction {
  #[serde(rename = "paletteItem")]
  palitem: PaletteItem,
  symbols: Symbols,
  formats: Formats,
}

impl AddPaletteItemAction {
  pub fn new(palitem: PaletteItem) -> Self {
    Self {
      palitem,
      symbols: Symbols::default(),
      formats: Formats::default(),
    }
  }
}

impl<R: tauri::Runtime> Action<R> for AddPaletteItemAction {
  /// Add the palette item to the pattern.
  ///
  /// **Emits:**
  /// - `palette:add_palette_item` with the added palette item and its related types.
  fn perform(&self, window: &WebviewWindow<R>, patproj: &mut PatternProject) -> Result<()> {
    patproj.pattern.palette.push(self.palitem.clone());
    patproj.display_settings.symbols.push(self.symbols.clone());
    patproj.display_settings.formats.push(self.formats.clone());
    window.emit(
      "palette:add_palette_item",
      AddedPaletteItemData {
        palitem: self.palitem.clone(),
        palindex: patproj.pattern.palette.len() - 1,
        symbols: self.symbols.clone(),
        formats: self.formats.clone(),
      },
    )?;
    Ok(())
  }

  /// Remove the added palette item from the pattern.
  ///
  /// **Emits:**
  /// - `palette:remove_palette_item` with the palette item index.
  fn revoke(&self, window: &WebviewWindow<R>, patproj: &mut PatternProject) -> Result<()> {
    patproj.pattern.palette.pop();
    patproj.display_settings.symbols.pop();
    patproj.display_settings.formats.pop();
    window.emit("palette:remove_palette_item", patproj.pattern.palette.len())?;
    Ok(())
  }
}

#[derive(Clone)]
pub struct RemovePaletteItemAction {
  palitem: PaletteItem,
  metadata: OnceLock<RemovePaletteItemActionMetadata>,
}

#[derive(Debug, Clone)]
struct RemovePaletteItemActionMetadata {
  palindex: usize,
  symbols: Symbols,
  formats: Formats,
  conflicts: Vec<Stitch>,
}

impl RemovePaletteItemAction {
  pub fn new(palitem: PaletteItem) -> Self {
    Self {
      palitem,
      metadata: OnceLock::new(),
    }
  }
}

impl<R: tauri::Runtime> Action<R> for RemovePaletteItemAction {
  /// Remove the palette item from the pattern.
  ///
  /// **Emits:**
  /// - `palette:remove_palette_item` with the palette item index.
  /// - `stitches:remove_many` with the stitches that should be removed.
  fn perform(&self, window: &WebviewWindow<R>, patproj: &mut PatternProject) -> Result<()> {
    let palindex = patproj
      .pattern
      .palette
      .iter()
      .position(|item| item == &self.palitem)
      .unwrap();
    patproj.pattern.palette.remove(palindex);
    let symbols = patproj.display_settings.symbols.remove(palindex);
    let formats = patproj.display_settings.formats.remove(palindex);
    let conflicts = patproj.pattern.remove_stitches_by_palindex(palindex as u8);
    window.emit("palette:remove_palette_item", palindex)?;
    window.emit("stitches:remove_many", &conflicts)?;
    if self.metadata.get().is_none() {
      self
        .metadata
        .set(RemovePaletteItemActionMetadata {
          palindex,
          symbols,
          formats,
          conflicts,
        })
        .unwrap();
    }
    Ok(())
  }

  /// Add the removed palette item back to the pattern.
  ///
  /// **Emits:**
  /// - `palette:add_palette_item` with the added palette item and its related types.
  /// - `stitches:add_many` with the stitches that should be restored.
  fn revoke(&self, window: &WebviewWindow<R>, patproj: &mut PatternProject) -> Result<()> {
    let metadata = self.metadata.get().unwrap();
    patproj.pattern.palette.insert(metadata.palindex, self.palitem.clone());
    patproj
      .display_settings
      .symbols
      .insert(metadata.palindex, metadata.symbols.clone());
    patproj
      .display_settings
      .formats
      .insert(metadata.palindex, metadata.formats.clone());
    patproj
      .pattern
      .restore_stitches(metadata.conflicts.clone(), metadata.palindex as u8);
    window.emit(
      "palette:add_palette_item",
      AddedPaletteItemData {
        palitem: self.palitem.clone(),
        palindex: metadata.palindex,
        symbols: metadata.symbols.clone(),
        formats: metadata.formats.clone(),
      },
    )?;
    window.emit("stitches:add_many", &metadata.conflicts)?;
    Ok(())
  }
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(test, derive(PartialEq, serde::Deserialize))]
struct AddedPaletteItemData {
  #[serde(rename = "paletteItem")]
  palitem: PaletteItem,
  palindex: usize,
  symbols: Symbols,
  formats: Formats,
}

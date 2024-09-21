use borsh::{BorshDeserialize, BorshSerialize};

use super::stitches::*;

#[cfg(test)]
#[path = "./pattern.test.rs"]
mod tests;

pub type Coord = ordered_float::NotNan<f32>;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Pattern {
  pub properties: PatternProperties,
  pub info: PatternInfo,
  pub palette: Vec<PaletteItem>,
  pub fabric: Fabric,
  pub fullstitches: Stitches<FullStitch>,
  pub partstitches: Stitches<PartStitch>,
  pub nodes: Stitches<Node>,
  pub lines: Stitches<Line>,
}

impl Pattern {
  pub fn add_stitch(&mut self, stitch: Stitch) -> StitchConflicts {
    log::trace!("Adding stitch");
    match stitch {
      Stitch::Full(fullstitch) => {
        let conflicts = match fullstitch.kind {
          FullStitchKind::Full => StitchConflicts::default()
            .with_fullstitches(self.fullstitches.remove_conflicts_with_full_stitch(&fullstitch))
            .with_partstitches(self.partstitches.remove_conflicts_with_full_stitch(&fullstitch)),
          FullStitchKind::Petite => StitchConflicts::default()
            .with_fullstitches(self.fullstitches.remove_conflicts_with_petite_stitch(&fullstitch))
            .with_partstitches(self.partstitches.remove_conflicts_with_petite_stitch(&fullstitch)),
        };
        conflicts.with_fullstitch(self.fullstitches.insert(fullstitch))
      }
      Stitch::Part(partstitch) => {
        let conflicts = match partstitch.kind {
          PartStitchKind::Half => StitchConflicts::default()
            .with_fullstitches(self.fullstitches.remove_conflicts_with_half_stitch(&partstitch))
            .with_partstitches(self.partstitches.remove_conflicts_with_half_stitch(&partstitch)),
          PartStitchKind::Quarter => StitchConflicts::default()
            .with_fullstitches(self.fullstitches.remove_conflicts_with_quarter_stitch(&partstitch))
            .with_partstitches(self.partstitches.remove_conflicts_with_quarter_stitch(&partstitch)),
        };
        conflicts.with_partstitch(self.partstitches.insert(partstitch))
      }
      Stitch::Node(node) => StitchConflicts::default().with_node(self.nodes.insert(node)),
      Stitch::Line(line) => StitchConflicts::default().with_line(self.lines.insert(line)),
    }
  }
}

// TODO: Load the default values from a bundlled pattern file.
impl Default for Pattern {
  fn default() -> Self {
    Self {
      properties: PatternProperties { width: 100, height: 100 },
      info: PatternInfo {
        title: "Untitled".to_string(),
        author: "".to_string(),
        copyright: "".to_string(),
        description: "".to_string(),
      },
      palette: Vec::from([
        PaletteItem {
          brand: "DMC".to_string(),
          number: "310".to_string(),
          name: "Black".to_string(),
          color: "000000".to_string(),
          blends: None,
        },
        PaletteItem {
          brand: "DMC".to_string(),
          number: "349".to_string(),
          name: "Coral-DK".to_string(),
          color: "C23131".to_string(),
          blends: None,
        },
      ]),
      fabric: Fabric {
        stitches_per_inch: (14, 14),
        kind: "Aida".to_string(),
        name: "White".to_string(),
        color: "FFFFFF".to_string(),
      },
      fullstitches: Stitches::new(),
      partstitches: Stitches::new(),
      nodes: Stitches::new(),
      lines: Stitches::new(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub struct PatternProperties {
  pub width: u16,
  pub height: u16,
}

impl Default for PatternProperties {
  fn default() -> Self {
    Self { width: 100, height: 100 }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub struct PatternInfo {
  pub title: String,
  pub author: String,
  pub copyright: String,
  pub description: String,
}

impl Default for PatternInfo {
  fn default() -> Self {
    Self {
      title: String::from("Untitled"),
      author: String::new(),
      copyright: String::new(),
      description: String::new(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub struct PaletteItem {
  pub brand: String,
  pub number: String,
  pub name: String,
  pub color: String,
  pub blends: Option<Vec<Blend>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub struct Blend {
  pub brand: String,
  pub number: String,
  pub strands: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub struct Fabric {
  pub stitches_per_inch: (u16, u16),
  pub kind: String,
  pub name: String,
  pub color: String,
}

impl Default for Fabric {
  fn default() -> Self {
    Self {
      stitches_per_inch: (14, 14),
      kind: String::from("Aida"),
      name: String::from("White"),
      color: String::from("FFFFFF"),
    }
  }
}

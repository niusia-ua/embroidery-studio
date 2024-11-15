use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use super::stitches::*;

#[derive(Debug, Default, Clone, BorshSerialize, BorshDeserialize)]
pub struct Pattern {
  pub properties: PatternProperties,
  pub info: PatternInfo,
  pub palette: Vec<PaletteItem>,
  pub fabric: Fabric,
  pub fullstitches: Stitches<FullStitch>,
  pub partstitches: Stitches<PartStitch>,
  pub nodes: Stitches<Node>,
  pub lines: Stitches<Line>,
  pub specialstitches: Stitches<SpecialStitch>,
  pub special_stitch_models: Vec<SpecialStitchModel>,
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

  pub fn remove_stitch(&mut self, stitch: Stitch) -> bool {
    log::trace!("Removing stitch");
    match stitch {
      Stitch::Full(fullstitch) => self.fullstitches.remove(&fullstitch),
      Stitch::Part(partstitch) => self.partstitches.remove(&partstitch),
      Stitch::Node(node) => self.nodes.remove(&node),
      Stitch::Line(line) => self.lines.remove(&line),
    }
  }
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct PatternProperties {
  pub width: u16,
  pub height: u16,
}

impl Default for PatternProperties {
  fn default() -> Self {
    Self { width: 100, height: 100 }
  }
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct PatternInfo {
  pub title: String,
  pub author: String,
  pub company: String,
  pub copyright: String,
  pub description: String,
}

impl Default for PatternInfo {
  fn default() -> Self {
    Self {
      title: String::from("Untitled"),
      author: String::new(),
      company: String::new(),
      copyright: String::new(),
      description: String::new(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct PaletteItem {
  pub brand: String,
  pub number: String,
  pub name: String,
  pub color: String,
  pub blends: Option<Vec<Blend>>,
  pub bead: Option<Bead>,
  pub strands: Option<StitchStrands>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Blend {
  pub brand: String,
  pub number: String,
  pub strands: u8,
}

pub type Millimetres = ordered_float::NotNan<f32>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Bead {
  pub length: Millimetres,
  pub diameter: Millimetres,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct StitchStrands {
  pub full: Option<u16>,
  pub petite: Option<u16>,
  pub half: Option<u16>,
  pub quarter: Option<u16>,
  pub back: Option<u16>,
  pub straight: Option<u16>,
  pub french_knot: Option<u16>,
  pub special: Option<u16>,
}

pub type StitchesPerInch = (u16, u16);

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Fabric {
  pub spi: StitchesPerInch,
  pub kind: String,
  pub name: String,
  pub color: String,
}

impl Default for Fabric {
  fn default() -> Self {
    Self {
      spi: (14, 14),
      kind: String::from("Aida"),
      name: String::from("White"),
      color: String::from("FFFFFF"),
    }
  }
}

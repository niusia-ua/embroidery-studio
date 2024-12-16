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
  /// Check if the pattern contains a stitch.
  pub fn contains_stitch(&self, stitch: &Stitch) -> bool {
    match stitch {
      Stitch::Full(fullstitch) => self.fullstitches.contains(fullstitch),
      Stitch::Part(partstitch) => self.partstitches.contains(partstitch),
      Stitch::Node(node) => self.nodes.contains(node),
      Stitch::Line(line) => self.lines.contains(line),
    }
  }

  /// Adds a stitch to the pattern and returns any conflicts that may have arisen.
  pub fn add_stitch(&mut self, stitch: Stitch) -> StitchBundle {
    log::trace!("Adding stitch");
    match stitch {
      Stitch::Full(fullstitch) => {
        let conflicts = match fullstitch.kind {
          FullStitchKind::Full => StitchBundle::default()
            .with_fullstitches(self.fullstitches.remove_conflicts_with_full_stitch(&fullstitch))
            .with_partstitches(self.partstitches.remove_conflicts_with_full_stitch(&fullstitch)),
          FullStitchKind::Petite => StitchBundle::default()
            .with_fullstitches(self.fullstitches.remove_conflicts_with_petite_stitch(&fullstitch))
            .with_partstitches(self.partstitches.remove_conflicts_with_petite_stitch(&fullstitch)),
        };
        conflicts.with_fullstitch(self.fullstitches.insert(fullstitch))
      }
      Stitch::Part(partstitch) => {
        let conflicts = match partstitch.kind {
          PartStitchKind::Half => StitchBundle::default()
            .with_fullstitches(self.fullstitches.remove_conflicts_with_half_stitch(&partstitch))
            .with_partstitches(self.partstitches.remove_conflicts_with_half_stitch(&partstitch)),
          PartStitchKind::Quarter => StitchBundle::default()
            .with_fullstitches(self.fullstitches.remove_conflicts_with_quarter_stitch(&partstitch))
            .with_partstitches(self.partstitches.remove_conflicts_with_quarter_stitch(&partstitch)),
        };
        conflicts.with_partstitch(self.partstitches.insert(partstitch))
      }
      Stitch::Node(node) => StitchBundle::default().with_node(self.nodes.insert(node)),
      Stitch::Line(line) => StitchBundle::default().with_line(self.lines.insert(line)),
    }
  }

  /// Removes and returns a stitch from the pattern.
  pub fn remove_stitch(&mut self, stitch: Stitch) -> Option<Stitch> {
    log::trace!("Removing stitch");
    match stitch {
      Stitch::Full(fullstitch) => self.fullstitches.remove(&fullstitch).map(|fs| fs.into()),
      Stitch::Part(partstitch) => self.partstitches.remove(&partstitch).map(|ps| ps.into()),
      Stitch::Node(node) => self.nodes.remove(&node).map(|node| node.into()),
      Stitch::Line(line) => self.lines.remove(&line).map(|line| line.into()),
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
  pub strands: Option<PaletteItemStitchStrands>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Blend {
  pub brand: String,
  pub number: String,
  pub strands: BlendStrands,
}

#[nutype::nutype(
  sanitize(with = |raw| raw.clamp(1, 6)),
  derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)
)]
pub struct BlendStrands(u8);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Bead {
  pub length: f32,
  pub diameter: f32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct StitchStrandsStruct<T> {
  pub full: T,
  pub petite: T,
  pub half: T,
  pub quarter: T,
  pub back: T,
  pub straight: T,
  pub french_knot: T,
  pub special: T,
}

#[nutype::nutype(
  sanitize(with = |raw| raw.clamp(1, 12)),
  derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)
)]
pub struct StitchStrands(u8);

pub type PaletteItemStitchStrands = StitchStrandsStruct<Option<StitchStrands>>;
pub type DefaultStitchStrands = StitchStrandsStruct<StitchStrands>;

impl Default for DefaultStitchStrands {
  fn default() -> Self {
    Self {
      full: StitchStrands::new(2),
      petite: StitchStrands::new(2),
      half: StitchStrands::new(2),
      quarter: StitchStrands::new(2),
      back: StitchStrands::new(1),
      straight: StitchStrands::new(1),
      french_knot: StitchStrands::new(2),
      special: StitchStrands::new(2),
    }
  }
}

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

pub type StitchesPerInch = (u16, u16);

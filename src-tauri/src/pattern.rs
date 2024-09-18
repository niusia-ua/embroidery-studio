use std::{cmp::Ordering, collections::BTreeSet, ffi::OsStr, fs, path::PathBuf, time::Instant};

use borsh::{BorshDeserialize, BorshSerialize};
use ordered_float::NotNan;
use serde::{Deserialize, Serialize};

use crate::{error::*, state::AppStateType};

pub mod events;
mod oxs;
mod xsd;

#[cfg(test)]
#[path = "pattern.test.rs"]
mod tests;

#[tauri::command]
pub fn load_pattern(file_path: PathBuf, state: tauri::State<AppStateType>) -> Result<Vec<u8>> {
  log::trace!("Loading pattern from {:?}", file_path);
  let mut state = state.write().unwrap();
  let pattern_key = PatternKey::from(file_path.clone());
  let pattern = match state.patterns.get(&pattern_key) {
    Some(pattern) => {
      log::trace!("Pattern already loaded");
      pattern.to_owned()
    }
    None => {
      let pattern_format = PatternFormat::try_from(file_path.extension())?;
      let pattern = match pattern_format {
        PatternFormat::Xsd => xsd::parse_pattern(file_path)?,
        PatternFormat::Oxs => oxs::parse_pattern(file_path)?,
        PatternFormat::Json => {
          let content = std::fs::read_to_string(file_path)?;
          serde_json::from_str(&content).unwrap()
        }
      };
      state.patterns.insert(pattern_key, pattern.clone());
      pattern
    }
  };
  log::trace!("Pattern loaded");
  Ok(borsh::to_vec(&pattern).unwrap())
}

#[tauri::command]
pub fn create_pattern(state: tauri::State<AppStateType>) -> (PatternKey, Vec<u8>) {
  log::trace!("Creating new pattern");
  let mut state = state.write().unwrap();
  let file_path = PathBuf::from(format!("Untitled-{:?}.json", Instant::now()));
  let pattern_key = PatternKey::from(file_path);
  let pattern = Pattern::default();
  state.patterns.insert(pattern_key.clone(), pattern.clone());
  log::trace!("Pattern created");
  (pattern_key, borsh::to_vec(&pattern).unwrap())
}

// TODO: Use a custom or different pattern format, but not the JSON.
#[tauri::command]
pub fn save_pattern(pattern_key: PatternKey, file_path: PathBuf, state: tauri::State<AppStateType>) -> Result<()> {
  log::trace!("Saving pattern to {:?}", file_path);
  let state = state.read().unwrap();
  let pattern = state.patterns.get(&pattern_key).unwrap();
  fs::write(file_path, serde_json::to_string(pattern).unwrap())?;
  log::trace!("Pattern saved");
  Ok(())
}

#[tauri::command]
pub fn close_pattern(pattern_key: PatternKey, state: tauri::State<AppStateType>) {
  log::trace!("Closing pattern {:?}", pattern_key);
  state.write().unwrap().patterns.remove(&pattern_key);
  log::trace!("Pattern closed");
}

enum PatternFormat {
  Xsd,
  Oxs,
  Json,
}

impl TryFrom<Option<&OsStr>> for PatternFormat {
  type Error = Error;

  fn try_from(value: Option<&OsStr>) -> Result<Self, Self::Error> {
    if let Some(extension) = value {
      let extension = extension.to_str().unwrap();
      match extension.to_lowercase().as_str() {
        "xsd" => Ok(Self::Xsd),
        "oxs" | "xml" => Ok(Self::Oxs),
        "json" => Ok(Self::Json),
        _ => Err(Error::UnsupportedPatternType {
          extension: extension.to_uppercase(),
        }),
      }
    } else {
      Err(Error::UnsupportedPatternType {
        extension: "[no extension]".to_string(),
      })
    }
  }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[repr(transparent)]
pub struct PatternKey(String);

impl From<PathBuf> for PatternKey {
  fn from(value: PathBuf) -> Self {
    Self(value.to_string_lossy().to_string())
  }
}

pub type Coord = NotNan<f32>;

#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Pattern {
  properties: PatternProperties,
  info: PatternInfo,
  palette: Vec<PaletteItem>,
  fabric: Fabric,
  fullstitches: Stitches<FullStitch>,
  partstitches: Stitches<PartStitch>,
  nodes: Stitches<Node>,
  lines: Stitches<Line>,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
struct PatternProperties {
  width: u16,
  height: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
struct PatternInfo {
  title: String,
  author: String,
  copyright: String,
  description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
struct PaletteItem {
  brand: String,
  number: String,
  name: String,
  color: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  blends: Option<Vec<Blend>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
struct Blend {
  brand: String,
  number: String,
  strands: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
struct Fabric {
  #[serde(rename = "stitchesPerInch")]
  stitches_per_inch: (u16, u16),
  kind: String,
  name: String,
  color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(transparent)]
pub struct Stitches<T: Ord> {
  inner: BTreeSet<T>,
}

impl<T: Ord> Stitches<T> {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    Self { inner: BTreeSet::new() }
  }

  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.inner.iter()
  }

  pub fn insert(&mut self, stitch: T) -> Option<T> {
    self.inner.replace(stitch)
  }

  pub fn remove(&mut self, stitch: &T) -> bool {
    self.inner.remove(stitch)
  }

  pub fn get(&self, stitch: &T) -> Option<&T> {
    self.inner.get(stitch)
  }
}

impl<T: Ord> FromIterator<T> for Stitches<T> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    Self { inner: BTreeSet::from_iter(iter) }
  }
}

impl Stitches<FullStitch> {
  /// Removes and returns all the conflicts with a given full stitch.
  /// It looks for any petite stitches that overlap with the full stitch.
  pub fn remove_conflicts_with_full_stitch(&mut self, fullstitch: &FullStitch) -> Vec<FullStitch> {
    debug_assert_eq!(fullstitch.kind, FullStitchKind::Full);
    let mut conflicts = Vec::new();

    let x = fullstitch.x + 0.5;
    let y = fullstitch.y + 0.5;
    let kind = FullStitchKind::Petite;

    for petite in [
      FullStitch { kind, ..*fullstitch },
      FullStitch { x, kind, ..*fullstitch },
      FullStitch { y, kind, ..*fullstitch },
      FullStitch { x, y, kind, ..*fullstitch },
    ] {
      self.remove(&petite).then(|| conflicts.push(petite));
    }

    conflicts
  }

  /// Removes and returns all the conflicts with a given petite stitch.
  /// It looks for the full stitch that overlaps with the petite stitch.
  pub fn remove_conflicts_with_petite_stitch(&mut self, fullstitch: &FullStitch) -> Vec<FullStitch> {
    debug_assert_eq!(fullstitch.kind, FullStitchKind::Petite);
    let mut conflicts = Vec::new();

    let fullstitch = FullStitch {
      x: NotNan::new(fullstitch.x.trunc()).unwrap(),
      y: NotNan::new(fullstitch.y.trunc()).unwrap(),
      palindex: fullstitch.palindex,
      kind: FullStitchKind::Full,
    };

    self.remove(&fullstitch).then(|| conflicts.push(fullstitch));

    conflicts
  }

  /// Removes and returns all the conflicts with a given half stitch.
  /// It looks for the full and any petite stitches that overlap with the half stitch.
  pub fn remove_conflicts_with_half_stitch(&mut self, partstitch: &PartStitch) -> Vec<FullStitch> {
    debug_assert_eq!(partstitch.kind, PartStitchKind::Half);
    let mut conflicts = Vec::new();
    let fullstitch: FullStitch = partstitch.to_owned().into();

    let x = partstitch.x + 0.5;
    let y = partstitch.y + 0.5;
    let kind = FullStitchKind::Petite;
    match partstitch.direction {
      PartStitchDirection::Forward => {
        for petite in [
          FullStitch { x, kind, ..fullstitch },
          FullStitch { y, kind, ..fullstitch },
        ] {
          self.remove(&petite).then(|| conflicts.push(petite));
        }
      }
      PartStitchDirection::Backward => {
        for petite in [
          FullStitch { kind, ..fullstitch },
          FullStitch { x, y, kind, ..fullstitch },
        ] {
          self.remove(&petite).then(|| conflicts.push(petite));
        }
      }
    };

    self.remove(&fullstitch).then(|| conflicts.push(fullstitch));

    conflicts
  }

  /// Removes and returns all the conflicts with a given quarter stitch.
  /// It looks for the full and petite stitches that overlap with the quarter stitch.
  pub fn remove_conflicts_with_quarter_stitch(&mut self, partstitch: &PartStitch) -> Vec<FullStitch> {
    debug_assert_eq!(partstitch.kind, PartStitchKind::Quarter);
    let mut conflicts = Vec::new();

    for fullstitch in [
      FullStitch {
        x: NotNan::new(partstitch.x.trunc()).unwrap(),
        y: NotNan::new(partstitch.y.trunc()).unwrap(),
        palindex: partstitch.palindex,
        kind: FullStitchKind::Full,
      },
      partstitch.to_owned().into(), // Petite
    ] {
      self.remove(&fullstitch).then(|| conflicts.push(fullstitch));
    }

    conflicts
  }
}

impl Stitches<PartStitch> {
  /// Removes and returns all the conflicts with a given full stitch.
  /// It looks for any half and quarter stitches that overlap with the full stitch.
  pub fn remove_conflicts_with_full_stitch(&mut self, fullstitch: &FullStitch) -> Vec<PartStitch> {
    debug_assert_eq!(fullstitch.kind, FullStitchKind::Full);
    let mut conflicts = Vec::new();

    let partstitch: PartStitch = fullstitch.to_owned().into();
    let x = fullstitch.x + 0.5;
    let y = fullstitch.y + 0.5;

    for partstitch in [
      PartStitch {
        direction: PartStitchDirection::Forward,
        ..partstitch
      },
      PartStitch {
        direction: PartStitchDirection::Backward,
        ..partstitch
      },
      PartStitch {
        kind: PartStitchKind::Quarter,
        direction: PartStitchDirection::Backward,
        ..partstitch
      },
      PartStitch {
        x,
        kind: PartStitchKind::Quarter,
        direction: PartStitchDirection::Forward,
        ..partstitch
      },
      PartStitch {
        y,
        kind: PartStitchKind::Quarter,
        direction: PartStitchDirection::Forward,
        ..partstitch
      },
      PartStitch {
        x,
        y,
        kind: PartStitchKind::Quarter,
        direction: PartStitchDirection::Backward,
        ..partstitch
      },
    ] {
      self.remove(&partstitch).then(|| conflicts.push(partstitch));
    }

    conflicts
  }

  /// Removes and returns all the conflicts with a given petite stitch.
  /// It looks for the half and quarter stitches that overlap with the petite stitch.
  pub fn remove_conflicts_with_petite_stitch(&mut self, fullstitch: &FullStitch) -> Vec<PartStitch> {
    debug_assert_eq!(fullstitch.kind, FullStitchKind::Petite);
    let mut conflicts = Vec::new();

    let x = fullstitch.x;
    let y = fullstitch.y;
    let palindex = fullstitch.palindex;
    let direction = PartStitchDirection::from((x, y));

    let half = PartStitch {
      x: NotNan::new(x.trunc()).unwrap(),
      y: NotNan::new(y.trunc()).unwrap(),
      palindex,
      direction,
      kind: PartStitchKind::Half,
    };
    self.remove(&half).then(|| conflicts.push(half));

    let quarter = PartStitch {
      x,
      y,
      palindex,
      direction,
      kind: PartStitchKind::Quarter,
    };
    self.remove(&quarter).then(|| conflicts.push(quarter));

    conflicts
  }

  /// Removes and returns all the conflicts with a given half stitch.
  /// It looks for any quarter stitches that overlap with the half stitch.
  pub fn remove_conflicts_with_half_stitch(&mut self, partstitch: &PartStitch) -> Vec<PartStitch> {
    debug_assert_eq!(partstitch.kind, PartStitchKind::Half);
    let mut conflicts = Vec::new();

    let x = partstitch.x + 0.5;
    let y = partstitch.y + 0.5;
    let kind = PartStitchKind::Quarter;

    match partstitch.direction {
      PartStitchDirection::Forward => {
        for quarter in [
          PartStitch {
            x,
            kind,
            direction: PartStitchDirection::Forward,
            ..*partstitch
          },
          PartStitch {
            y,
            kind,
            direction: PartStitchDirection::Forward,
            ..*partstitch
          },
        ] {
          self.remove(&quarter).then(|| conflicts.push(quarter));
        }
      }
      PartStitchDirection::Backward => {
        for quarter in [
          PartStitch {
            kind,
            direction: PartStitchDirection::Backward,
            ..*partstitch
          },
          PartStitch {
            x,
            y,
            kind,
            direction: PartStitchDirection::Backward,
            ..*partstitch
          },
        ] {
          self.remove(&quarter).then(|| conflicts.push(quarter));
        }
      }
    }

    conflicts
  }

  /// Removes and returns all the conflicts with a given quarter stitch.
  /// It looks for the half stitch that overlap with the quarter stitch.
  pub fn remove_conflicts_with_quarter_stitch(&mut self, partstitch: &PartStitch) -> Vec<PartStitch> {
    debug_assert_eq!(partstitch.kind, PartStitchKind::Quarter);
    let mut conflicts = Vec::new();

    let half = PartStitch {
      x: NotNan::new(partstitch.x.trunc()).unwrap(),
      y: NotNan::new(partstitch.y.trunc()).unwrap(),
      palindex: partstitch.palindex,
      direction: PartStitchDirection::from((partstitch.x, partstitch.y)),
      kind: PartStitchKind::Half,
    };
    self.remove(&half).then(|| conflicts.push(half));

    conflicts
  }
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub enum Stitch {
  Full(FullStitch),
  Part(PartStitch),
  Node(Node),
  Line(Line),
}

#[derive(Debug, Default, Clone, BorshSerialize, BorshDeserialize)]
pub struct StitchConflicts {
  fullstitches: Vec<FullStitch>,
  partstitches: Vec<PartStitch>,
  node: Option<Node>,
  line: Option<Line>,
}

impl StitchConflicts {
  fn with_fullstitches(mut self, fullstitches: Vec<FullStitch>) -> Self {
    self.fullstitches = fullstitches;
    self
  }

  fn with_fullstitch(mut self, fullstitch: Option<FullStitch>) -> Self {
    if let Some(fullstitch) = fullstitch {
      self.fullstitches.push(fullstitch);
    }
    self
  }

  fn with_partstitches(mut self, partstitches: Vec<PartStitch>) -> Self {
    self.partstitches = partstitches;
    self
  }

  fn with_partstitch(mut self, partstitch: Option<PartStitch>) -> Self {
    if let Some(partstitch) = partstitch {
      self.partstitches.push(partstitch);
    }
    self
  }

  fn with_node(mut self, node: Option<Node>) -> Self {
    self.node = node;
    self
  }

  fn with_line(mut self, line: Option<Line>) -> Self {
    self.line = line;
    self
  }
}

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, BorshSerialize, BorshDeserialize,
)]
#[borsh(use_discriminant = true)]
pub enum FullStitchKind {
  Full = 0,
  Petite = 1,
}

impl From<PartStitchKind> for FullStitchKind {
  fn from(kind: PartStitchKind) -> Self {
    match kind {
      PartStitchKind::Half => FullStitchKind::Full,
      PartStitchKind::Quarter => FullStitchKind::Petite,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct FullStitch {
  pub x: Coord,
  pub y: Coord,
  pub palindex: u8,
  pub kind: FullStitchKind,
}

impl PartialOrd for FullStitch {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for FullStitch {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .y
      .cmp(&other.y)
      .then(self.x.cmp(&other.x))
      .then(self.kind.cmp(&other.kind))
  }
}

impl From<PartStitch> for FullStitch {
  fn from(partstitch: PartStitch) -> Self {
    Self {
      x: partstitch.x,
      y: partstitch.y,
      palindex: partstitch.palindex,
      kind: partstitch.kind.into(),
    }
  }
}

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, BorshSerialize, BorshDeserialize,
)]
#[borsh(use_discriminant = true)]
pub enum PartStitchDirection {
  Forward = 1,
  Backward = 2,
}

impl From<(Coord, Coord)> for PartStitchDirection {
  fn from((x, y): (Coord, Coord)) -> Self {
    if (x.fract() < 0.5 && y.fract() < 0.5) || (x.fract() >= 0.5 && y.fract() >= 0.5) {
      PartStitchDirection::Backward
    } else {
      PartStitchDirection::Forward
    }
  }
}

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, BorshSerialize, BorshDeserialize,
)]
#[borsh(use_discriminant = true)]
pub enum PartStitchKind {
  Half = 0,
  Quarter = 1,
}

impl From<FullStitchKind> for PartStitchKind {
  fn from(kind: FullStitchKind) -> Self {
    match kind {
      FullStitchKind::Full => PartStitchKind::Half,
      FullStitchKind::Petite => PartStitchKind::Quarter,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct PartStitch {
  pub x: Coord,
  pub y: Coord,
  pub palindex: u8,
  pub direction: PartStitchDirection,
  pub kind: PartStitchKind,
}

impl PartialOrd for PartStitch {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for PartStitch {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .y
      .cmp(&other.y)
      .then(self.x.cmp(&other.x))
      .then(self.kind.cmp(&other.kind))
  }
}

impl From<FullStitch> for PartStitch {
  fn from(fullstitch: FullStitch) -> Self {
    Self {
      x: fullstitch.x,
      y: fullstitch.y,
      palindex: fullstitch.palindex,
      direction: PartStitchDirection::from((fullstitch.x, fullstitch.y)),
      kind: fullstitch.kind.into(),
    }
  }
}

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, BorshSerialize, BorshDeserialize,
)]
#[borsh(use_discriminant = true)]
enum NodeKind {
  FrenchKnot = 0,
  Bead = 1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Node {
  x: Coord,
  y: Coord,
  rotated: bool,
  palindex: u8,
  kind: NodeKind,
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    self.y.cmp(&other.y).then(self.x.cmp(&other.x))
  }
}

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, BorshSerialize, BorshDeserialize,
)]
#[borsh(use_discriminant = true)]
enum LineKind {
  Back = 0,
  Straight = 1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Line {
  x: (Coord, Coord),
  y: (Coord, Coord),
  palindex: u8,
  kind: LineKind,
}

impl PartialOrd for Line {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Line {
  fn cmp(&self, other: &Self) -> Ordering {
    self.y.cmp(&other.y).then(self.x.cmp(&other.x))
  }
}

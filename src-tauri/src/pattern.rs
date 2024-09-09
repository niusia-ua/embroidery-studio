use serde::{Deserialize, Serialize, Serializer};
use std::{collections::BTreeMap, ffi::OsStr, path::PathBuf, time::Instant};

use crate::{error::*, state::AppStateType};

pub mod events;
mod oxs;
mod xsd;

#[tauri::command]
pub fn load_pattern(file_path: PathBuf, state: tauri::State<AppStateType>) -> Result<Pattern> {
  let mut state = state.write().unwrap();
  let pattern_key = PatternKey(file_path.clone());
  let pattern = match state.patterns.get(&pattern_key) {
    Some(pattern) => pattern.to_owned(),
    None => {
      let pattern_format = PatternFormat::try_from(file_path.extension())?;
      let pattern = match pattern_format {
        PatternFormat::XSD => xsd::parse_pattern(file_path)?,
        PatternFormat::OXS => oxs::parse_pattern(file_path)?,
      };
      state.patterns.insert(pattern_key, pattern.clone());
      pattern
    }
  };
  Ok(pattern)
}

#[tauri::command]
pub fn create_pattern(state: tauri::State<AppStateType>) -> (PatternKey, Pattern) {
  let mut state = state.write().unwrap();
  let file_path = PathBuf::from(format!("Untitled-{:?}.oxs", Instant::now()));
  let pattern_key = PatternKey(file_path);
  let pattern = Pattern::default();
  state.patterns.insert(pattern_key.clone(), pattern.clone());
  (pattern_key, pattern)
}

enum PatternFormat {
  XSD,
  OXS,
}

impl TryFrom<Option<&OsStr>> for PatternFormat {
  type Error = Error;

  fn try_from(value: Option<&OsStr>) -> Result<Self, Self::Error> {
    if let Some(extension) = value {
      let extension = extension.to_str().unwrap();
      match extension.to_lowercase().as_str() {
        "xsd" => Ok(Self::XSD),
        "oxs" | "xml" => Ok(Self::OXS),
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PatternKey(PathBuf);

#[derive(Debug, Clone, Serialize)]
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

// TODO: Load the default values from a bundlled pattern file.
impl Default for Pattern {
  fn default() -> Self {
    Self {
      properties: PatternProperties {
        width: 100,
        height: 100,
      },
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

#[derive(Debug, Clone, PartialEq, Serialize)]
struct PatternProperties {
  width: u16,
  height: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct PatternInfo {
  title: String,
  author: String,
  copyright: String,
  description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct PaletteItem {
  brand: String,
  number: String,
  name: String,
  color: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  blends: Option<Vec<Blend>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Blend {
  brand: String,
  number: String,
  strands: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Fabric {
  #[serde(rename = "stitchesPerInch")]
  stitches_per_inch: (u16, u16),
  kind: String,
  name: String,
  color: String,
}

pub trait Key {
  fn key(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Stitches<T> {
  inner: BTreeMap<String, T>,
}

impl<T> Stitches<T> {
  pub fn new() -> Self {
    Self {
      inner: BTreeMap::new(),
    }
  }

  pub fn len(&self) -> usize {
    self.inner.len()
  }
}

impl<T: Key> Stitches<T> {
  pub fn insert(&mut self, stitch: T) -> Option<T> {
    self.inner.insert(stitch.key(), stitch)
  }

  pub fn remove(&mut self, stitch: T) -> Option<T> {
    let key = stitch.key();
    self.inner.remove(&key)
  }

  pub fn get(&self, key: &str) -> Option<&T> {
    self.inner.get(key)
  }
}

impl<T: Clone + Serialize> Serialize for Stitches<T> {
  fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
    let stitches: Vec<T> = self.inner.values().cloned().collect();
    serde::Serialize::serialize(&stitches, ser)
  }
}

impl<T: Key> FromIterator<T> for Stitches<T> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    Self {
      inner: BTreeMap::from_iter(iter.into_iter().map(|item| (item.key(), item))),
    }
  }
}

impl Stitches<FullStitch> {
  pub fn find_conflicts_with_full_stitch(&self, fullstitch: &FullStitch) -> Vec<FullStitch> {
    assert_eq!(fullstitch.kind, FullStitchKind::Full);

    let x = fullstitch.x;
    let y = fullstitch.y;
    let palindex = fullstitch.palindex;
    let kind = FullStitchKind::Petite;

    let mut conflicts = Vec::new();
    for petite in [
      FullStitch {
        x,
        y,
        palindex,
        kind,
      },
      FullStitch {
        x: x + 0.5,
        y,
        palindex,
        kind,
      },
      FullStitch {
        x,
        y: y + 0.5,
        palindex,
        kind,
      },
      FullStitch {
        x: x + 0.5,
        y: y + 0.5,
        palindex,
        kind,
      },
    ] {
      if let Some(fullstitch) = self.get(&petite.key()) {
        conflicts.push(fullstitch.clone());
      }
    }
    conflicts
  }

  pub fn find_conflicts_with_petite_stitch(&self, fullstitch: &FullStitch) -> Option<FullStitch> {
    assert_eq!(fullstitch.kind, FullStitchKind::Petite);
    let fullstitch = FullStitch {
      x: fullstitch.x.trunc(),
      y: fullstitch.y.trunc(),
      palindex: fullstitch.palindex,
      kind: FullStitchKind::Full,
    };
    self.get(&fullstitch.key()).cloned()
  }

  pub fn find_conflicts_with_half_stitch(&self, partstitch: &PartStitch) -> Vec<FullStitch> {
    assert_eq!(partstitch.kind, PartStitchKind::Half);

    let x = partstitch.x;
    let y = partstitch.y;
    let palindex = partstitch.palindex;

    let mut conflicts = Vec::new();

    let fullstitch = FullStitch {
      x,
      y,
      palindex,
      kind: FullStitchKind::Full,
    };
    if let Some(fullstitch) = self.get(&fullstitch.key()) {
      conflicts.push(fullstitch.clone());
    }

    let kind = FullStitchKind::Petite;
    match partstitch.direction {
      PartStitchDirection::Forward => {
        for petite in [
          FullStitch {
            x: x + 0.5,
            y,
            palindex,
            kind,
          },
          FullStitch {
            x,
            y: y + 0.5,
            palindex,
            kind,
          },
        ] {
          if let Some(fullstitch) = self.get(&petite.key()) {
            conflicts.push(fullstitch.clone());
          }
        }
      }
      PartStitchDirection::Backward => {
        for petite in [
          FullStitch {
            x,
            y,
            palindex,
            kind,
          },
          FullStitch {
            x: x + 0.5,
            y: y + 0.5,
            palindex,
            kind,
          },
        ] {
          if let Some(fullstitch) = self.get(&petite.key()) {
            conflicts.push(fullstitch.clone());
          }
        }
      }
    };

    conflicts
  }

  pub fn find_conflicts_with_quarter_stitch(&self, partstitch: &PartStitch) -> Vec<FullStitch> {
    assert_eq!(partstitch.kind, PartStitchKind::Quarter);
    let mut conflicts = Vec::new();
    for fullstitch in [
      FullStitch {
        x: partstitch.x.trunc(),
        y: partstitch.y.trunc(),
        palindex: partstitch.palindex,
        kind: FullStitchKind::Full,
      },
      FullStitch {
        x: partstitch.x,
        y: partstitch.y,
        palindex: partstitch.palindex,
        kind: FullStitchKind::Petite,
      },
    ] {
      if let Some(fullstitch) = self.get(&fullstitch.key()) {
        conflicts.push(fullstitch.clone());
      }
    }
    conflicts
  }
}

impl Stitches<PartStitch> {
  pub fn find_conflicts_with_full_stitch(&self, fullstitch: &FullStitch) -> Vec<PartStitch> {
    assert_eq!(fullstitch.kind, FullStitchKind::Full);

    let x = fullstitch.x;
    let y = fullstitch.y;
    let palindex = fullstitch.palindex;

    let mut conflicts = Vec::new();
    for quarter in [
      PartStitch {
        x,
        y,
        palindex,
        kind: PartStitchKind::Half,
        direction: PartStitchDirection::Forward,
      },
      PartStitch {
        x,
        y,
        palindex,
        kind: PartStitchKind::Half,
        direction: PartStitchDirection::Backward,
      },
      PartStitch {
        x,
        y,
        palindex,
        kind: PartStitchKind::Quarter,
        direction: PartStitchDirection::Backward,
      },
      PartStitch {
        x: x + 0.5,
        y,
        palindex,
        kind: PartStitchKind::Quarter,
        direction: PartStitchDirection::Forward,
      },
      PartStitch {
        x,
        y: y + 0.5,
        palindex,
        kind: PartStitchKind::Quarter,
        direction: PartStitchDirection::Forward,
      },
      PartStitch {
        x: x + 0.5,
        y: y + 0.5,
        palindex,
        kind: PartStitchKind::Quarter,
        direction: PartStitchDirection::Backward,
      },
    ] {
      if let Some(partstitch) = self.get(&quarter.key()) {
        conflicts.push(partstitch.clone());
      }
    }
    conflicts
  }

  pub fn find_conflicts_with_petite_stitch(&self, fullstitch: &FullStitch) -> Vec<PartStitch> {
    assert_eq!(fullstitch.kind, FullStitchKind::Petite);

    let x = fullstitch.x;
    let x_fract = x.fract();
    let y = fullstitch.y;
    let y_fract = y.fract();
    let palindex = fullstitch.palindex;
    let direction = if (x_fract < 0.5 && y_fract < 0.5) || (x_fract >= 0.5 && y_fract >= 0.5) {
      PartStitchDirection::Backward
    } else {
      PartStitchDirection::Forward
    };

    let mut conflicts = Vec::new();

    let half = PartStitch {
      x,
      y,
      palindex,
      direction,
      kind: PartStitchKind::Half,
    };
    if let Some(half) = self.get(&half.key()) {
      conflicts.push(half.clone());
    }

    let quarter = PartStitch {
      x,
      y,
      palindex,
      direction,
      kind: PartStitchKind::Quarter,
    };
    if let Some(quarter) = self.get(&quarter.key()) {
      conflicts.push(quarter.clone());
    }

    conflicts
  }

  pub fn find_conflicts_with_half_stitch(&self, partstitch: &PartStitch) -> Vec<PartStitch> {
    assert_eq!(partstitch.kind, PartStitchKind::Half);

    let x = partstitch.x;
    let y = partstitch.y;
    let palindex = partstitch.palindex;

    let mut conflicts = Vec::new();
    match partstitch.direction {
      PartStitchDirection::Forward => {
        for quarter in [
          PartStitch {
            x: x + 0.5,
            y,
            palindex,
            kind: PartStitchKind::Quarter,
            direction: PartStitchDirection::Forward,
          },
          PartStitch {
            x,
            y: y + 0.5,
            palindex,
            kind: PartStitchKind::Quarter,
            direction: PartStitchDirection::Forward,
          },
        ] {
          if let Some(partstitch) = self.get(&quarter.key()) {
            conflicts.push(partstitch.clone());
          }
        }
      }
      PartStitchDirection::Backward => {
        for quarter in [
          PartStitch {
            x,
            y,
            palindex,
            kind: PartStitchKind::Quarter,
            direction: PartStitchDirection::Backward,
          },
          PartStitch {
            x: x + 0.5,
            y: y + 0.5,
            palindex,
            kind: PartStitchKind::Quarter,
            direction: PartStitchDirection::Backward,
          },
        ] {
          if let Some(partstitch) = self.get(&quarter.key()) {
            conflicts.push(partstitch.clone());
          }
        }
      }
    }
    conflicts
  }

  pub fn find_conflicts_with_quarter_stitch(&self, partstitch: &PartStitch) -> Option<PartStitch> {
    assert_eq!(partstitch.kind, PartStitchKind::Quarter);

    let x = partstitch.x;
    let x_fract = x.fract();
    let y = partstitch.y;
    let y_fract = y.fract();
    let palindex = partstitch.palindex;
    let direction = if (x_fract < 0.5 && y_fract < 0.5) || (x_fract >= 0.5 && y_fract >= 0.5) {
      PartStitchDirection::Backward
    } else {
      PartStitchDirection::Forward
    };

    let half = PartStitch {
      x: x.trunc(),
      y: y.trunc(),
      palindex,
      direction,
      kind: PartStitchKind::Half,
    };

    self.get(&half.key()).cloned()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FullStitch {
  pub x: f64,
  pub y: f64,
  pub palindex: u8,
  pub kind: FullStitchKind,
}

impl Key for FullStitch {
  fn key(&self) -> String {
    format!("{}:{}|{}", self.x, self.y, self.kind as u8)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FullStitchKind {
  Full,
  Petite,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartStitch {
  pub x: f64,
  pub y: f64,
  pub palindex: u8,
  pub direction: PartStitchDirection,
  pub kind: PartStitchKind,
}

impl Key for PartStitch {
  fn key(&self) -> String {
    format!(
      "{}:{}|{}|{}",
      self.x, self.y, self.direction as u8, self.kind as u8
    )
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PartStitchDirection {
  Forward,
  Backward,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PartStitchKind {
  Half,
  Quarter,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
  x: f64,
  y: f64,
  rotated: bool,
  palindex: u8,
  kind: NodeKind,
}

impl Key for Node {
  fn key(&self) -> String {
    format!("{}:{}", self.x, self.y)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum NodeKind {
  FrenchKnot,
  Bead,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Line {
  x: (f64, f64),
  y: (f64, f64),
  palindex: u8,
  kind: LineKind,
}

impl Key for Line {
  fn key(&self) -> String {
    format!("{}:{}:{}:{}", self.x.0, self.y.0, self.x.1, self.y.1)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum LineKind {
  Back,
  Straight,
}

use serde::{Serialize, Serializer};
use std::{collections::BTreeMap, ffi::OsStr, path::PathBuf};

use crate::error::*;

mod oxs;
mod xsd;

#[tauri::command]
pub fn load_pattern(file_path: PathBuf) -> Result<Pattern> {
  let pattern_format = PatternFormat::try_from(file_path.extension())?;
  match pattern_format {
    PatternFormat::XSD => xsd::parse_pattern(file_path),
    PatternFormat::OXS => oxs::parse_pattern(file_path),
  }
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

type StitchKey = Vec<u16>;

trait Key {
  fn key(&self) -> StitchKey;
}

#[derive(Debug)]
struct Stitches<T> {
  inner: BTreeMap<StitchKey, T>,
}

impl<T> Stitches<T> {
  fn new() -> Self {
    Self {
      inner: BTreeMap::new(),
    }
  }

  fn len(&self) -> usize {
    self.inner.len()
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

#[derive(Debug, Serialize)]
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

#[derive(Debug, PartialEq, Serialize)]
struct PatternProperties {
  width: u16,
  height: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
struct PatternInfo {
  title: String,
  author: String,
  copyright: String,
  description: String,
}

#[derive(Debug, PartialEq, Serialize)]
struct PaletteItem {
  brand: String,
  number: String,
  name: String,
  color: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  blends: Option<Vec<Blend>>,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
struct Blend {
  brand: String,
  number: String,
  strands: u8,
}

#[derive(Debug, PartialEq, Serialize)]
struct Fabric {
  #[serde(rename = "stitchesPerInch")]
  stitches_per_inch: (u16, u16),
  kind: String,
  name: String,
  color: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct FullStitch {
  x: f64,
  y: f64,
  palindex: u8,
  kind: FullStitchKind,
}

impl Key for FullStitch {
  fn key(&self) -> StitchKey {
    match self.kind {
      FullStitchKind::Full => vec![self.x as u16, self.y as u16],
      FullStitchKind::Petite => vec![(self.x * 2.0) as u16, (self.y * 2.0) as u16],
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
enum FullStitchKind {
  Full,
  Petite,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct PartStitch {
  x: f64,
  y: f64,
  palindex: u8,
  direction: PartStitchDirection,
  kind: PartStitchKind,
}

impl Key for PartStitch {
  fn key(&self) -> StitchKey {
    match self.kind {
      PartStitchKind::Half => vec![self.x as u16, self.y as u16],
      PartStitchKind::Quarter => vec![(self.x * 2.0) as u16, (self.y * 2.0) as u16],
    }
  }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
enum PartStitchDirection {
  Forward,
  Backward,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
enum PartStitchKind {
  Half,
  Quarter,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Node {
  x: f64,
  y: f64,
  rotated: bool,
  palindex: u8,
  kind: NodeKind,
}

impl Key for Node {
  fn key(&self) -> StitchKey {
    vec![(self.x * 2.0) as u16, (self.y * 2.0) as u16]
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
enum NodeKind {
  FrenchKnot,
  Bead,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct Line {
  x: (f64, f64),
  y: (f64, f64),
  palindex: u8,
  kind: LineKind,
}

impl Key for Line {
  fn key(&self) -> StitchKey {
    vec![
      (self.x.0 * 2.0) as u16,
      (self.y.0 * 2.0) as u16,
      (self.x.1 * 2.0) as u16,
      (self.y.1 * 2.0) as u16,
    ]
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
enum LineKind {
  Back,
  Straight,
}

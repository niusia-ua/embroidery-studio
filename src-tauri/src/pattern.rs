use serde::Serialize;
use std::{ffi::OsStr, path::Path};

use crate::error::*;

mod oxs;
mod xsd;

#[tauri::command]
pub fn load_pattern(file_path: &str) -> Result<Pattern> {
  let file_path = Path::new(file_path);
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

#[derive(Debug, Serialize)]
pub struct Pattern {
  properties: PatternProperties,
  info: PatternInfo,
  palette: Vec<PaletteItem>,
  fabric: Fabric,
  fullstitches: Vec<FullStitch>,
  partstitches: Vec<PartStitch>,
  nodes: Vec<Node>,
  lines: Vec<Line>,
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

#[derive(Debug, PartialEq, Serialize)]
struct FullStitch {
  x: f64,
  y: f64,
  palindex: u8,
  kind: FullStitchKind,
}

#[derive(Debug, PartialEq, Serialize)]
enum FullStitchKind {
  Full,
  Petite,
}

#[derive(Debug, PartialEq, Serialize)]
struct PartStitch {
  x: f64,
  y: f64,
  palindex: u8,
  direction: PartStitchDirection,
  kind: PartStitchKind,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
enum PartStitchDirection {
  Forward,
  Backward,
}

#[derive(Debug, PartialEq, Serialize)]
enum PartStitchKind {
  Half,
  Quarter,
}

#[derive(Debug, PartialEq, Serialize)]
struct Node {
  x: f64,
  y: f64,
  rotated: bool,
  palindex: u8,
  kind: NodeKind,
}

#[derive(Debug, PartialEq, Serialize)]
enum NodeKind {
  FrenchKnot,
  Bead,
}

#[derive(Debug, PartialEq, Serialize)]
struct Line {
  x: (f64, f64),
  y: (f64, f64),
  palindex: u8,
  kind: LineKind,
}

#[derive(Debug, PartialEq, Serialize)]
enum LineKind {
  Back,
  Straight,
}

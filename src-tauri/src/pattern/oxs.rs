use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use super::*;

#[cfg(test)]
#[path = "oxs.test.rs"]
mod oxs_tests;

pub fn parse_pattern(path: impl AsRef<Path>) -> Result<Pattern> {
  let xml = fs::read_to_string(path)?;
  let pattern: OxsPattern = serde_xml_rs::from_str(&xml).unwrap();

  let properties = &pattern.properties;
  let palette = pattern.palette.as_ref();
  let fabric = palette.first().unwrap();
  let fullstitches = pattern.fullstitches.as_ref();
  let partstitches = pattern.partstitches.as_ref();
  let backstitches = pattern.backstitches.as_ref();
  let ornaments = pattern.ornaments.as_ref();

  Ok(Pattern {
    properties: PatternProperties {
      width: properties.width,
      height: properties.height,
    },

    info: PatternInfo {
      title: properties.title.clone(),
      author: properties.author.clone(),
      copyright: properties.copyright.clone(),
      description: properties.description.clone(),
    },

    fabric: Fabric {
      stitches_per_inch: (
        properties.stitches_per_inch_x,
        properties.stitches_per_inch_y,
      ),
      kind: String::new(),
      name: fabric.name.clone(),
      color: fabric.color.clone(),
    },

    palette: palette
      .iter()
      .skip(1) // Skip the fabric.
      .map(|item| {
        let bn: Vec<&str> = item.number.split(" ").collect();
        PaletteItem {
          brand: bn.first().unwrap().to_string(),
          number: bn.last().unwrap().to_string(),
          name: item.name.clone(),
          color: item.color.clone(),
          blends: None,
        }
      })
      .collect(),

    fullstitches: Stitches::from_iter(fullstitches.iter().map(|stitch| FullStitch {
      x: stitch.x,
      y: stitch.y,
      palindex: stitch.palindex - 1,
      kind: FullStitchKind::Full,
    })),

    partstitches: Stitches::from_iter(
      partstitches
        .iter()
        .flat_map(|stitch| {
          let mut stitches = vec![];
          let direction = match stitch.direction {
            1 | 3 => PartStitchDirection::Backward,
            2 | 4 => PartStitchDirection::Forward,
            _ => panic!("Unknown part stitch direction"),
          };
          if stitch.palindex1 != 0 {
            stitches.push(PartStitch {
              x: stitch.x,
              y: stitch.y,
              palindex: stitch.palindex1 - 1,
              kind: PartStitchKind::Half,
              direction,
            });
          }
          if stitch.palindex2 != 0 {
            stitches.push(PartStitch {
              x: stitch.x,
              y: stitch.y,
              palindex: stitch.palindex2 - 1,
              kind: PartStitchKind::Half,
              direction,
            });
          }
          stitches
        })
        .chain(
          ornaments
            .iter()
            .filter(|obj| obj.kind == "quarter")
            .map(|obj| PartStitch {
              x: obj.x,
              y: obj.y,
              palindex: obj.palindex - 1,
              kind: PartStitchKind::Quarter,
              direction: PartStitchDirection::Forward,
            }),
        ),
    ),

    lines: Stitches::from_iter(backstitches.iter().map(|stitch| Line {
      x: (stitch.x1, stitch.x2),
      y: (stitch.y1, stitch.y2),
      palindex: stitch.palindex - 1,
      kind: LineKind::Back,
    })),

    nodes: Stitches::from_iter(
      ornaments
        .iter()
        .filter(|obj| obj.kind.starts_with("bead") | (obj.kind == "knot"))
        .map(|obj| {
          let kind = match obj.kind.as_str() {
            "knot" => NodeKind::FrenchKnot,
            _ => NodeKind::Bead,
          };
          Node {
            x: obj.x,
            y: obj.y,
            rotated: false,
            palindex: obj.palindex - 1,
            kind,
          }
        }),
    ),
  })
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsPattern {
  properties: OxsPatternProperties,
  palette: OxsPalette,
  fullstitches: OxsFullStitches,
  partstitches: OxsPartStitches,
  backstitches: OxsBackStitches,
  #[serde(rename = "ornaments_inc_knots_and_beads")]
  ornaments: OxsOrnaments,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsPatternProperties {
  #[serde(rename = "chartwidth")]
  width: u16,
  #[serde(rename = "chartheight")]
  height: u16,

  #[serde(rename = "charttitle")]
  title: String,
  author: String,
  copyright: String,
  #[serde(rename = "instructions")]
  description: String,

  #[serde(rename = "stitchesperinch")]
  stitches_per_inch_x: u16,
  #[serde(rename = "stitchesperinch_y")]
  stitches_per_inch_y: u16,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsPalette {
  #[serde(rename = "palette_item")]
  items: Vec<OxsPaletteItem>,
}

impl AsRef<Vec<OxsPaletteItem>> for OxsPalette {
  fn as_ref(&self) -> &Vec<OxsPaletteItem> {
    &self.items
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsPaletteItem {
  number: String,
  name: String,
  color: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsFullStitches {
  #[serde(rename = "stitch")]
  items: Vec<OxsFullStitch>,
}

impl AsRef<Vec<OxsFullStitch>> for OxsFullStitches {
  fn as_ref(&self) -> &Vec<OxsFullStitch> {
    &self.items
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsFullStitch {
  x: f64,
  y: f64,
  palindex: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsPartStitches {
  #[serde(rename = "partstitch")]
  items: Vec<OxsPartStitch>,
}

impl AsRef<Vec<OxsPartStitch>> for OxsPartStitches {
  fn as_ref(&self) -> &Vec<OxsPartStitch> {
    &self.items
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsPartStitch {
  x: f64,
  y: f64,
  palindex1: u8,
  palindex2: u8,
  direction: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsBackStitches {
  #[serde(rename = "backstitch")]
  items: Vec<OxsBackStitch>,
}

impl AsRef<Vec<OxsBackStitch>> for OxsBackStitches {
  fn as_ref(&self) -> &Vec<OxsBackStitch> {
    &self.items
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsBackStitch {
  x1: f64,
  y1: f64,
  x2: f64,
  y2: f64,
  palindex: u8,
  #[serde(rename = "objecttype")]
  kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsOrnaments {
  #[serde(rename = "object")]
  items: Vec<OxsOrnament>,
}

impl AsRef<Vec<OxsOrnament>> for OxsOrnaments {
  fn as_ref(&self) -> &Vec<OxsOrnament> {
    &self.items
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsOrnament {
  #[serde(rename = "x1")]
  x: f64,
  #[serde(rename = "y1")]
  y: f64,
  palindex: u8,
  #[serde(rename = "objecttype")]
  kind: String,
}

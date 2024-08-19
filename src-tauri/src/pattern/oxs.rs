use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use super::*;

#[cfg(test)]
#[path = "oxs.test.rs"]
mod oxs_tests;

pub fn parse_pattern(path: &Path) -> Result<Pattern> {
  let xml = fs::read_to_string(path)?;
  let pattern: OxsPattern = serde_xml_rs::from_str(&xml).unwrap();

  let properties = &pattern.properties;
  let palette = &pattern.palette.palette_item;
  let fabric = palette.first().unwrap();
  let fullstitches = &pattern.fullstitches.stitch;
  let partstitches = &pattern.partstitches.partstitch;
  let backstitches = &pattern.backstitches.backstitch;
  let objects = &pattern.ornaments_inc_knots_and_beads.object;

  Ok(Pattern {
    properties: PatternProperties {
      width: properties.chartwidth,
      height: properties.chartheight,
    },

    info: PatternInfo {
      title: properties.charttitle.clone(),
      author: properties.author.clone(),
      copyright: properties.copyright.clone(),
      description: properties.instructions.clone(),
    },

    fabric: Fabric {
      stitches_per_inch: (properties.stitchesperinch, properties.stitchesperinch_y),
      kind: String::new(),
      name: fabric.name.clone(),
      color: fabric.color.clone(),
    },

    palette: palette
      .iter()
      .skip(1) // Skip the fabric.
      .map(|item| PaletteItem {
        vendor_id: 0,
        number: item.number.clone(),
        name: item.name.clone(),
        color: item.color.clone(),
        blends: vec![],
      })
      .collect(),

    fullstitches: fullstitches
      .iter()
      .map(|stitch| FullStitch {
        x: stitch.x,
        y: stitch.y,
        palindex: stitch.palindex - 1,
        kind: FullStitchKind::Full,
      })
      .collect(),

    partstitches: partstitches
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
        objects
          .iter()
          .filter(|obj| obj.objecttype == "quarter")
          .map(|obj| PartStitch {
            x: obj.x1,
            y: obj.y1,
            palindex: obj.palindex - 1,
            kind: PartStitchKind::Quarter,
            direction: PartStitchDirection::Forward,
          }),
      )
      .collect(),

    lines: backstitches
      .iter()
      .map(|stitch| Line {
        x: (stitch.x1, stitch.x2),
        y: (stitch.y1, stitch.y2),
        palindex: stitch.palindex - 1,
        kind: LineKind::Back,
      })
      .collect(),

    nodes: objects
      .iter()
      .filter(|obj| obj.objecttype.starts_with("bead") | (obj.objecttype == "knot"))
      .map(|obj| {
        let kind = match obj.objecttype.as_str() {
          "knot" => NodeKind::FrenchKnot,
          _ => NodeKind::Bead,
        };
        Node {
          x: obj.x1,
          y: obj.y1,
          rotated: false,
          palindex: obj.palindex - 1,
          kind,
        }
      })
      .collect(),
  })
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsPattern {
  properties: OxsPatternProperties,
  palette: OxsPalette,
  fullstitches: OxsFullStitches,
  partstitches: OxsPartStitches,
  backstitches: OxsBackStitches,
  ornaments_inc_knots_and_beads: OxsObjects,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsPatternProperties {
  oxsversion: f64,
  software: String,
  software_version: String,
  chartheight: u16,
  chartwidth: u16,
  charttitle: String,
  author: String,
  copyright: String,
  instructions: String,
  stitchesperinch: u16,
  stitchesperinch_y: u16,
  palettecount: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsPalette {
  palette_item: Vec<OxsPaletteItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsPaletteItem {
  index: u8,
  number: String,
  name: String,
  color: String,
  printcolor: String,
  blendcolor: String,
  comments: String,
  strands: u8,
  symbol: String,
  dashpattern: String,
  bsstrands: u8,
  bscolor: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsFullStitches {
  stitch: Vec<OxsFullStitch>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsFullStitch {
  x: f64,
  y: f64,
  palindex: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsPartStitches {
  partstitch: Vec<OxsPartStitch>,
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
  backstitch: Vec<OxsBackStitch>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsBackStitch {
  x1: f64,
  y1: f64,
  x2: f64,
  y2: f64,
  palindex: u8,
  objecttype: String,
  sequence: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsObjects {
  object: Vec<OxsObject>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OxsObject {
  x1: f64,
  y1: f64,
  palindex: u8,
  objecttype: String,
}

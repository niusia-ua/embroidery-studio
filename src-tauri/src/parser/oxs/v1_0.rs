use quick_xml::events::Event;

use super::utils::{process_attributes, Software};
use crate::pattern::{display::DisplaySettings, print::PrintSettings, *};

#[cfg(test)]
#[path = "v1_0.test.rs"]
mod tests;

// TODO: Implement the comprehensive parser for the OXS 1.0 format
pub fn parse_pattern(file_path: std::path::PathBuf, software: Software) -> anyhow::Result<PatternProject> {
  log::trace!("OXS version is 1.0 in the {software:?} edition");

  let mut reader = quick_xml::Reader::from_file(&file_path)?;
  let mut buf = Vec::new();

  let mut properties = PatternProperties::default();
  let mut info = PatternInfo::default();
  let mut fabric = Fabric::default();
  let mut palette = Vec::new();
  let mut fullstitches = Vec::new();
  let mut partstitches = Vec::new();
  let mut backstitches = Vec::new();
  let mut nodes = Vec::new();

  loop {
    match reader.read_event_into(&mut buf) {
      Ok(Event::Start(ref e)) => log::trace!("Parsing {:?}", String::from_utf8(e.name().as_ref().to_vec())?),
      Ok(Event::Empty(ref e)) => match e.name().as_ref() {
        b"properties" => {
          let attributes = process_attributes(e.attributes())?;

          properties = PatternProperties {
            width: attributes.get("chartwidth").unwrap().parse()?,
            height: attributes.get("chartheight").unwrap().parse()?,
          };

          info = PatternInfo {
            title: attributes.get("charttitle").unwrap().to_owned(),
            author: attributes.get("author").unwrap().to_owned(),
            company: String::from(""),
            copyright: attributes.get("copyright").unwrap().to_owned(),
            description: attributes.get("instructions").unwrap().to_owned(),
          };

          fabric.spi = (
            attributes.get("stitchesperinch").unwrap().parse()?,
            attributes.get("stitchesperinch_y").unwrap().parse()?,
          );
        }
        b"palette_item" => {
          let attributes = process_attributes(e.attributes())?;
          let index: u8 = attributes.get("index").unwrap().parse()?;
          if index == 0 {
            fabric.name = attributes.get("name").unwrap().to_owned();
            fabric.color = attributes.get("color").unwrap().to_owned();
          } else {
            let number = attributes.get("number").unwrap().to_owned();
            let data = number.split(' ').collect::<Vec<_>>();
            palette.push(PaletteItem {
              brand: data.first().unwrap().to_string(),
              number: data.last().unwrap().to_string(),
              name: attributes.get("name").unwrap().to_owned(),
              color: attributes.get("color").unwrap().to_owned(),
              blends: None,
              bead: None,
              strands: StitchStrands::default(),
            });
          }
        }
        b"stitch" => {
          let attributes = process_attributes(e.attributes())?;
          fullstitches.push(FullStitch {
            x: attributes.get("x").unwrap().parse()?,
            y: attributes.get("y").unwrap().parse()?,
            palindex: attributes.get("palindex").unwrap().parse::<u8>()? - 1,
            kind: FullStitchKind::Full,
          });
        }
        b"partstitch" => {
          let attributes = process_attributes(e.attributes())?;

          let x: Coord = attributes.get("x").unwrap().parse()?;
          let y: Coord = attributes.get("y").unwrap().parse()?;

          let direction = match attributes.get("direction").unwrap().parse()? {
            1 | 3 => PartStitchDirection::Backward,
            2 | 4 => PartStitchDirection::Forward,
            _ => panic!("Unknown part stitch direction"),
          };

          let palindex1: u8 = attributes.get("palindex1").unwrap().parse()?;
          if palindex1 != 0 {
            partstitches.push(PartStitch {
              x,
              y,
              palindex: palindex1 - 1,
              kind: PartStitchKind::Half,
              direction,
            });
          }

          let palindex2: u8 = attributes.get("palindex2").unwrap().parse()?;
          if palindex2 != 0 {
            partstitches.push(PartStitch {
              x,
              y,
              palindex: palindex2 - 1,
              kind: PartStitchKind::Half,
              direction,
            });
          }
        }
        b"backstitch" => {
          let attributes = process_attributes(e.attributes())?;
          backstitches.push(Line {
            x: (
              attributes.get("x1").unwrap().parse()?,
              attributes.get("x2").unwrap().parse()?,
            ),
            y: (
              attributes.get("y1").unwrap().parse()?,
              attributes.get("y2").unwrap().parse()?,
            ),
            palindex: attributes.get("palindex").unwrap().parse::<u8>()? - 1,
            kind: LineKind::Back,
          });
        }
        b"object" => {
          let attributes = process_attributes(e.attributes())?;

          let x: Coord = attributes.get("x1").unwrap().parse()?;
          let y: Coord = attributes.get("y1").unwrap().parse()?;
          let palindex: u8 = attributes.get("palindex").unwrap().parse::<u8>()? - 1;
          let kind = attributes.get("objecttype").unwrap();

          if kind.starts_with("bead") || kind == "knot" {
            let kind = match kind.as_ref() {
              "knot" => NodeKind::FrenchKnot,
              _ => NodeKind::Bead,
            };
            nodes.push(Node {
              x,
              y,
              rotated: false,
              palindex,
              kind,
            });
          }
        }
        _ => {}
      },
      Ok(Event::End(ref e)) => {
        if e.name().as_ref() == b"chart" {
          break;
        }
      }
      // We don't expect to receive EOF here,
      // because we should have found the end of the `chart` tag.
      Ok(Event::Eof) => anyhow::bail!("Unexpected EOF"),
      Err(e) => anyhow::bail!("Error at position {}: {e:?}", reader.error_position()),
      _ => {}
    }
    buf.clear();
  }

  Ok(PatternProject {
    file_path: Some(file_path),
    display_settings: DisplaySettings::new(palette.len()),
    print_settings: PrintSettings::default(),
    pattern: Pattern {
      properties,
      info,
      palette,
      fabric,
      fullstitches: Stitches::from_iter(fullstitches),
      partstitches: Stitches::from_iter(partstitches),
      nodes: Stitches::from_iter(nodes),
      lines: Stitches::from_iter(backstitches),
      specialstitches: Stitches::new(),
      special_stitch_models: Vec::new(),
    },
  })
}

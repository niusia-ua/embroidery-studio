use std::io;
use std::str::FromStr;

use anyhow::Result;
use ordered_float::NotNan;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::{Reader, Writer};

use super::utils::*;
use crate::core::pattern::display::DisplaySettings;
use crate::core::pattern::print::PrintSettings;
use crate::core::pattern::*;

pub fn parse_pattern(file_path: std::path::PathBuf, software: Software) -> Result<PatternProject> {
  log::trace!("OXS version is 1.0 in the {software:?} edition");

  let mut buf = Vec::new();
  let mut reader = Reader::from_file(&file_path)?;

  reader.config_mut().expand_empty_elements = true;
  reader.config_mut().check_end_names = true;
  reader.config_mut().trim_text(true);

  let mut pattern = Pattern::default();
  let mut palette_size = None;

  loop {
    match reader.read_event_into(&mut buf) {
      Ok(Event::Start(ref e)) => {
        log::trace!("Parsing {:?}", String::from_utf8(e.name().as_ref().to_vec())?);
        match e.name().as_ref() {
          b"properties" => {
            let attributes = process_attributes(e.attributes())?;
            let (properties, info, spi, palsize) = read_pattern_properties(&attributes)?;
            pattern.properties = properties;
            pattern.info = info;
            pattern.fabric.spi = spi;
            palette_size = Some(palsize)
          }
          b"palette" => {
            if let Some(palette_size) = palette_size {
              let (fabric, palette) = read_palette(&mut reader, software, palette_size)?;
              pattern.fabric = Fabric {
                spi: pattern.fabric.spi,
                ..fabric
              };
              pattern.palette = palette;
            } else {
              anyhow::bail!("Palette size is not set or the pattern properties are not read yet");
            }
          }
          b"fullstitches" => pattern.fullstitches.extend(read_fullstitches(&mut reader)?),
          b"partstitches" => pattern.partstitches.extend(read_partstitches(&mut reader)?),
          b"backstitches" => pattern.lines.extend(read_lines(&mut reader)?),
          b"ornaments_inc_knots_and_beads" => {
            let (fullstitches, nodes, specialstitches) = read_ornaments(&mut reader)?;
            pattern.fullstitches.extend(fullstitches);
            pattern.nodes.extend(nodes);
            pattern.specialstitches.extend(specialstitches);
          }
          b"special_stitch_models" if software == Software::EmbroideryStudio => pattern
            .special_stitch_models
            .extend(read_special_stitch_models(&mut reader)?),
          _ => {}
        }
      }
      Ok(Event::End(ref e)) if e.name().as_ref() == b"chart" => break,
      // We don't expect to receive EOF here, because we should have found the end of the `chart` tag.
      Ok(Event::Eof) => anyhow::bail!("Unexpected EOF"),
      Err(e) => anyhow::bail!("Error at position {}: {e:?}", reader.error_position()),
      _ => {}
    }
    buf.clear();
  }

  Ok(PatternProject {
    file_path,
    display_settings: DisplaySettings::new(pattern.palette.len()),
    print_settings: PrintSettings::default(),
    pattern,
  })
}

pub fn save_pattern(patproj: &PatternProject) -> Result<()> {
  let file = std::fs::OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open(&patproj.file_path)?;

  // In the development mode, we want to have a pretty-printed XML file for easy debugging.
  #[cfg(debug_assertions)]
  let mut writer = Writer::new_with_indent(file, b' ', 2);
  #[cfg(not(debug_assertions))]
  let mut writer = Writer::new(file);

  writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
  writer.create_element("chart").write_inner_content(|writer| {
    let pattern = &patproj.pattern;
    write_pattern_properties(
      writer,
      &pattern.properties,
      &pattern.info,
      pattern.fabric.spi,
      pattern.palette.len(),
    )?;
    write_palette(writer, &pattern.palette, &pattern.fabric)?;
    write_fullstitches(writer, &pattern.fullstitches)?;
    write_partstitches(writer, &pattern.partstitches)?;
    write_lines(writer, &pattern.lines)?;
    write_ornaments(writer, &pattern.fullstitches, &pattern.nodes, &pattern.specialstitches)?;
    write_special_stitch_models(writer, &pattern.special_stitch_models)?;
    Ok(())
  })?;

  Ok(())
}

fn read_pattern_properties(
  attributes: &MapAttributes,
) -> Result<(PatternProperties, PatternInfo, StitchesPerInch, usize)> {
  let properties = PatternProperties {
    width: attributes.get("chartwidth").unwrap().parse()?,
    height: attributes.get("chartheight").unwrap().parse()?,
  };

  let info = PatternInfo {
    title: attributes.get("charttitle").unwrap().to_owned(),
    author: attributes.get("author").unwrap().to_owned(),
    company: attributes.get("company").unwrap_or(&String::new()).to_owned(),
    copyright: attributes.get("copyright").unwrap().to_owned(),
    description: attributes.get("instructions").unwrap().to_owned(),
  };

  let spi = (
    attributes.get("stitchesperinch").unwrap().parse()?,
    attributes.get("stitchesperinch_y").unwrap().parse()?,
  );

  let palette_size: usize = attributes.get("palettecount").unwrap().parse()?;

  Ok((properties, info, spi, palette_size))
}

fn write_pattern_properties<W: io::Write>(
  writer: &mut Writer<W>,
  properties: &PatternProperties,
  info: &PatternInfo,
  spi: StitchesPerInch,
  palette_size: usize,
) -> io::Result<()> {
  writer
    .create_element("properties")
    .with_attributes([
      ("oxsversion", "1.0"),
      ("software", "Embroidery Studio"),
      // ("software_version", "0.0.0"),
      ("chartwidth", properties.width.to_string().as_str()),
      ("chartheight", properties.height.to_string().as_str()),
      ("charttitle", info.title.as_str()),
      ("author", info.author.as_str()),
      ("company", info.company.as_str()),
      ("copyright", info.copyright.as_str()),
      ("instructions", info.description.as_str()),
      ("stitchesperinch", spi.0.to_string().as_str()),
      ("stitchesperinch_y", spi.1.to_string().as_str()),
      ("palettecount", palette_size.to_string().as_str()),
    ])
    .write_empty()?;
  Ok(())
}

fn read_palette<R: io::BufRead>(
  reader: &mut Reader<R>,
  software: Software,
  palette_size: usize,
) -> Result<(Fabric, Vec<PaletteItem>)> {
  fn parse_brand_and_number(value: &str) -> (String, String) {
    let data = value.split(' ').collect::<Vec<_>>();
    (
      data[0..(data.len() - 1)].join(" ").trim_end().to_string(),
      data.last().unwrap().to_string(),
    )
  }

  let mut buf = Vec::new();
  let fabric = if let Event::Start(ref e) = reader.read_event_into(&mut buf)? {
    let attributes = process_attributes(e.attributes())?;
    Fabric {
      name: attributes.get("name").unwrap().to_owned(),
      color: attributes.get("color").unwrap().to_owned(),
      kind: attributes.get("kind").unwrap_or(&String::from("Aida")).to_owned(),
      ..Fabric::default()
    }
  } else {
    anyhow::bail!("Expected a start tag for the fabric palette item")
  };
  reader.read_event_into(&mut buf)?; // end of the fabric palette item tag

  let mut palette = Vec::with_capacity(palette_size);
  for _ in 0..palette_size {
    buf.clear();
    if let Event::Start(ref e) = reader.read_event_into(&mut buf)? {
      let attributes = process_attributes(e.attributes())?;
      let mut palette_item = {
        let (brand, number) = parse_brand_and_number(attributes.get("number").unwrap());
        PaletteItem {
          brand,
          number,
          name: attributes.get("name").unwrap().to_owned(),
          color: attributes.get("color").unwrap().to_owned(),
          blends: None,
          bead: None,
          strands: None,
        }
      };

      if software == Software::EmbroideryStudio {
        let mut buf = Vec::new();
        let blendscount: usize = attributes.get("blendscount").unwrap().parse()?;
        for _ in 0..blendscount {
          if let Event::Start(ref e) = reader.read_event_into(&mut buf)? {
            let attributes = process_attributes(e.attributes())?;
            let (brand, number) = parse_brand_and_number(attributes.get("number").unwrap());
            palette_item
              .blends
              .get_or_insert(Vec::with_capacity(blendscount))
              .push(Blend {
                brand,
                number,
                strands: BlendStrands::new(1),
              });
            reader.read_event_into(&mut buf)?; // end of the blend tag
          }
          buf.clear();
        }
      }

      palette.push(palette_item);

      // Skip the rest of the palette item tag.
      reader.read_to_end_into(e.to_end().name(), &mut Vec::new())?;
    } else {
      anyhow::bail!("Expected a start tag for the fabric palette item")
    }
  }

  Ok((fabric, palette))
}

fn write_palette<W: io::Write>(writer: &mut Writer<W>, palette: &[PaletteItem], fabric: &Fabric) -> io::Result<()> {
  writer.create_element("palette").write_inner_content(|writer| {
    writer
      .create_element("palette_item")
      .with_attributes([
        ("index", "0"),
        ("number", "cloth"),
        ("name", fabric.name.as_str()),
        ("color", fabric.color.as_str()),
        ("kind", fabric.kind.as_str()),
      ])
      .write_empty()?;

    for (index, pi) in palette.iter().enumerate() {
      writer
        .create_element("palette_item")
        .with_attributes([
          ("index", (index + 1).to_string().as_str()),
          ("number", format!("{} {}", pi.brand, pi.number).as_str()),
          ("name", pi.name.as_str()),
          ("color", pi.color.as_str()),
          (
            "blendscount",
            pi.blends
              .as_ref()
              .map_or(String::from("0"), |blends| blends.len().to_string())
              .as_str(),
          ),
        ])
        .write_inner_content(|writer| {
          if let Some(blends) = &pi.blends {
            for blend in blends.iter() {
              writer
                .create_element("blend")
                .with_attribute(("number", format!("{} {}", blend.brand, blend.number).as_str()))
                .write_empty()?;
            }
          }
          Ok(())
        })?;
    }

    Ok(())
  })?;
  Ok(())
}

fn read_fullstitches<R: io::BufRead>(reader: &mut Reader<R>) -> Result<Stitches<FullStitch>> {
  let mut buf = Vec::new();
  let mut fullstitches = Stitches::new();
  loop {
    match reader.read_event_into(&mut buf)? {
      Event::Start(ref e) if e.name().as_ref() == b"stitch" => {
        let attributes = process_attributes(e.attributes())?;
        fullstitches.insert(FullStitch {
          x: attributes.get("x").unwrap().parse()?,
          y: attributes.get("y").unwrap().parse()?,
          palindex: attributes.get("palindex").unwrap().parse::<u8>()? - 1,
          kind: FullStitchKind::Full,
        });
      }
      Event::End(ref e) if e.name().as_ref() == b"fullstitches" => break,
      _ => {}
    }
    buf.clear();
  }
  Ok(fullstitches)
}

fn write_fullstitches<W: io::Write>(writer: &mut Writer<W>, fullstitches: &Stitches<FullStitch>) -> io::Result<()> {
  writer.create_element("fullstitches").write_inner_content(|writer| {
    for fullstitch in fullstitches.iter().filter(|fs| fs.kind == FullStitchKind::Full) {
      writer
        .create_element("stitch")
        .with_attributes([
          ("x", fullstitch.x.to_string().as_str()),
          ("y", fullstitch.y.to_string().as_str()),
          ("palindex", (fullstitch.palindex + 1).to_string().as_str()),
        ])
        .write_empty()?;
    }
    Ok(())
  })?;
  Ok(())
}

fn read_partstitches<R: io::BufRead>(reader: &mut Reader<R>) -> Result<Stitches<PartStitch>> {
  let mut buf = Vec::new();
  let mut partstitches = Stitches::new();
  loop {
    match reader.read_event_into(&mut buf)? {
      Event::Start(ref e) if e.name().as_ref() == b"partstitch" => {
        let attributes = process_attributes(e.attributes())?;

        let x: Coord = attributes.get("x").unwrap().parse()?;
        let y: Coord = attributes.get("y").unwrap().parse()?;

        let direction_value: u8 = attributes.get("direction").unwrap().parse()?;
        let direction = match direction_value {
          1 | 3 => PartStitchDirection::Forward,
          2 | 4 => PartStitchDirection::Backward,
          _ => anyhow::bail!("Unknown part stitch direction"),
        };
        let kind = match direction_value {
          1 | 2 => PartStitchKind::Quarter,
          3 | 4 => PartStitchKind::Half,
          _ => anyhow::bail!("Unknown part stitch kind"),
        };

        let palindex1: u8 = attributes.get("palindex1").unwrap().parse()?;
        let palindex2: u8 = attributes.get("palindex2").unwrap().parse()?;

        if palindex1 != 0 {
          let (x, y) = if direction_value == 1 { (x, y + 0.5) } else { (x, y) };
          partstitches.insert(PartStitch {
            x,
            y,
            palindex: palindex1 - 1,
            kind,
            direction,
          });
        }

        if palindex2 != 0 {
          let (x, y) = if direction_value == 1 {
            (x + 0.5, y)
          } else if direction_value == 2 {
            (x + 0.5, y + 0.5)
          } else {
            (x, y)
          };
          partstitches.insert(PartStitch {
            x,
            y,
            palindex: palindex2 - 1,
            kind,
            direction,
          });
        }
      }
      Event::End(ref e) if e.name().as_ref() == b"partstitches" => break,
      _ => {}
    }
    buf.clear();
  }
  Ok(partstitches)
}

fn write_partstitches<W: io::Write>(writer: &mut Writer<W>, partstitches: &Stitches<PartStitch>) -> io::Result<()> {
  writer.create_element("partstitches").write_inner_content(|writer| {
    // The Ursa Software's OXS uses a quite unusual way to store quarter stitches.
    // It stores them as two half stitches with the same coordinates
    // and their kind and color depends on the combination of palindex* and direction.
    // To be compatible with this format we need to look ahead for the needed another quarter stitch.
    // We store the coordinates of the seen quarter stitches in the `seen_quarters` set.
    let mut seen_quarters = std::collections::HashSet::new();
    for partstitch in partstitches.iter() {
      let (palindex1, palindex2) = match partstitch.kind {
        PartStitchKind::Half => (partstitch.palindex + 1, 0u8),
        PartStitchKind::Quarter => {
          if seen_quarters.contains(&(partstitch.x, partstitch.y)) {
            continue;
          }
          seen_quarters.insert((partstitch.x, partstitch.y));

          let mut indices = (0, 0);

          match partstitch.direction {
            PartStitchDirection::Forward => {
              if partstitch.is_on_bottom_left() {
                indices.0 = partstitch.palindex + 1;
              } else if let Some(partstitch) = partstitches.get(&PartStitch {
                x: NotNan::new(partstitch.x.floor()).unwrap(),
                y: partstitch.y + 0.5,
                ..*partstitch
              }) {
                seen_quarters.insert((partstitch.x, partstitch.y));
                indices.0 = partstitch.palindex + 1;
              }

              if partstitch.is_on_top_right() {
                indices.1 = partstitch.palindex + 1;
              } else if let Some(partstitch) = partstitches.get(&PartStitch {
                x: partstitch.x + 0.5,
                y: NotNan::new(partstitch.y.floor()).unwrap(),
                ..*partstitch
              }) {
                seen_quarters.insert((partstitch.x, partstitch.y));
                indices.1 = partstitch.palindex + 1;
              }
            }

            PartStitchDirection::Backward => {
              if partstitch.is_on_top_left() {
                indices.0 = partstitch.palindex + 1;
              } else if let Some(partstitch) = partstitches.get(&PartStitch {
                x: NotNan::new(partstitch.x.floor()).unwrap(),
                y: NotNan::new(partstitch.y.floor()).unwrap(),
                ..*partstitch
              }) {
                seen_quarters.insert((partstitch.x, partstitch.y));
                indices.0 = partstitch.palindex + 1;
              }

              if partstitch.is_on_bottom_right() {
                indices.1 = partstitch.palindex + 1;
              } else if let Some(partstitch) = partstitches.get(&PartStitch {
                x: partstitch.x + 0.5,
                y: partstitch.y + 0.5,
                ..*partstitch
              }) {
                seen_quarters.insert((partstitch.x, partstitch.y));
                indices.1 = partstitch.palindex + 1;
              }
            }
          };

          indices
        }
      };

      writer
        .create_element("partstitch")
        .with_attributes([
          ("x", partstitch.x.trunc().to_string().as_str()),
          ("y", partstitch.y.trunc().to_string().as_str()),
          ("palindex1", palindex1.to_string().as_str()),
          ("palindex2", palindex2.to_string().as_str()),
          (
            "direction",
            (match partstitch.kind {
              PartStitchKind::Half => partstitch.direction as u8 + 2,
              PartStitchKind::Quarter => partstitch.direction as u8,
            })
            .to_string()
            .as_str(),
          ),
        ])
        .write_empty()?;
    }

    Ok(())
  })?;
  Ok(())
}

fn read_lines<R: io::BufRead>(reader: &mut Reader<R>) -> Result<Stitches<Line>> {
  let mut buf = Vec::new();
  let mut lines = Stitches::new();
  loop {
    match reader.read_event_into(&mut buf)? {
      Event::Start(ref e) if e.name().as_ref() == b"backstitch" => {
        let attributes = process_attributes(e.attributes())?;
        lines.insert(Line {
          x: (
            attributes.get("x1").unwrap().parse()?,
            attributes.get("x2").unwrap().parse()?,
          ),
          y: (
            attributes.get("y1").unwrap().parse()?,
            attributes.get("y2").unwrap().parse()?,
          ),
          palindex: attributes.get("palindex").unwrap().parse::<u8>()? - 1,
          kind: attributes
            .get("objecttype")
            .unwrap()
            .parse::<LineKind>()
            .map_err(|e| anyhow::anyhow!(e))?,
        });
      }
      Event::End(ref e) if e.name().as_ref() == b"backstitches" => break,
      _ => {}
    }
    buf.clear();
  }
  Ok(lines)
}

fn write_lines<W: io::Write>(writer: &mut Writer<W>, lines: &Stitches<Line>) -> io::Result<()> {
  writer.create_element("backstitches").write_inner_content(|writer| {
    for line in lines.iter() {
      writer
        .create_element("backstitch")
        .with_attributes([
          ("x1", line.x.0.to_string().as_str()),
          ("y1", line.y.0.to_string().as_str()),
          ("x2", line.x.1.to_string().as_str()),
          ("y2", line.y.1.to_string().as_str()),
          ("palindex", (line.palindex + 1).to_string().as_str()),
          ("objecttype", line.kind.to_string().as_str()),
        ])
        .write_empty()?;
    }
    Ok(())
  })?;
  Ok(())
}

fn read_ornaments<R: io::BufRead>(
  reader: &mut Reader<R>,
) -> Result<(Stitches<FullStitch>, Stitches<Node>, Stitches<SpecialStitch>)> {
  let mut buf = Vec::new();
  let mut fullstitches = Stitches::new();
  let mut nodes = Stitches::new();
  let mut specialstitches = Stitches::new();
  loop {
    match reader.read_event_into(&mut buf)? {
      Event::Start(ref e) if e.name().as_ref() == b"object" => {
        let attributes = process_attributes(e.attributes())?;

        let x: Coord = attributes.get("x1").unwrap().parse()?;
        let y: Coord = attributes.get("y1").unwrap().parse()?;
        let rotated = if let Some(rotated) = attributes.get("rotated") {
          rotated.parse()?
        } else {
          false
        };
        let palindex: u8 = attributes.get("palindex").unwrap().parse::<u8>()? - 1;
        let kind = attributes.get("objecttype").unwrap();

        // Yes, the Ursa Software's OXS format uses the "quarter" stitch for petites.
        if kind == "quarter" {
          let kind = FullStitchKind::Petite;
          fullstitches.insert(FullStitch { x, y, palindex, kind });
        }

        if kind.starts_with("bead") || kind == "knot" {
          let kind = NodeKind::from_str(kind).unwrap();
          nodes.insert(Node { x, y, rotated, palindex, kind });
        }

        if kind == "special" {
          specialstitches.insert(SpecialStitch {
            x,
            y,
            rotation: attributes.get("rotation").unwrap().parse()?,
            flip: (
              attributes.get("flip_x").unwrap().parse()?,
              attributes.get("flip_y").unwrap().parse()?,
            ),
            palindex,
            modindex: attributes.get("modindex").unwrap().parse()?,
          });
        }
      }
      Event::End(ref e) if e.name().as_ref() == b"ornaments_inc_knots_and_beads" => break,
      _ => {}
    }
    buf.clear();
  }
  Ok((fullstitches, nodes, specialstitches))
}

fn write_ornaments<W: io::Write>(
  writer: &mut Writer<W>,
  fullstitches: &Stitches<FullStitch>,
  nodes: &Stitches<Node>,
  specialstitches: &Stitches<SpecialStitch>,
) -> io::Result<()> {
  writer
    .create_element("ornaments_inc_knots_and_beads")
    .write_inner_content(|writer| {
      for fullstitch in fullstitches.iter().filter(|fs| fs.kind == FullStitchKind::Petite) {
        writer
          .create_element("object")
          .with_attributes([
            ("x1", fullstitch.x.to_string().as_str()),
            ("y1", fullstitch.y.to_string().as_str()),
            ("palindex", (fullstitch.palindex + 1).to_string().as_str()),
            // Yes, the Ursa Software's OXS format uses the "quarter" stitch for petites.
            ("objecttype", "quarter"),
          ])
          .write_empty()?;
      }

      for node in nodes.iter() {
        writer
          .create_element("object")
          .with_attributes([
            ("x1", node.x.to_string().as_str()),
            ("y1", node.y.to_string().as_str()),
            ("rotated", node.rotated.to_string().as_str()),
            ("palindex", (node.palindex + 1).to_string().as_str()),
            ("objecttype", node.kind.to_string().as_str()),
          ])
          .write_empty()?;
      }

      for specialstitch in specialstitches.iter() {
        writer
          .create_element("object")
          .with_attributes([
            ("x1", specialstitch.x.to_string().as_str()),
            ("y1", specialstitch.y.to_string().as_str()),
            ("rotation", specialstitch.rotation.to_string().as_str()),
            ("flip_x", specialstitch.flip.0.to_string().as_str()),
            ("flip_y", specialstitch.flip.1.to_string().as_str()),
            ("palindex", (specialstitch.palindex + 1).to_string().as_str()),
            ("modindex", specialstitch.modindex.to_string().as_str()),
            ("objecttype", "special"),
          ])
          .write_empty()?;
      }

      Ok(())
    })?;
  Ok(())
}

fn read_special_stitch_models<R: io::BufRead>(reader: &mut Reader<R>) -> Result<Vec<SpecialStitchModel>> {
  let mut buf = Vec::new();
  let mut special_stitch_models = Vec::new();
  loop {
    match reader.read_event_into(&mut buf)? {
      Event::Start(ref e) if e.name().as_ref() == b"model" => {
        let attributes = process_attributes(e.attributes())?;
        let unique_name = attributes.get("unique_name").unwrap().to_owned();
        let name = attributes.get("name").unwrap().to_owned();
        let mut lines = Vec::new();
        let mut nodes = Vec::new();
        let mut curves = Vec::new();
        loop {
          match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) if e.name().as_ref() == b"line" => {
              let attributes = process_attributes(e.attributes())?;
              lines.push(Line {
                x: (
                  attributes.get("x1").unwrap().parse()?,
                  attributes.get("x2").unwrap().parse()?,
                ),
                y: (
                  attributes.get("y1").unwrap().parse()?,
                  attributes.get("y2").unwrap().parse()?,
                ),
                kind: attributes
                  .get("kind")
                  .unwrap()
                  .parse::<LineKind>()
                  .map_err(|e| anyhow::anyhow!(e))?,
                palindex: 0,
              });
            }
            Event::Start(ref e) if e.name().as_ref() == b"node" => {
              let attributes = process_attributes(e.attributes())?;
              nodes.push(Node {
                x: attributes.get("x").unwrap().parse()?,
                y: attributes.get("y").unwrap().parse()?,
                rotated: attributes.get("rotated").unwrap().parse()?,
                kind: attributes
                  .get("kind")
                  .unwrap()
                  .parse::<NodeKind>()
                  .map_err(|e| anyhow::anyhow!(e))?,
                palindex: 0,
              });
            }
            Event::Start(ref e) if e.name().as_ref() == b"curve" => {
              let mut points = Vec::new();
              loop {
                match reader.read_event_into(&mut buf)? {
                  Event::Start(ref e) if e.name().as_ref() == b"point" => {
                    let attributes = process_attributes(e.attributes())?;
                    points.push((
                      attributes.get("x").unwrap().parse()?,
                      attributes.get("y").unwrap().parse()?,
                    ));
                  }
                  Event::End(ref e) if e.name().as_ref() == b"curve" => {
                    curves.push(Curve { points });
                    break;
                  }
                  _ => {}
                }
                buf.clear();
              }
            }
            Event::End(ref e) if e.name().as_ref() == b"model" => {
              special_stitch_models.push(SpecialStitchModel {
                unique_name,
                name,
                lines,
                nodes,
                curves,
              });
              break;
            }
            _ => {}
          }
        }
      }
      Event::End(ref e) if e.name().as_ref() == b"special_stitch_models" => break,
      _ => {}
    }
  }

  Ok(special_stitch_models)
}

fn write_special_stitch_models<W: io::Write>(
  writer: &mut Writer<W>,
  spsmodels: &[SpecialStitchModel],
) -> io::Result<()> {
  writer
    .create_element("special_stitch_models")
    .write_inner_content(|writer| {
      for model in spsmodels.iter() {
        writer
          .create_element("model")
          .with_attributes([
            ("unique_name", model.unique_name.as_str()),
            ("name", model.name.as_str()),
          ])
          .write_inner_content(|writer| {
            for line in model.lines.iter() {
              writer
                .create_element("line")
                .with_attributes([
                  ("x1", line.x.0.to_string().as_str()),
                  ("y1", line.y.0.to_string().as_str()),
                  ("x2", line.x.1.to_string().as_str()),
                  ("y2", line.y.1.to_string().as_str()),
                  ("kind", line.kind.to_string().as_str()),
                ])
                .write_empty()?;
            }

            for node in model.nodes.iter() {
              writer
                .create_element("node")
                .with_attributes([
                  ("x", node.x.to_string().as_str()),
                  ("y", node.y.to_string().as_str()),
                  ("rotated", node.rotated.to_string().as_str()),
                  ("kind", node.kind.to_string().as_str()),
                ])
                .write_empty()?;
            }

            for curve in model.curves.iter() {
              writer.create_element("curve").write_inner_content(|writer| {
                for point in curve.points.iter() {
                  writer
                    .create_element("point")
                    .with_attributes([("x", point.0.to_string().as_str()), ("y", point.1.to_string().as_str())])
                    .write_empty()?;
                }
                Ok(())
              })?;
            }

            Ok(())
          })?;
      }
      Ok(())
    })?;
  Ok(())
}

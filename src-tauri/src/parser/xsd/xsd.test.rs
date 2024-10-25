use std::{fs::File, io::Cursor};

use super::*;

fn load_fixture(name: &str) -> File {
  let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("testdata/xsd")
    .join(name);
  File::open(path).unwrap()
}

#[test]
fn reads_signature() {
  let buf: Vec<u8> = vec![0x10, 0x05];
  assert_eq!(read_signature(&mut Cursor::new(buf)).unwrap(), VALID_SIGNATURE);

  let buf = vec![0x00, 0x00];
  assert_ne!(read_signature(&mut Cursor::new(buf)).unwrap(), VALID_SIGNATURE);
}

#[test]
fn reads_palette() {
  let loaded_palette = read_palette(&mut load_fixture("palette")).unwrap();
  let expected_palette = vec![
    PaletteItem {
      brand: String::from("DMC"),
      number: String::from("310"),
      name: String::from("Black"),
      color: String::from("2C3225"),
      blends: None,
      bead: None,
      strands: StitchStrands::default(),
    },
    PaletteItem {
      brand: String::from("PNK Kirova"),
      number: String::from("9224"),
      name: String::from("ПНК Кирова"),
      color: String::from("B40032"),
      blends: None,
      bead: None,
      strands: StitchStrands::default(),
    },
    PaletteItem {
      brand: String::from("Mill Hill Frosted Glass Seed Bead"),
      number: String::from("62038"),
      name: String::from("Frosted Aquamarine"),
      color: String::from("A6D3D9"),
      blends: None,
      bead: Some(Bead {
        length: NotNan::new(1.5).unwrap(),
        diameter: NotNan::new(2.5).unwrap(),
      }),
      strands: StitchStrands::default(),
    },
    PaletteItem {
      brand: String::from("Blend"),
      number: String::from("11"),
      name: String::from(""),
      color: String::from("93D0D3"),
      blends: Some(vec![
        Blend {
          brand: String::from("DMC"),
          number: String::from("964"),
          strands: 1,
        },
        Blend {
          brand: String::from("DMC"),
          number: String::from("3766"),
          strands: 1,
        },
      ]),
      bead: None,
      strands: StitchStrands {
        full: Some(2),
        petite: Some(2),
        half: Some(2),
        quarter: Some(2),
        back: Some(2),
        straight: Some(2),
        french_knot: Some(2),
        special: Some(2),
      },
    },
  ];
  for (loaded, expected) in loaded_palette.iter().zip(expected_palette.iter()) {
    assert_eq!(loaded, expected);
  }
}

#[test]
fn reads_pattern_info() {
  let loaded_pattern_info = read_pattern_info(&mut load_fixture("pattern_info")).unwrap();
  let expected_pattern_info = PatternInfo {
    title: String::from("Embroidery Studio Demo"),
    author: String::from("Nazar Antoniuk"),
    company: String::from("Embroidery Studio"),
    copyright: String::from("Embroidery Studio"),
    description: String::from("Shows different stitch types"),
  };
  assert_eq!(loaded_pattern_info, expected_pattern_info);
}

#[test]
fn reproduces_decoding_values() {
  let xsd_random_numbers = [498347506, 626547637, 1679951037, 2146703145];
  let (decoding_key, decoding_values) = reproduce_decoding_values(&xsd_random_numbers).unwrap();
  assert_eq!(decoding_key, -228908503);
  assert_eq!(
    decoding_values,
    [18, 25, 28, 30, 21, 26, 13, 22, 29, 30, 15, 23, 9, 20, 10, 5]
  );
}

#[test]
fn reads_stitches() {
  let (loaded_fullstitches, loaded_partstitches) =
    read_stitches(&mut load_fixture("stitches"), 10, 10 * 10, 8).unwrap();
  let expected_fullstitches = [
    FullStitch {
      x: NotNan::new(0.0).unwrap(),
      y: NotNan::new(0.0).unwrap(),
      palindex: 1,
      kind: FullStitchKind::Full,
    },
    FullStitch {
      x: NotNan::new(9.0).unwrap(),
      y: NotNan::new(0.0).unwrap(),
      palindex: 2,
      kind: FullStitchKind::Full,
    },
    FullStitch {
      x: NotNan::new(1.0).unwrap(),
      y: NotNan::new(1.0).unwrap(),
      palindex: 3,
      kind: FullStitchKind::Petite,
    },
    FullStitch {
      x: NotNan::new(2.5).unwrap(),
      y: NotNan::new(1.0).unwrap(),
      palindex: 3,
      kind: FullStitchKind::Petite,
    },
    FullStitch {
      x: NotNan::new(1.0).unwrap(),
      y: NotNan::new(2.5).unwrap(),
      palindex: 3,
      kind: FullStitchKind::Petite,
    },
    FullStitch {
      x: NotNan::new(2.5).unwrap(),
      y: NotNan::new(2.5).unwrap(),
      palindex: 3,
      kind: FullStitchKind::Petite,
    },
    FullStitch {
      x: NotNan::new(0.0).unwrap(),
      y: NotNan::new(9.0).unwrap(),
      palindex: 6,
      kind: FullStitchKind::Full,
    },
    FullStitch {
      x: NotNan::new(9.0).unwrap(),
      y: NotNan::new(9.0).unwrap(),
      palindex: 0,
      kind: FullStitchKind::Full,
    },
  ];
  for (loaded, expected) in loaded_fullstitches.iter().zip(expected_fullstitches.iter()) {
    assert_eq!(loaded, expected);
  }

  let expected_partstitches = [
    PartStitch {
      x: NotNan::new(1.5).unwrap(),
      y: NotNan::new(1.5).unwrap(),
      palindex: 4,
      direction: PartStitchDirection::Backward,
      kind: PartStitchKind::Quarter,
    },
    PartStitch {
      x: NotNan::new(2.0).unwrap(),
      y: NotNan::new(1.5).unwrap(),
      palindex: 4,
      direction: PartStitchDirection::Forward,
      kind: PartStitchKind::Quarter,
    },
    PartStitch {
      x: NotNan::new(1.5).unwrap(),
      y: NotNan::new(2.0).unwrap(),
      palindex: 4,
      direction: PartStitchDirection::Forward,
      kind: PartStitchKind::Quarter,
    },
    PartStitch {
      x: NotNan::new(2.0).unwrap(),
      y: NotNan::new(2.0).unwrap(),
      palindex: 4,
      direction: PartStitchDirection::Backward,
      kind: PartStitchKind::Quarter,
    },
    PartStitch {
      x: NotNan::new(3.0).unwrap(),
      y: NotNan::new(3.0).unwrap(),
      palindex: 5,
      direction: PartStitchDirection::Backward,
      kind: PartStitchKind::Half,
    },
    PartStitch {
      x: NotNan::new(4.0).unwrap(),
      y: NotNan::new(3.0).unwrap(),
      palindex: 5,
      direction: PartStitchDirection::Forward,
      kind: PartStitchKind::Half,
    },
    PartStitch {
      x: NotNan::new(3.0).unwrap(),
      y: NotNan::new(4.0).unwrap(),
      palindex: 5,
      direction: PartStitchDirection::Forward,
      kind: PartStitchKind::Half,
    },
    PartStitch {
      x: NotNan::new(4.0).unwrap(),
      y: NotNan::new(4.0).unwrap(),
      palindex: 5,
      direction: PartStitchDirection::Backward,
      kind: PartStitchKind::Half,
    },
  ];
  for (loaded, expected) in loaded_partstitches.iter().zip(expected_partstitches.iter()) {
    assert_eq!(loaded, expected);
  }
}

#[test]
fn reads_joints() {
  let (loaded_nodes, loaded_lines, ..) = read_joints(&mut load_fixture("joints"), 8).unwrap();
  let expected_nodes = [
    Node {
      x: NotNan::new(3.0).unwrap(),
      y: NotNan::new(3.0).unwrap(),
      rotated: false,
      palindex: 2,
      kind: NodeKind::FrenchKnot,
    },
    Node {
      x: NotNan::new(3.0).unwrap(),
      y: NotNan::new(4.5).unwrap(),
      rotated: false,
      palindex: 3,
      kind: NodeKind::Bead,
    },
    Node {
      x: NotNan::new(3.0).unwrap(),
      y: NotNan::new(5.5).unwrap(),
      rotated: true,
      palindex: 3,
      kind: NodeKind::Bead,
    },
  ];
  for (loaded, expected) in loaded_nodes.iter().zip(expected_nodes.iter()) {
    assert_eq!(loaded, expected);
  }

  let expected_lines = [
    Line {
      x: (NotNan::new(1.0).unwrap(), NotNan::new(2.0).unwrap()),
      y: (NotNan::new(1.0).unwrap(), NotNan::new(1.0).unwrap()),
      palindex: 1,
      kind: LineKind::Back,
    },
    Line {
      x: (NotNan::new(2.0).unwrap(), NotNan::new(3.0).unwrap()),
      y: (NotNan::new(1.0).unwrap(), NotNan::new(2.0).unwrap()),
      palindex: 1,
      kind: LineKind::Back,
    },
    Line {
      x: (NotNan::new(3.0).unwrap(), NotNan::new(4.0).unwrap()),
      y: (NotNan::new(2.0).unwrap(), NotNan::new(1.0).unwrap()),
      palindex: 1,
      kind: LineKind::Back,
    },
    Line {
      x: (NotNan::new(4.0).unwrap(), NotNan::new(5.0).unwrap()),
      y: (NotNan::new(1.0).unwrap(), NotNan::new(1.0).unwrap()),
      palindex: 1,
      kind: LineKind::Back,
    },
    Line {
      x: (NotNan::new(1.0).unwrap(), NotNan::new(5.0).unwrap()),
      y: (NotNan::new(2.0).unwrap(), NotNan::new(2.0).unwrap()),
      palindex: 0,
      kind: LineKind::Straight,
    },
  ];
  for (loaded, expected) in loaded_lines.iter().zip(expected_lines.iter()) {
    assert_eq!(loaded, expected);
  }
}

#[test]
fn parses_xsd_pattern() {
  let file_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/patterns/piggies.xsd");
  let pattern = parse_pattern(file_path).unwrap().pattern;

  assert_eq!(pattern.properties, PatternProperties { width: 69, height: 73 });

  assert_eq!(
    pattern.info,
    PatternInfo {
      title: String::from("Piggies"),
      author: String::from(""),
      company: String::from(""),
      copyright: String::from("by Ursa Software"),
      description: String::from(""),
    }
  );

  assert_eq!(pattern.palette.len(), 8);
  assert_eq!(
    pattern.palette[0],
    PaletteItem {
      brand: String::from("DMC"),
      number: String::from("943"),
      name: String::from("Bright Green-MD"),
      color: String::from("1B997F"),
      blends: None,
      bead: None,
      strands: StitchStrands::default()
    }
  );
  assert_eq!(
    pattern.palette[7],
    PaletteItem {
      brand: String::from("Mill Hill Glass Seed Bead"),
      number: String::from("00968"),
      name: String::from("Red"),
      color: String::from("C74761"),
      blends: None,
      bead: Some(Bead {
        length: NotNan::new(1.5).unwrap(),
        diameter: NotNan::new(2.5).unwrap()
      }),
      strands: StitchStrands::default()
    }
  );

  assert_eq!(
    pattern.fabric,
    Fabric {
      spi: (14, 14),
      kind: String::from("Aida"),
      name: String::from("White"),
      color: String::from("FFFFFF"),
    }
  );

  assert_eq!(pattern.fullstitches.len(), 1000);
  assert_eq!(pattern.partstitches.len(), 54);
  assert_eq!(pattern.nodes.len(), 18);
  assert_eq!(pattern.lines.len(), 446);
}

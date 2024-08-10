use crate::pattern::xsd::*;

#[cfg(test)]
mod xsd_read_tests {
  use super::*;

  #[test]
  fn reads_cstring() {
    let utf8_buf = vec![0x57, 0x68, 0x69, 0x74, 0x65, 0x00, 0x00, 0x00];
    assert_eq!(
      Cursor::new(utf8_buf).read_cstring(8).unwrap(),
      String::from("White")
    );

    let cp1251_buf = vec![0xE3, 0xEE, 0xEB, 0xF3, 0xE1, 0xEE, 0xE9, 0x00];
    assert_eq!(
      Cursor::new(cp1251_buf).read_cstring(8).unwrap(),
      String::from("голубой")
    );
  }

  #[test]
  fn returns_empty_string_on_non_null_terminated_cstring() {
    let not_nul_terminated_buf = vec![0x43, 0x6F, 0x66, 0x66, 0x65, 0x65];
    assert_eq!(
      Cursor::new(not_nul_terminated_buf).read_cstring(6).unwrap(),
      String::from("")
    );
  }

  #[test]
  fn reads_hex_color() {
    let black_color_buf = vec![0x00, 0x00, 0x00];
    assert_eq!(
      Cursor::new(black_color_buf).read_hex_color().unwrap(),
      String::from("000000")
    );

    let white_color_buf = vec![0xff, 0xff, 0xff];
    assert_eq!(
      Cursor::new(white_color_buf).read_hex_color().unwrap(),
      String::from("FFFFFF")
    );
  }

  #[test]
  fn reads_fractional_coors() {
    assert_eq!(
      Cursor::new(vec![0x08, 0x00, 0x09, 0x00])
        .read_fractional_coors()
        .unwrap(),
      (4.0, 4.5)
    );
  }
}

fn load_fixture(name: &str) -> Cursor {
  let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("tests/fixtures/xsd")
    .join(name);
  let buf = std::fs::read(path).unwrap();
  Cursor::new(buf)
}

#[test]
fn reads_signature() {
  let buf: Vec<u8> = vec![0x10, 0x05];
  assert_eq!(
    read_signature(&mut Cursor::new(buf)).unwrap(),
    XSD_VALID_SIGNATURE
  );

  let buf = vec![0x00, 0x00];
  assert_ne!(
    read_signature(&mut Cursor::new(buf)).unwrap(),
    XSD_VALID_SIGNATURE
  );
}

#[test]
fn reads_pattern_properties() {
  let buf = vec![
    0x64, 0x00, 0x64, 0x00, 0xA6, 0x01, 0x00, 0x00, 0x89, 0x03, 0x0E, 0x00, 0x0E, 0x00, 0xFF, 0xFF,
    0xFF, 0xFF, 0x00, 0x16, 0x4B, 0x00,
  ];
  let loaded = read_pattern_properties(&mut Cursor::new(buf)).unwrap();
  let expected = XsdPatternProperties {
    width: 100,
    height: 100,
    small_stitches_count: 422,
    joints_count: 905,
    stitches_per_inch: (14, 14),
    palette_size: 75,
  };
  assert_eq!(loaded, expected);
}

#[test]
fn reads_palette() {
  let mut cursor = load_fixture("test_palette");
  let loaded_palette = read_palette(&mut cursor, 4).unwrap();
  let expected_palette = vec![
    PaletteItem {
      vendor_id: 0,
      number: String::from("310"),
      name: String::from("Black"),
      color: String::from("2C3225"),
      blends: vec![],
    },
    PaletteItem {
      vendor_id: 143,
      number: String::from("9224"),
      name: String::from("ПНК Кирова"),
      color: String::from("B40032"),
      blends: vec![],
    },
    PaletteItem {
      vendor_id: 203,
      number: String::from("62038"),
      name: String::from("Frosted Aquamarine"),
      color: String::from("A6D3D9"),
      blends: vec![],
    },
    PaletteItem {
      vendor_id: 252,
      number: String::from("57"),
      name: String::from(""),
      color: String::from("93D0D3"),
      blends: vec![
        Blend {
          vendor_id: 0,
          number: String::from("964"),
          strands: 1,
        },
        Blend {
          vendor_id: 0,
          number: String::from("3766"),
          strands: 1,
        },
      ],
    },
  ];
  for (loaded, expected) in loaded_palette.iter().zip(expected_palette.iter()) {
    assert_eq!(loaded, expected);
  }
}

#[test]
fn reads_blend() {
  let buf = vec![
    0x00, 0x33, 0x31, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  ];
  let loaded = read_blend_item(&mut Cursor::new(buf)).unwrap();
  let expected = Blend {
    vendor_id: 0,
    number: String::from("310"),
    strands: 0,
  };
  assert_eq!(loaded, expected);
}

#[test]
fn reads_blends_strands() {
  let buf = vec![0x01, 0x02, 0x00, 0x00];
  let mut blends = vec![
    Blend {
      vendor_id: 0,
      number: String::from("000"),
      strands: 0,
    },
    Blend {
      vendor_id: 0,
      number: String::from("000"),
      strands: 0,
    },
  ];
  read_blend_strands(&mut Cursor::new(buf), &mut blends).unwrap();
  assert_eq!(blends[0].strands, 1);
  assert_eq!(blends[1].strands, 2);
}

#[test]
fn reads_fabric_info() {
  let mut cursor = load_fixture("test_fabric_info");
  let loaded = read_fabric_info(&mut cursor).unwrap();
  let expected = XsdFabric {
    name: String::from("White"),
    color: String::from("FFFFFF"),
  };
  assert_eq!(loaded, expected);
}

#[test]
fn reads_pattern_info() {
  let mut cursor = load_fixture("test_pattern_info");
  let (loaded_pattern_info, loaded_fabric_kind) = read_pattern_info(&mut cursor).unwrap();
  let expected_pattern_info = PatternInfo {
    title: String::from("Embroidery Studio Demo"),
    author: String::from("Nazar Antoniuk"),
    copyright: String::from("Embroidery Studio"),
    description: String::from("Shows different stitch types"),
  };
  assert_eq!(loaded_pattern_info, expected_pattern_info);
  assert_eq!(loaded_fabric_kind, String::from("Aida"));
}

#[test]
fn reproduces_decoding_values() {
  let xsd_random_numbers: XsdRandomNumbers = [498347506, 626547637, 1679951037, 2146703145];
  let (decoding_key, decoding_values) = reproduce_decoding_values(&xsd_random_numbers).unwrap();
  assert_eq!(decoding_key, -228908503);
  assert_eq!(
    decoding_values,
    [18, 25, 28, 30, 21, 26, 13, 22, 29, 30, 15, 23, 9, 20, 10, 5]
  );
}

#[test]
fn reads_stitches() {
  let mut cursor = load_fixture("test_stitches_data");
  let pattern_properties = XsdPatternProperties {
    width: 10,
    height: 10,
    small_stitches_count: 8,
    joints_count: 0,
    stitches_per_inch: (14, 14),
    palette_size: 7,
  };
  let (loaded_fullstitches, loaded_partstitches) =
    read_stitches(&mut cursor, &pattern_properties).unwrap();
  let expected_fullstitches = vec![
    FullStitch {
      x: 0.0,
      y: 0.0,
      palindex: 1,
      kind: FullStitchKind::Full,
    },
    FullStitch {
      x: 9.0,
      y: 0.0,
      palindex: 2,
      kind: FullStitchKind::Full,
    },
    FullStitch {
      x: 1.0,
      y: 1.0,
      palindex: 3,
      kind: FullStitchKind::Petite,
    },
    FullStitch {
      x: 2.5,
      y: 1.0,
      palindex: 3,
      kind: FullStitchKind::Petite,
    },
    FullStitch {
      x: 1.0,
      y: 2.5,
      palindex: 3,
      kind: FullStitchKind::Petite,
    },
    FullStitch {
      x: 2.5,
      y: 2.5,
      palindex: 3,
      kind: FullStitchKind::Petite,
    },
    FullStitch {
      x: 0.0,
      y: 9.0,
      palindex: 6,
      kind: FullStitchKind::Full,
    },
    FullStitch {
      x: 9.0,
      y: 9.0,
      palindex: 0,
      kind: FullStitchKind::Full,
    },
  ];
  for (loaded, expected) in loaded_fullstitches.iter().zip(expected_fullstitches.iter()) {
    assert_eq!(loaded, expected);
  }

  let expected_partstitches = vec![
    PartStitch {
      x: 1.5,
      y: 1.5,
      palindex: 4,
      direction: PartStitchDirection::Backward,
      kind: PartStitchKind::Quarter,
    },
    PartStitch {
      x: 2.0,
      y: 1.5,
      palindex: 4,
      direction: PartStitchDirection::Forward,
      kind: PartStitchKind::Quarter,
    },
    PartStitch {
      x: 1.5,
      y: 2.0,
      palindex: 4,
      direction: PartStitchDirection::Forward,
      kind: PartStitchKind::Quarter,
    },
    PartStitch {
      x: 2.0,
      y: 2.0,
      palindex: 4,
      direction: PartStitchDirection::Backward,
      kind: PartStitchKind::Quarter,
    },
    PartStitch {
      x: 3.0,
      y: 3.0,
      palindex: 5,
      direction: PartStitchDirection::Backward,
      kind: PartStitchKind::Half,
    },
    PartStitch {
      x: 4.0,
      y: 3.0,
      palindex: 5,
      direction: PartStitchDirection::Forward,
      kind: PartStitchKind::Half,
    },
    PartStitch {
      x: 3.0,
      y: 4.0,
      palindex: 5,
      direction: PartStitchDirection::Forward,
      kind: PartStitchKind::Half,
    },
    PartStitch {
      x: 4.0,
      y: 4.0,
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
  let joints_count = 8;
  let mut cursor = load_fixture("test_joints_data");
  let (loaded_nodes, loaded_lines) = read_joints(&mut cursor, joints_count).unwrap();
  let expected_nodes = vec![
    Node {
      x: 3.0,
      y: 3.0,
      rotated: false,
      palindex: 2,
      kind: NodeKind::FrenchKnot,
    },
    Node {
      x: 3.0,
      y: 4.5,
      rotated: false,
      palindex: 3,
      kind: NodeKind::Bead,
    },
    Node {
      x: 3.0,
      y: 5.5,
      rotated: true,
      palindex: 3,
      kind: NodeKind::Bead,
    },
  ];
  for (loaded, expected) in loaded_nodes.iter().zip(expected_nodes.iter()) {
    assert_eq!(loaded, expected);
  }

  let expected_lines = vec![
    Line {
      x: (1.0, 2.0),
      y: (1.0, 1.0),
      palindex: 1,
      kind: LineKind::Back,
    },
    Line {
      x: (2.0, 3.0),
      y: (1.0, 2.0),
      palindex: 1,
      kind: LineKind::Back,
    },
    Line {
      x: (3.0, 4.0),
      y: (2.0, 1.0),
      palindex: 1,
      kind: LineKind::Back,
    },
    Line {
      x: (4.0, 5.0),
      y: (1.0, 1.0),
      palindex: 1,
      kind: LineKind::Back,
    },
    Line {
      x: (1.0, 5.0),
      y: (2.0, 2.0),
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
  let pathbuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/patterns/piggies.xsd");
  let pattern = parse_pattern(pathbuf.as_path());
  assert!(pattern.is_ok());

  let pattern = pattern.unwrap();

  assert_eq!(
    pattern.properties,
    PatternProperties {
      width: 69,
      height: 73,
    }
  );

  assert_eq!(
    pattern.info,
    PatternInfo {
      title: String::from("Piggies"),
      author: String::from(""),
      copyright: String::from("by Ursa Software"),
      description: String::from(""),
    }
  );

  assert_eq!(pattern.palette.len(), 8);
  assert_eq!(
    pattern.palette[0],
    PaletteItem {
      vendor_id: 0,
      number: String::from("943"),
      name: String::from("Bright Green-MD"),
      color: String::from("1B997F"),
      blends: vec![],
    }
  );
  assert_eq!(
    pattern.palette[7],
    PaletteItem {
      vendor_id: 200,
      number: String::from("00968"),
      name: String::from("Red"),
      color: String::from("C74761"),
      blends: vec![],
    }
  );

  assert_eq!(
    pattern.fabric,
    Fabric {
      stitches_per_inch: (14, 14),
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

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
  assert_eq!(
    read_signature(&mut Cursor::new(vec![0x10, 0x05])).unwrap(),
    VALID_SIGNATURE
  );

  assert_ne!(
    read_signature(&mut Cursor::new(vec![0x00, 0x00])).unwrap(),
    VALID_SIGNATURE
  );
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
      strands: Some(StitchStrands::default()),
    },
    PaletteItem {
      brand: String::from("PNK Kirova"),
      number: String::from("9224"),
      name: String::from("ПНК Кирова"),
      color: String::from("B40032"),
      blends: None,
      bead: None,
      strands: Some(StitchStrands::default()),
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
      strands: Some(StitchStrands::default()),
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
      strands: Some(StitchStrands {
        full: Some(2),
        petite: Some(2),
        half: Some(2),
        quarter: Some(2),
        back: Some(2),
        straight: Some(2),
        french_knot: Some(2),
        special: Some(2),
      }),
    },
  ];
  for (loaded, expected) in loaded_palette.iter().zip(expected_palette.iter()) {
    assert_eq!(loaded, expected);
  }
}

#[test]
fn reads_formats() {
  let loaded_formats = read_formats(&mut load_fixture("formats"), 2).unwrap();
  let expected_formats = vec![
    Formats {
      symbol: SymbolFormat {
        use_alt_bg_color: false,
        bg_color: String::from("FFFFFF"),
        fg_color: String::from("000000"),
      },
      back: LineFormat {
        use_alt_color: false,
        color: String::from("000000"),
        style: LineStyle::Solid,
        thickness: NotNan::new(1.0).unwrap(),
      },
      straight: LineFormat {
        use_alt_color: false,
        color: String::from("000000"),
        style: LineStyle::Solid,
        thickness: NotNan::new(1.0).unwrap(),
      },
      french: NodeFormat {
        use_dot_style: true,
        use_alt_color: false,
        color: String::from("000000"),
        diameter: NotNan::new(4.0).unwrap(),
      },
      bead: NodeFormat {
        use_dot_style: true,
        use_alt_color: false,
        color: String::from("000000"),
        diameter: NotNan::new(4.0).unwrap(),
      },
      special: LineFormat {
        use_alt_color: false,
        color: String::from("000000"),
        style: LineStyle::Solid,
        thickness: NotNan::new(1.0).unwrap(),
      },
      font: FontFormat {
        font_name: Some(String::from("CrossStitch3")),
        bold: false,
        italic: false,
        stitch_size: 100,
        small_stitch_size: 60,
      },
    },
    Formats {
      symbol: SymbolFormat {
        use_alt_bg_color: false,
        bg_color: String::from("FFFFFF"),
        fg_color: String::from("000000"),
      },
      back: LineFormat {
        use_alt_color: false,
        color: String::from("FFFFFF"),
        style: LineStyle::Dashed,
        thickness: NotNan::new(1.5).unwrap(),
      },
      straight: LineFormat {
        use_alt_color: false,
        color: String::from("FFFFFF"),
        style: LineStyle::Dotted,
        thickness: NotNan::new(0.8).unwrap(),
      },
      french: NodeFormat {
        use_dot_style: false,
        use_alt_color: false,
        color: String::from("FFFFFF"),
        diameter: NotNan::new(4.0).unwrap(),
      },
      bead: NodeFormat {
        use_dot_style: false,
        use_alt_color: false,
        color: String::from("FFFFFF"),
        diameter: NotNan::new(4.0).unwrap(),
      },
      special: LineFormat {
        use_alt_color: false,
        color: String::from("FFFFFF"),
        style: LineStyle::Solid,
        thickness: NotNan::new(1.5).unwrap(),
      },
      font: FontFormat {
        font_name: Some(String::from("CrossStitch3")),
        bold: false,
        italic: false,
        stitch_size: 80,
        small_stitch_size: 50,
      },
    },
  ];
  for (loaded, expected) in loaded_formats.iter().zip(expected_formats.iter()) {
    assert_eq!(loaded, expected);
  }
}

#[test]
fn reads_symbols() {
  let loaded_symbols = read_symbols(&mut load_fixture("symbols"), 2).unwrap();
  let expected_symbols = vec![
    Symbols {
      full: Some(33),
      petite: Some(34),
      half: Some(35),
      quarter: Some(36),
      french_knot: Some(37),
      bead: Some(38),
    },
    Symbols {
      full: Some(164),
      petite: None,
      half: None,
      quarter: None,
      french_knot: None,
      bead: None,
    },
  ];
  for (loaded, expected) in loaded_symbols.iter().zip(expected_symbols.iter()) {
    assert_eq!(loaded, expected);
  }
}

#[test]
fn reads_pattern_settings() {
  assert_eq!(
    read_pattern_settings(&mut load_fixture("pattern_settings")).unwrap(),
    XsdPatternSettings {
      stitch_font_name: String::from("CrossStitch3"),
      font: Font {
        name: String::from("Courier New"),
        size: 10,
        weight: 400,
        italic: false,
      },
      view: View::Solid,
      zoom: 400,
      show_grid: true,
      show_rulers: true,
      show_centering_marks: false,
      show_fabric_colors_with_symbols: false,
      gaps_between_stitches: false,
      page_header: String::from("&l&t &r&n"),
      page_footer: String::from(""),
      page_margins: PageMargins {
        left: NotNan::new(0.5).unwrap(),
        right: NotNan::new(0.5).unwrap(),
        top: NotNan::new(0.5).unwrap(),
        bottom: NotNan::new(0.5).unwrap(),
        header: NotNan::new(0.5).unwrap(),
        footer: NotNan::new(0.5).unwrap(),
      },
      show_page_numbers: true,
      show_adjacent_page_numbers: true,
      center_chart_on_pages: false,
    }
  );
}

#[test]
fn reads_grid_settings() {
  assert_eq!(
    read_grid_settings(&mut load_fixture("grid_settings")).unwrap(),
    Grid::default()
  );
}

#[test]
fn reads_pattern_info() {
  assert_eq!(
    read_pattern_info(&mut load_fixture("pattern_info")).unwrap(),
    PatternInfo {
      title: String::from("Embroidery Studio Demo"),
      author: String::from("Nazar Antoniuk"),
      company: String::from("Embroidery Studio"),
      copyright: String::from("Embroidery Studio"),
      description: String::from("Shows different stitch types"),
    }
  );
}

#[test]
fn reads_stitch_settings() {
  let (stitch_settings, outlined_stitches, stitch_outline) =
    read_stitch_settings(&mut load_fixture("stitch_settings")).unwrap();
  assert_eq!(stitch_settings, StitchSettings::default());
  assert_eq!(outlined_stitches, true);
  assert_eq!(stitch_outline, StitchOutline::default());
}

#[test]
fn reads_symbol_settings() {
  assert_eq!(
    read_symbol_settings(&mut load_fixture("symbol_settings")).unwrap(),
    SymbolSettings::default()
  );
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
fn reads_special_stitch_models() {
  let loaded_special_stitch_models = read_special_stitch_models(&mut load_fixture("special_stitch_models")).unwrap();
  let expected_speciql_stitch_models = vec![
    SpecialStitchModel {
      unique_name: String::from("Lasy Daisy Over 2x1"),
      name: String::from(""),
      nodes: vec![],
      lines: vec![],
      curves: vec![Curve {
        points: vec![
          (NotNan::new(1.5666666).unwrap(), NotNan::new(2.0666666).unwrap()),
          (NotNan::new(0.6).unwrap(), NotNan::new(0.8333333).unwrap()),
          (NotNan::new(0.6333333).unwrap(), NotNan::new(0.23333333).unwrap()),
          (NotNan::new(0.79999995).unwrap(), NotNan::new(0.06666667).unwrap()),
          (NotNan::new(1.1333333).unwrap(), NotNan::new(0.2).unwrap()),
          (NotNan::new(1.3666667).unwrap(), NotNan::new(0.56666666).unwrap()),
          (NotNan::new(1.5666666).unwrap(), NotNan::new(2.0666666).unwrap()),
        ],
      }],
    },
    SpecialStitchModel {
      unique_name: String::from("Rhodes Heart - over 6"),
      name: String::from("Rhodes Heart"),
      nodes: vec![],
      lines: vec![
        Line {
          x: (NotNan::new(1.0).unwrap(), NotNan::new(2.0).unwrap()),
          y: (NotNan::new(2.0).unwrap(), NotNan::new(0.0).unwrap()),
          palindex: 0,
          kind: LineKind::Straight,
        },
        Line {
          x: (NotNan::new(0.5).unwrap(), NotNan::new(2.5).unwrap()),
          y: (NotNan::new(1.5).unwrap(), NotNan::new(0.0).unwrap()),
          palindex: 0,
          kind: LineKind::Straight,
        },
        Line {
          x: (NotNan::new(0.0).unwrap(), NotNan::new(3.0).unwrap()),
          y: (NotNan::new(1.0).unwrap(), NotNan::new(0.5).unwrap()),
          palindex: 0,
          kind: LineKind::Straight,
        },
        Line {
          x: (NotNan::new(0.0).unwrap(), NotNan::new(3.0).unwrap()),
          y: (NotNan::new(0.5).unwrap(), NotNan::new(1.0).unwrap()),
          palindex: 0,
          kind: LineKind::Straight,
        },
        Line {
          x: (NotNan::new(0.5).unwrap(), NotNan::new(2.5).unwrap()),
          y: (NotNan::new(0.0).unwrap(), NotNan::new(1.5).unwrap()),
          palindex: 0,
          kind: LineKind::Straight,
        },
        Line {
          x: (NotNan::new(1.0).unwrap(), NotNan::new(2.0).unwrap()),
          y: (NotNan::new(0.0).unwrap(), NotNan::new(2.0).unwrap()),
          palindex: 0,
          kind: LineKind::Straight,
        },
        Line {
          x: (NotNan::new(1.5).unwrap(), NotNan::new(1.5).unwrap()),
          y: (NotNan::new(0.5).unwrap(), NotNan::new(2.5).unwrap()),
          palindex: 0,
          kind: LineKind::Straight,
        },
      ],
      curves: vec![],
    },
  ];
  for (loaded, expected) in loaded_special_stitch_models
    .iter()
    .zip(expected_speciql_stitch_models.iter())
  {
    assert_eq!(loaded, expected);
  }
}

#[test]
fn reads_joints() {
  let (loaded_nodes, loaded_lines, _, loaded_special_stitches) = read_joints(&mut load_fixture("joints"), 16).unwrap();

  let expected_nodes = [
    Node {
      x: NotNan::new(3.0).unwrap(),
      y: NotNan::new(3.0).unwrap(),
      rotated: false,
      palindex: 0,
      kind: NodeKind::FrenchKnot,
    },
    Node {
      x: NotNan::new(3.0).unwrap(),
      y: NotNan::new(4.5).unwrap(),
      rotated: false,
      palindex: 2,
      kind: NodeKind::Bead,
    },
    Node {
      x: NotNan::new(3.0).unwrap(),
      y: NotNan::new(5.5).unwrap(),
      rotated: true,
      palindex: 2,
      kind: NodeKind::Bead,
    },
  ];
  for (loaded, expected) in loaded_nodes.iter().zip(expected_nodes.iter()) {
    assert_eq!(loaded, expected);
  }

  let expected_lines = [
    Line {
      x: (NotNan::new(1.0).unwrap(), NotNan::new(2.0).unwrap()),
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
      palindex: 1,
      kind: LineKind::Straight,
    },
  ];
  for (loaded, expected) in loaded_lines.iter().zip(expected_lines.iter()) {
    assert_eq!(loaded, expected);
  }

  let expected_special_stitches = [
    SpecialStitch {
      x: NotNan::new(5.5).unwrap(),
      y: NotNan::new(1.0).unwrap(),
      rotation: 0,
      flip: (false, false),
      palindex: 0,
      modindex: 0,
    },
    SpecialStitch {
      x: NotNan::new(9.0).unwrap(),
      y: NotNan::new(1.0).unwrap(),
      rotation: 0,
      flip: (true, false),
      palindex: 0,
      modindex: 0,
    },
    SpecialStitch {
      x: NotNan::new(8.5).unwrap(),
      y: NotNan::new(3.0).unwrap(),
      rotation: 0,
      flip: (false, true),
      palindex: 0,
      modindex: 0,
    },
    SpecialStitch {
      x: NotNan::new(12.0).unwrap(),
      y: NotNan::new(3.0).unwrap(),
      rotation: 0,
      flip: (true, true),
      palindex: 0,
      modindex: 0,
    },
    SpecialStitch {
      x: NotNan::new(9.0).unwrap(),
      y: NotNan::new(4.5).unwrap(),
      rotation: 90,
      flip: (false, false),
      palindex: 0,
      modindex: 0,
    },
    SpecialStitch {
      x: NotNan::new(9.0).unwrap(),
      y: NotNan::new(5.5).unwrap(),
      rotation: 270,
      flip: (false, false),
      palindex: 0,
      modindex: 0,
    },
    SpecialStitch {
      x: NotNan::new(9.0).unwrap(),
      y: NotNan::new(6.5).unwrap(),
      rotation: 90,
      flip: (false, true),
      palindex: 0,
      modindex: 0,
    },
    SpecialStitch {
      x: NotNan::new(9.0).unwrap(),
      y: NotNan::new(8.0).unwrap(),
      rotation: 90,
      flip: (true, false),
      palindex: 0,
      modindex: 0,
    },
    SpecialStitch {
      x: NotNan::new(11.0).unwrap(),
      y: NotNan::new(5.0).unwrap(),
      rotation: 0,
      flip: (false, false),
      palindex: 1,
      modindex: 1,
    },
  ];
  for (loaded, expected) in loaded_special_stitches.iter().zip(expected_special_stitches.iter()) {
    assert_eq!(loaded, expected);
  }
}

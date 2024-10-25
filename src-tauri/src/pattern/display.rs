use borsh::{BorshDeserialize, BorshSerialize};
use ordered_float::NotNan;

pub type Points = NotNan<f32>;

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct DisplaySettings {
  pub default_stitch_font: String,
  pub symbols: Vec<Symbols>,
  pub symbol_settings: SymbolSettings,
  pub formats: Vec<Formats>,
  pub grid: Grid,
  pub view: View,
  pub zoom: u16,
  pub show_grid: bool,
  pub show_rulers: bool,
  pub show_centering_marks: bool,
  pub show_fabric_colors_with_symbols: bool,
  pub gaps_between_stitches: bool,
  pub outlined_stitches: bool,
  pub stitch_outline: StitchOutline,
  pub stitch_settings: StitchSettings,
}

impl DisplaySettings {
  pub fn new(palette_size: usize) -> Self {
    Self {
      default_stitch_font: String::from("CrossStitch3"),
      symbols: vec![Symbols::default(); palette_size],
      symbol_settings: SymbolSettings::default(),
      formats: vec![Formats::default(); palette_size],
      grid: Grid::default(),
      view: View::Solid,
      zoom: 100,
      show_grid: true,
      show_rulers: true,
      show_centering_marks: true,
      show_fabric_colors_with_symbols: false,
      gaps_between_stitches: false,
      outlined_stitches: true,
      stitch_outline: StitchOutline::default(),
      stitch_settings: StitchSettings::default(),
    }
  }
}

#[derive(Debug, Default, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Symbols {
  pub full: Option<u16>,
  pub petite: Option<u16>,
  pub half: Option<u16>,
  pub quarter: Option<u16>,
  pub french_knot: Option<u16>,
  pub bead: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SymbolSettings {
  pub screen_spacing: (u16, u16),
  pub printer_spacing: (u16, u16),
  pub scale_using_maximum_font_width: bool,
  pub scale_using_font_height: bool,
  pub stitch_size: u16,
  pub small_stitch_size: u16,
  pub draw_symbols_over_backstitches: bool,
  pub show_stitch_color: bool,
  pub use_large_half_stitch_symbol: bool,
  pub use_triangles_behind_quarter_stitches: bool,
}

impl Default for SymbolSettings {
  fn default() -> Self {
    Self {
      screen_spacing: (1, 1),
      printer_spacing: (1, 1),
      scale_using_maximum_font_width: true,
      scale_using_font_height: true,
      stitch_size: 100,
      small_stitch_size: 60,
      draw_symbols_over_backstitches: false,
      show_stitch_color: false,
      use_large_half_stitch_symbol: false,
      use_triangles_behind_quarter_stitches: false,
    }
  }
}

#[derive(Debug, Default, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Formats {
  pub symbol: SymbolFormat,
  pub back: LineFormat,
  pub straight: LineFormat,
  pub french: NodeFormat,
  pub bead: NodeFormat,
  pub special: LineFormat,
  pub font: FontFormat,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SymbolFormat {
  pub use_alt_bg_color: bool,
  pub bg_color: String,
  pub fg_color: String,
}

impl Default for SymbolFormat {
  fn default() -> Self {
    Self {
      use_alt_bg_color: false,
      bg_color: String::from("FFFFFF"),
      fg_color: String::from("000000"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct LineFormat {
  pub use_alt_color: bool,
  pub color: String,
  pub style: LineStyle,
  pub thickness: Points,
}

impl Default for LineFormat {
  fn default() -> Self {
    Self {
      use_alt_color: false,
      color: String::from("000000"),
      style: LineStyle::Solid,
      thickness: NotNan::new(1.0).unwrap(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
#[borsh(use_discriminant = true)]
pub enum LineStyle {
  Solid = 0,
  Barred = 1,
  Dotted = 2,
  ChainDotted = 3,
  Dashed = 4,
  Outlined = 5,
  Zebra = 6,
  ZigZag = 7,
  Morse = 8,
}

impl From<u16> for LineStyle {
  fn from(value: u16) -> Self {
    match value {
      // These are the values used by Pattern Maker.
      0 | 5 => LineStyle::Solid,
      1 | 7 => LineStyle::Barred,
      2 | 6 => LineStyle::Dotted,
      11 => LineStyle::ChainDotted,
      3 | 8 => LineStyle::Dashed,
      9 => LineStyle::Outlined,
      10 => LineStyle::Zebra,
      12 => LineStyle::ZigZag,
      4 => LineStyle::Morse,
      _ => panic!("Invalid LineStyle value: {value}"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct NodeFormat {
  pub use_dot_style: bool,
  pub use_alt_color: bool,
  pub color: String,
  pub diameter: Points,
}

impl Default for NodeFormat {
  fn default() -> Self {
    Self {
      use_dot_style: true,
      use_alt_color: false,
      color: String::from("000000"),
      diameter: NotNan::new(1.0).unwrap(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct FontFormat {
  pub font_name: Option<String>,
  pub bold: bool,
  pub italic: bool,
  pub stitch_size: u16,
  pub small_stitch_size: u16,
}

impl Default for FontFormat {
  fn default() -> Self {
    Self {
      font_name: None,
      bold: false,
      italic: false,
      stitch_size: 100,
      small_stitch_size: 60,
    }
  }
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Grid {
  pub major_line_every_stitches: u16,
  pub minor_screen_lines: GridLineStyle,
  pub major_screen_lines: GridLineStyle,
  pub minor_printer_lines: GridLineStyle,
  pub major_printer_lines: GridLineStyle,
}

impl Default for Grid {
  fn default() -> Self {
    Self {
      major_line_every_stitches: 10,
      minor_screen_lines: GridLineStyle {
        color: String::from("000000"),
        thickness: NotNan::new(0.072).unwrap(),
      },
      major_screen_lines: GridLineStyle {
        color: String::from("000000"),
        thickness: NotNan::new(0.072).unwrap(),
      },
      minor_printer_lines: GridLineStyle {
        color: String::from("000000"),
        thickness: NotNan::new(0.144).unwrap(),
      },
      major_printer_lines: GridLineStyle {
        color: String::from("000000"),
        thickness: NotNan::new(0.504).unwrap(),
      },
    }
  }
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct GridLineStyle {
  pub color: String,
  pub thickness: Points,
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
#[borsh(use_discriminant = true)]
pub enum View {
  Stitches = 0,
  Symbols = 1,
  Solid = 2,
  Information = 3,
  MachineEmbInfo = 4,
}

impl From<u16> for View {
  fn from(value: u16) -> Self {
    match value {
      0 => View::Stitches,
      1 => View::Symbols,
      2 => View::Solid,
      3 => View::Information,
      5 => View::MachineEmbInfo,
      _ => panic!("Invalid View value: {value}"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct StitchOutline {
  pub color: Option<String>,
  pub color_percentage: u16,
  pub thickness: Points,
}

impl Default for StitchOutline {
  fn default() -> Self {
    Self {
      color: None,
      color_percentage: 80,
      thickness: NotNan::new(0.2).unwrap(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct StitchSettings {
  pub default_strands: DefaultStitchStrands,
  /// 1..=12 - strands, 13 - french knot.
  pub display_thickness: [Points; 13],
}

impl Default for StitchSettings {
  fn default() -> Self {
    Self {
      default_strands: DefaultStitchStrands::default(),
      display_thickness: [
        NotNan::new(1.0).unwrap(), // 1 strand
        NotNan::new(1.5).unwrap(), // 2 strands
        NotNan::new(2.5).unwrap(), // 3 strands
        NotNan::new(3.0).unwrap(), // 4 strands
        NotNan::new(3.5).unwrap(), // 5 strands
        NotNan::new(4.0).unwrap(), // 6 strands
        NotNan::new(4.5).unwrap(), // 7 strands
        NotNan::new(5.0).unwrap(), // 8 strands
        NotNan::new(5.5).unwrap(), // 9 strands
        NotNan::new(6.0).unwrap(), // 10 strands
        NotNan::new(6.5).unwrap(), // 11 strands
        NotNan::new(7.0).unwrap(), // 12 strands
        NotNan::new(4.0).unwrap(), // French knot
      ],
    }
  }
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct DefaultStitchStrands {
  pub full: u16,
  pub petite: u16,
  pub half: u16,
  pub quarter: u16,
  pub back: u16,
  pub straight: u16,
  pub special: u16,
}

impl Default for DefaultStitchStrands {
  fn default() -> Self {
    Self {
      full: 2,
      petite: 2,
      half: 2,
      quarter: 2,
      back: 1,
      straight: 1,
      special: 2,
    }
  }
}

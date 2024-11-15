use borsh::{BorshDeserialize, BorshSerialize};
use ordered_float::NotNan;

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct PrintSettings {
  pub font: Font,
  pub header: String,
  pub footer: String,
  pub margins: PageMargins,
  pub show_page_numbers: bool,
  pub show_adjacent_page_numbers: bool,
  pub center_chart_on_pages: bool,
}

impl Default for PrintSettings {
  fn default() -> Self {
    Self {
      font: Font::default(),
      header: String::new(),
      footer: String::new(),
      margins: PageMargins::default(),
      show_page_numbers: true,
      show_adjacent_page_numbers: true,
      center_chart_on_pages: true,
    }
  }
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Font {
  pub name: String,
  pub size: u16,
  pub weight: u16,
  pub italic: bool,
}

impl Default for Font {
  fn default() -> Self {
    Self {
      name: String::from("Arial"),
      size: 12,
      weight: 400,
      italic: false,
    }
  }
}

pub type Inches = NotNan<f32>;

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct PageMargins {
  pub left: Inches,
  pub right: Inches,
  pub top: Inches,
  pub bottom: Inches,
  pub header: Inches,
  pub footer: Inches,
}

impl Default for PageMargins {
  fn default() -> Self {
    Self {
      left: NotNan::new(0.5).unwrap(),
      right: NotNan::new(0.5).unwrap(),
      top: NotNan::new(0.5).unwrap(),
      bottom: NotNan::new(0.5).unwrap(),
      header: NotNan::new(0.5).unwrap(),
      footer: NotNan::new(0.5).unwrap(),
    }
  }
}

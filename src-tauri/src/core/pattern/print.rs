use borsh::{BorshDeserialize, BorshSerialize};

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
  pub weight: FontWeight,
  pub italic: bool,
}

impl Default for Font {
  fn default() -> Self {
    Self {
      name: String::from("Arial"),
      size: 12,
      weight: FontWeight::new(400),
      italic: false,
    }
  }
}

#[nutype::nutype(
  sanitize(with = |raw| raw.clamp(100, 900)),
  derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)
)]
pub struct FontWeight(u16);

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct PageMargins {
  pub left: f32,
  pub right: f32,
  pub top: f32,
  pub bottom: f32,
  pub header: f32,
  pub footer: f32,
}

impl Default for PageMargins {
  fn default() -> Self {
    Self {
      left: 0.5,
      right: 0.5,
      top: 0.5,
      bottom: 0.5,
      header: 0.5,
      footer: 0.5,
    }
  }
}

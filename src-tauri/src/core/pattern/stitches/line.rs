use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use super::PaletteIndex;
use crate::core::pattern::Coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Line {
  pub x: (Coord, Coord),
  pub y: (Coord, Coord),
  pub palindex: u8,
  pub kind: LineKind,
}

impl PartialOrd for Line {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Line {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.y.cmp(&other.y).then(self.x.cmp(&other.x))
  }
}

impl PaletteIndex for Line {
  fn palindex(&self) -> u8 {
    self.palindex
  }

  fn set_palindex(&mut self, palindex: u8) {
    self.palindex = palindex;
  }
}

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, BorshSerialize, BorshDeserialize,
)]
#[borsh(use_discriminant = true)]
pub enum LineKind {
  Back = 0,
  Straight = 1,
}

impl std::fmt::Display for LineKind {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      LineKind::Back => write!(f, "backstitch"),
      LineKind::Straight => write!(f, "straightstitch"),
    }
  }
}

impl std::str::FromStr for LineKind {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "backstitch" => Ok(LineKind::Back),
      "straightstitch" => Ok(LineKind::Straight),
      _ => Ok(LineKind::Back),
    }
  }
}

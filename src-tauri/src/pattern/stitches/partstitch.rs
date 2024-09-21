use std::cmp::Ordering;

use borsh::{BorshDeserialize, BorshSerialize};

use super::fullstitch::*;
use crate::pattern::Coord;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct PartStitch {
  pub x: Coord,
  pub y: Coord,
  pub palindex: u8,
  pub direction: PartStitchDirection,
  pub kind: PartStitchKind,
}

impl PartialOrd for PartStitch {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for PartStitch {
  fn cmp(&self, other: &Self) -> Ordering {
    self
      .y
      .cmp(&other.y)
      .then(self.x.cmp(&other.x))
      .then(self.kind.cmp(&other.kind))
  }
}

impl From<FullStitch> for PartStitch {
  fn from(fullstitch: FullStitch) -> Self {
    Self {
      x: fullstitch.x,
      y: fullstitch.y,
      palindex: fullstitch.palindex,
      direction: PartStitchDirection::from((fullstitch.x, fullstitch.y)),
      kind: fullstitch.kind.into(),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
#[borsh(use_discriminant = true)]
pub enum PartStitchDirection {
  Forward = 1,
  Backward = 2,
}

impl From<(Coord, Coord)> for PartStitchDirection {
  fn from((x, y): (Coord, Coord)) -> Self {
    if (x.fract() < 0.5 && y.fract() < 0.5) || (x.fract() >= 0.5 && y.fract() >= 0.5) {
      PartStitchDirection::Backward
    } else {
      PartStitchDirection::Forward
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
#[borsh(use_discriminant = true)]
pub enum PartStitchKind {
  Half = 0,
  Quarter = 1,
}

impl From<FullStitchKind> for PartStitchKind {
  fn from(kind: FullStitchKind) -> Self {
    match kind {
      FullStitchKind::Full => PartStitchKind::Half,
      FullStitchKind::Petite => PartStitchKind::Quarter,
    }
  }
}

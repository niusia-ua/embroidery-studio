use std::cmp::Ordering;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use crate::pattern::Coord;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Line {
  pub x: (Coord, Coord),
  pub y: (Coord, Coord),
  pub palindex: u8,
  pub kind: LineKind,
}

impl PartialOrd for Line {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Line {
  fn cmp(&self, other: &Self) -> Ordering {
    self.y.cmp(&other.y).then(self.x.cmp(&other.x))
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

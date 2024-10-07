use std::cmp::Ordering;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pattern::Coord;

#[derive(Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Node {
  pub x: Coord,
  pub y: Coord,
  pub rotated: bool,
  pub palindex: u8,
  pub kind: NodeKind,
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    self.y.cmp(&other.y).then(self.x.cmp(&other.x))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
#[borsh(use_discriminant = true)]
pub enum NodeKind {
  FrenchKnot = 0,
  Bead = 1,
}

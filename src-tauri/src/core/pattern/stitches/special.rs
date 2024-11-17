use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use super::{Line, Node};
use crate::core::pattern::Coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct SpecialStitch {
  pub x: Coord,
  pub y: Coord,
  pub rotation: u16,
  pub flip: (bool, bool),
  pub palindex: u8,
  pub modindex: u16,
}

impl PartialOrd for SpecialStitch {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for SpecialStitch {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.y.cmp(&other.y).then(self.x.cmp(&other.x))
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct SpecialStitchModel {
  pub unique_name: String,
  pub name: String,
  pub nodes: Vec<Node>,
  pub lines: Vec<Line>,
  pub curves: Vec<Curve>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Curve {
  pub points: Vec<(Coord, Coord)>,
}

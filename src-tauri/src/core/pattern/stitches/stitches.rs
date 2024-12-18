use std::cmp::Ordering;
use std::collections::BTreeSet;

use borsh::{BorshDeserialize, BorshSerialize};
use ordered_float::NotNan;
use serde::{Deserialize, Serialize};

use super::*;

#[cfg(test)]
#[path = "./stitches.test.rs"]
mod tests;

pub type Coord = ordered_float::NotNan<f32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stitch {
  Full(FullStitch),
  Part(PartStitch),
  Line(Line),
  Node(Node),
}

impl From<FullStitch> for Stitch {
  fn from(fullstitch: FullStitch) -> Self {
    Self::Full(fullstitch)
  }
}

impl From<PartStitch> for Stitch {
  fn from(partstitch: PartStitch) -> Self {
    Self::Part(partstitch)
  }
}

impl From<Node> for Stitch {
  fn from(node: Node) -> Self {
    Self::Node(node)
  }
}

impl From<Line> for Stitch {
  fn from(line: Line) -> Self {
    Self::Line(line)
  }
}

/// A set of stitches.
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Stitches<T: Ord> {
  inner: BTreeSet<T>,
}

impl<T: Ord> Stitches<T> {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    Self { inner: BTreeSet::new() }
  }

  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.inner.iter()
  }

  #[cfg(test)]
  pub fn len(&self) -> usize {
    self.inner.len()
  }

  /// Returns `true` if the set contains a stitch.
  pub fn contains(&self, stitch: &T) -> bool {
    // We need to use the `get` method to get the actual stitch.
    // Then we need to compare the actual stitch with the passed stitch.
    // This is because the indexing is done only by the fields that are used for ordering (coordinates, kind, etc.).
    // But we need to compare all the other values (mainly, palindex).
    if let Some(contained) = self.inner.get(stitch) {
      contained == stitch
    } else {
      false
    }
  }

  /// Inserts a stitch into the set, replacing the existing one.
  /// Returns the replaced stitch if any.
  pub fn insert(&mut self, stitch: T) -> Option<T> {
    // We need to use the `replace` method to get the replaced value from the set.
    // We need to return the previous value to pass it back to the caller, so it can be used to update the pattern on the frontend.
    self.inner.replace(stitch)
  }

  /// Removes and returns a stitch from the set.
  pub fn remove(&mut self, stitch: &T) -> Option<T> {
    // We need to use the `take` method to get the actual value from the set.
    // The passed `stitch` contains only the fields that are used for ordering (coordinates, kind, etc.).
    // Hovewer, we need to return the actual stitch that contains all the other values (mainly, palindex), so it can be used to update the pattern on the frontend.
    self.inner.take(stitch)
  }

  pub fn get(&self, stitch: &T) -> Option<&T> {
    self.inner.get(stitch)
  }

  pub fn extend(&mut self, stitches: Stitches<T>) {
    self.inner.extend(stitches.inner);
  }
}

impl<T: Ord> FromIterator<T> for Stitches<T> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    Self { inner: BTreeSet::from_iter(iter) }
  }
}

impl<T: Ord> Default for Stitches<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl Stitches<FullStitch> {
  /// Removes and returns all the conflicts with a given full stitch.
  /// It looks for any petite stitches that overlap with the full stitch.
  pub fn remove_conflicts_with_full_stitch(&mut self, fullstitch: &FullStitch) -> Vec<FullStitch> {
    debug_assert_eq!(fullstitch.kind, FullStitchKind::Full);
    let mut conflicts = Vec::new();

    let x = fullstitch.x + 0.5;
    let y = fullstitch.y + 0.5;
    let kind = FullStitchKind::Petite;

    for petite in [
      FullStitch { kind, ..*fullstitch },
      FullStitch { x, kind, ..*fullstitch },
      FullStitch { y, kind, ..*fullstitch },
      FullStitch { x, y, kind, ..*fullstitch },
    ] {
      self.remove(&petite).inspect(|&petite| conflicts.push(petite));
    }

    conflicts
  }

  /// Removes and returns all the conflicts with a given petite stitch.
  /// It looks for the full stitch that overlaps with the petite stitch.
  pub fn remove_conflicts_with_petite_stitch(&mut self, fullstitch: &FullStitch) -> Vec<FullStitch> {
    debug_assert_eq!(fullstitch.kind, FullStitchKind::Petite);
    let mut conflicts = Vec::new();

    let fullstitch = FullStitch {
      x: NotNan::new(fullstitch.x.trunc()).unwrap(),
      y: NotNan::new(fullstitch.y.trunc()).unwrap(),
      palindex: fullstitch.palindex,
      kind: FullStitchKind::Full,
    };

    self.remove(&fullstitch).inspect(|&fs| conflicts.push(fs));

    conflicts
  }

  /// Removes and returns all the conflicts with a given half stitch.
  /// It looks for the full and any petite stitches that overlap with the half stitch.
  pub fn remove_conflicts_with_half_stitch(&mut self, partstitch: &PartStitch) -> Vec<FullStitch> {
    debug_assert_eq!(partstitch.kind, PartStitchKind::Half);
    let mut conflicts = Vec::new();
    let fullstitch: FullStitch = partstitch.to_owned().into();

    let x = partstitch.x + 0.5;
    let y = partstitch.y + 0.5;
    let kind = FullStitchKind::Petite;
    match partstitch.direction {
      PartStitchDirection::Forward => {
        for petite in [
          FullStitch { x, kind, ..fullstitch },
          FullStitch { y, kind, ..fullstitch },
        ] {
          self.remove(&petite).inspect(|&petite| conflicts.push(petite));
        }
      }
      PartStitchDirection::Backward => {
        for petite in [
          FullStitch { kind, ..fullstitch },
          FullStitch { x, y, kind, ..fullstitch },
        ] {
          self.remove(&petite).inspect(|&petite| conflicts.push(petite));
        }
      }
    };

    self.remove(&fullstitch).inspect(|&fs| conflicts.push(fs));

    conflicts
  }

  /// Removes and returns all the conflicts with a given quarter stitch.
  /// It looks for the full and petite stitches that overlap with the quarter stitch.
  pub fn remove_conflicts_with_quarter_stitch(&mut self, partstitch: &PartStitch) -> Vec<FullStitch> {
    debug_assert_eq!(partstitch.kind, PartStitchKind::Quarter);
    let mut conflicts = Vec::new();

    for fullstitch in [
      FullStitch {
        x: NotNan::new(partstitch.x.trunc()).unwrap(),
        y: NotNan::new(partstitch.y.trunc()).unwrap(),
        palindex: partstitch.palindex,
        kind: FullStitchKind::Full,
      },
      partstitch.to_owned().into(), // Petite
    ] {
      self.remove(&fullstitch).inspect(|&fs| conflicts.push(fs));
    }

    conflicts
  }
}

impl Stitches<PartStitch> {
  /// Removes and returns all the conflicts with a given full stitch.
  /// It looks for any half and quarter stitches that overlap with the full stitch.
  pub fn remove_conflicts_with_full_stitch(&mut self, fullstitch: &FullStitch) -> Vec<PartStitch> {
    debug_assert_eq!(fullstitch.kind, FullStitchKind::Full);
    let mut conflicts = Vec::new();

    let partstitch: PartStitch = fullstitch.to_owned().into();
    let x = fullstitch.x + 0.5;
    let y = fullstitch.y + 0.5;

    for partstitch in [
      PartStitch {
        direction: PartStitchDirection::Forward,
        ..partstitch
      },
      PartStitch {
        direction: PartStitchDirection::Backward,
        ..partstitch
      },
      PartStitch {
        kind: PartStitchKind::Quarter,
        direction: PartStitchDirection::Backward,
        ..partstitch
      },
      PartStitch {
        x,
        kind: PartStitchKind::Quarter,
        direction: PartStitchDirection::Forward,
        ..partstitch
      },
      PartStitch {
        y,
        kind: PartStitchKind::Quarter,
        direction: PartStitchDirection::Forward,
        ..partstitch
      },
      PartStitch {
        x,
        y,
        kind: PartStitchKind::Quarter,
        direction: PartStitchDirection::Backward,
        ..partstitch
      },
    ] {
      self.remove(&partstitch).inspect(|&ps| conflicts.push(ps));
    }

    conflicts
  }

  /// Removes and returns all the conflicts with a given petite stitch.
  /// It looks for the half and quarter stitches that overlap with the petite stitch.
  pub fn remove_conflicts_with_petite_stitch(&mut self, fullstitch: &FullStitch) -> Vec<PartStitch> {
    debug_assert_eq!(fullstitch.kind, FullStitchKind::Petite);
    let mut conflicts = Vec::new();

    let x = fullstitch.x;
    let y = fullstitch.y;
    let palindex = fullstitch.palindex;
    let direction = PartStitchDirection::from((x, y));

    let half = PartStitch {
      x: NotNan::new(x.trunc()).unwrap(),
      y: NotNan::new(y.trunc()).unwrap(),
      palindex,
      direction,
      kind: PartStitchKind::Half,
    };
    self.remove(&half).inspect(|&half| conflicts.push(half));

    let quarter = PartStitch {
      x,
      y,
      palindex,
      direction,
      kind: PartStitchKind::Quarter,
    };
    self.remove(&quarter).inspect(|&quarter| conflicts.push(quarter));

    conflicts
  }

  /// Removes and returns all the conflicts with a given half stitch.
  /// It looks for any quarter stitches that overlap with the half stitch.
  pub fn remove_conflicts_with_half_stitch(&mut self, partstitch: &PartStitch) -> Vec<PartStitch> {
    debug_assert_eq!(partstitch.kind, PartStitchKind::Half);
    let mut conflicts = Vec::new();

    let x = partstitch.x + 0.5;
    let y = partstitch.y + 0.5;
    let kind = PartStitchKind::Quarter;

    match partstitch.direction {
      PartStitchDirection::Forward => {
        for quarter in [
          PartStitch {
            x,
            kind,
            direction: PartStitchDirection::Forward,
            ..*partstitch
          },
          PartStitch {
            y,
            kind,
            direction: PartStitchDirection::Forward,
            ..*partstitch
          },
        ] {
          self.remove(&quarter).inspect(|&quarter| conflicts.push(quarter));
        }
      }
      PartStitchDirection::Backward => {
        for quarter in [
          PartStitch {
            kind,
            direction: PartStitchDirection::Backward,
            ..*partstitch
          },
          PartStitch {
            x,
            y,
            kind,
            direction: PartStitchDirection::Backward,
            ..*partstitch
          },
        ] {
          self.remove(&quarter).inspect(|&quarter| conflicts.push(quarter));
        }
      }
    }

    conflicts
  }

  /// Removes and returns all the conflicts with a given quarter stitch.
  /// It looks for the half stitch that overlap with the quarter stitch.
  pub fn remove_conflicts_with_quarter_stitch(&mut self, partstitch: &PartStitch) -> Vec<PartStitch> {
    debug_assert_eq!(partstitch.kind, PartStitchKind::Quarter);
    let mut conflicts = Vec::new();

    let half = PartStitch {
      x: NotNan::new(partstitch.x.trunc()).unwrap(),
      y: NotNan::new(partstitch.y.trunc()).unwrap(),
      palindex: partstitch.palindex,
      direction: PartStitchDirection::from((partstitch.x, partstitch.y)),
      kind: PartStitchKind::Half,
    };
    self.remove(&half).inspect(|&half| conflicts.push(half));

    conflicts
  }
}

// TODO: rewrite
// Just defines some common methods to work with the palette indices.
// That allows to share some logic across different stitch types.
pub trait PaletteIndex {
  fn palindex(&self) -> u8;
  fn set_palindex(&mut self, palindex: u8);
}

impl<T: Ord + PaletteIndex> Stitches<T> {
  pub fn remove_stitches_by_palindex(&mut self, palindex: u8) -> Vec<T> {
    let mut conflicts = Vec::new();
    for mut stitch in std::mem::take(&mut self.inner).into_iter() {
      match stitch.palindex().cmp(&palindex) {
        Ordering::Less => {
          self.inner.insert(stitch);
        }
        Ordering::Equal => {
          conflicts.push(stitch);
        }
        Ordering::Greater => {
          stitch.set_palindex(stitch.palindex() - 1);
          self.inner.insert(stitch);
        }
      };
    }
    conflicts
  }

  pub fn restore_stitches(&mut self, stitches: Vec<T>, palindex: u8) {
    for mut stitch in std::mem::take(&mut self.inner).into_iter() {
      if stitch.palindex() >= palindex {
        stitch.set_palindex(stitch.palindex() + 1);
      }
      self.inner.insert(stitch);
    }
    self.inner.extend(stitches);
  }
}

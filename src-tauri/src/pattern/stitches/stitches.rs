use std::collections::BTreeSet;

use borsh::{BorshDeserialize, BorshSerialize};
use ordered_float::NotNan;
use serde::{Deserialize, Serialize};

use super::*;

#[cfg(test)]
#[path = "./stitches.test.rs"]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stitch {
  Full(FullStitch),
  Part(PartStitch),
  Node(Node),
  Line(Line),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct StitchConflicts {
  fullstitches: Vec<FullStitch>,
  partstitches: Vec<PartStitch>,
  node: Option<Node>,
  line: Option<Line>,
}

impl StitchConflicts {
  pub fn with_fullstitches(mut self, fullstitches: Vec<FullStitch>) -> Self {
    self.fullstitches = fullstitches;
    self
  }

  pub fn with_fullstitch(mut self, fullstitch: Option<FullStitch>) -> Self {
    if let Some(fullstitch) = fullstitch {
      self.fullstitches.push(fullstitch);
    }
    self
  }

  pub fn with_partstitches(mut self, partstitches: Vec<PartStitch>) -> Self {
    self.partstitches = partstitches;
    self
  }

  pub fn with_partstitch(mut self, partstitch: Option<PartStitch>) -> Self {
    if let Some(partstitch) = partstitch {
      self.partstitches.push(partstitch);
    }
    self
  }

  pub fn with_node(mut self, node: Option<Node>) -> Self {
    self.node = node;
    self
  }

  pub fn with_line(mut self, line: Option<Line>) -> Self {
    self.line = line;
    self
  }
}

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

  pub fn insert(&mut self, stitch: T) -> Option<T> {
    self.inner.replace(stitch)
  }

  pub fn remove(&mut self, stitch: &T) -> bool {
    self.inner.remove(stitch)
  }

  pub fn get(&self, stitch: &T) -> Option<&T> {
    self.inner.get(stitch)
  }
}

impl<T: Ord> FromIterator<T> for Stitches<T> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    Self { inner: BTreeSet::from_iter(iter) }
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
      self.remove(&petite).then(|| conflicts.push(petite));
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

    self.remove(&fullstitch).then(|| conflicts.push(fullstitch));

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
          self.remove(&petite).then(|| conflicts.push(petite));
        }
      }
      PartStitchDirection::Backward => {
        for petite in [
          FullStitch { kind, ..fullstitch },
          FullStitch { x, y, kind, ..fullstitch },
        ] {
          self.remove(&petite).then(|| conflicts.push(petite));
        }
      }
    };

    self.remove(&fullstitch).then(|| conflicts.push(fullstitch));

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
      self.remove(&fullstitch).then(|| conflicts.push(fullstitch));
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
      self.remove(&partstitch).then(|| conflicts.push(partstitch));
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
    self.remove(&half).then(|| conflicts.push(half));

    let quarter = PartStitch {
      x,
      y,
      palindex,
      direction,
      kind: PartStitchKind::Quarter,
    };
    self.remove(&quarter).then(|| conflicts.push(quarter));

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
          self.remove(&quarter).then(|| conflicts.push(quarter));
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
          self.remove(&quarter).then(|| conflicts.push(quarter));
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
    self.remove(&half).then(|| conflicts.push(half));

    conflicts
  }
}

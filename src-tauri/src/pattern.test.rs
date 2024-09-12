use crate::pattern::*;

mod keys {
  use super::*;

  #[test]
  fn fullstitches() {
    let fullstitch = FullStitch {
      x: 1.0,
      y: 2.0,
      palindex: 0,
      kind: FullStitchKind::Full,
    };
    assert_eq!(fullstitch.key(), String::from("1.0:2.0|0"));

    let petite = FullStitch {
      x: 5.5,
      y: 2.5,
      palindex: 0,
      kind: FullStitchKind::Petite,
    };
    assert_eq!(petite.key(), String::from("5.5:2.5|1"));
  }

  #[test]
  fn partstitches() {
    let halfstitch = PartStitch {
      x: 1.0,
      y: 2.0,
      palindex: 0,
      direction: PartStitchDirection::Forward,
      kind: PartStitchKind::Half,
    };
    assert_eq!(halfstitch.key(), String::from("1.0:2.0|0|0"));

    let quarter = PartStitch {
      x: 5.5,
      y: 2.5,
      palindex: 0,
      direction: PartStitchDirection::Backward,
      kind: PartStitchKind::Quarter,
    };
    assert_eq!(quarter.key(), String::from("5.5:2.5|1|1"));
  }

  #[test]
  fn lines() {
    let backstitch = Line {
      x: (1.0, 2.0),
      y: (5.5, 2.5),
      palindex: 0,
      kind: LineKind::Back,
    };
    assert_eq!(backstitch.key(), String::from("1.0:5.5:2.0:2.5"));

    let straightstitch = Line {
      x: (1.0, 2.0),
      y: (5.5, 2.5),
      palindex: 0,
      kind: LineKind::Straight,
    };
    assert_eq!(straightstitch.key(), String::from("1.0:5.5:2.0:2.5"));
  }

  #[test]
  fn nodes() {
    let frenchknot = Node {
      x: 1.0,
      y: 2.0,
      rotated: false,
      palindex: 0,
      kind: NodeKind::FrenchKnot,
    };
    assert_eq!(frenchknot.key(), String::from("1.0:2.0"));

    let bead = Node {
      x: 5.5,
      y: 2.5,
      rotated: true,
      palindex: 0,
      kind: NodeKind::Bead,
    };
    assert_eq!(bead.key(), String::from("5.5:2.5"));
  }
}

mod stitches {
  use super::*;
  use std::sync::LazyLock;

  static TEST_FULLSTITCHES: LazyLock<Stitches<FullStitch>> = LazyLock::new(|| {
    Stitches::from_iter([
      FullStitch {
        x: 0.0,
        y: 0.0,
        palindex: 0,
        kind: FullStitchKind::Full,
      },
      FullStitch {
        x: 1.0,
        y: 1.0,
        palindex: 0,
        kind: FullStitchKind::Petite,
      },
      FullStitch {
        x: 1.5,
        y: 1.0,
        palindex: 0,
        kind: FullStitchKind::Petite,
      },
      FullStitch {
        x: 1.0,
        y: 1.5,
        palindex: 0,
        kind: FullStitchKind::Petite,
      },
      FullStitch {
        x: 1.5,
        y: 1.5,
        palindex: 0,
        kind: FullStitchKind::Petite,
      },
    ])
  });

  static TEST_PARTSTITCHES: LazyLock<Stitches<PartStitch>> = LazyLock::new(|| {
    Stitches::from_iter([
      PartStitch {
        x: 0.0,
        y: 0.0,
        palindex: 0,
        direction: PartStitchDirection::Backward,
        kind: PartStitchKind::Half,
      },
      PartStitch {
        x: 0.0,
        y: 0.0,
        palindex: 0,
        direction: PartStitchDirection::Forward,
        kind: PartStitchKind::Half,
      },
      PartStitch {
        x: 1.0,
        y: 1.0,
        palindex: 0,
        direction: PartStitchDirection::Backward,
        kind: PartStitchKind::Quarter,
      },
      PartStitch {
        x: 1.5,
        y: 1.0,
        palindex: 0,
        direction: PartStitchDirection::Forward,
        kind: PartStitchKind::Quarter,
      },
      PartStitch {
        x: 1.0,
        y: 1.5,
        palindex: 0,
        direction: PartStitchDirection::Forward,
        kind: PartStitchKind::Quarter,
      },
      PartStitch {
        x: 1.5,
        y: 1.5,
        palindex: 0,
        direction: PartStitchDirection::Backward,
        kind: PartStitchKind::Quarter,
      },
    ])
  });

  static TEST_LINES: LazyLock<Stitches<Line>> = LazyLock::new(|| {
    Stitches::from_iter([
      Line {
        x: (0.0, 1.0),
        y: (0.0, 1.0),
        palindex: 0,
        kind: LineKind::Back,
      },
      Line {
        x: (1.0, 2.0),
        y: (1.0, 2.0),
        palindex: 0,
        kind: LineKind::Straight,
      },
    ])
  });

  static TEST_NODES: LazyLock<Stitches<Node>> = LazyLock::new(|| {
    Stitches::from_iter([
      Node {
        x: 0.0,
        y: 0.0,
        rotated: false,
        palindex: 0,
        kind: NodeKind::FrenchKnot,
      },
      Node {
        x: 1.0,
        y: 1.0,
        rotated: false,
        palindex: 0,
        kind: NodeKind::Bead,
      },
    ])
  });

  fn full(base: f64) -> FullStitch {
    FullStitch {
      x: base,
      y: base,
      palindex: 0,
      kind: FullStitchKind::Full,
    }
  }

  fn petites(base: f64) -> [FullStitch; 4] {
    [
      FullStitch {
        x: base,
        y: base,
        palindex: 0,
        kind: FullStitchKind::Petite,
      },
      FullStitch {
        x: base + 0.5,
        y: base,
        palindex: 0,
        kind: FullStitchKind::Petite,
      },
      FullStitch {
        x: base,
        y: base + 0.5,
        palindex: 0,
        kind: FullStitchKind::Petite,
      },
      FullStitch {
        x: base + 0.5,
        y: base + 0.5,
        palindex: 0,
        kind: FullStitchKind::Petite,
      },
    ]
  }

  fn halves(base: f64) -> [PartStitch; 2] {
    [
      PartStitch {
        x: base,
        y: base,
        palindex: 0,
        direction: PartStitchDirection::Backward,
        kind: PartStitchKind::Half,
      },
      PartStitch {
        x: base,
        y: base,
        palindex: 0,
        direction: PartStitchDirection::Forward,
        kind: PartStitchKind::Half,
      },
    ]
  }

  fn quarters(base: f64) -> [PartStitch; 4] {
    [
      PartStitch {
        x: base,
        y: base,
        palindex: 0,
        direction: PartStitchDirection::Backward,
        kind: PartStitchKind::Quarter,
      },
      PartStitch {
        x: base + 0.5,
        y: base,
        palindex: 0,
        direction: PartStitchDirection::Forward,
        kind: PartStitchKind::Quarter,
      },
      PartStitch {
        x: base,
        y: base + 0.5,
        palindex: 0,
        direction: PartStitchDirection::Forward,
        kind: PartStitchKind::Quarter,
      },
      PartStitch {
        x: base + 0.5,
        y: base + 0.5,
        palindex: 0,
        direction: PartStitchDirection::Backward,
        kind: PartStitchKind::Quarter,
      },
    ]
  }

  fn line(base: f64, kind: LineKind) -> Line {
    Line {
      x: (base, base + 1.0),
      y: (base, base + 1.0),
      palindex: 0,
      kind,
    }
  }

  fn node(base: f64, kind: NodeKind) -> Node {
    Node {
      x: base,
      y: base,
      rotated: false,
      palindex: 0,
      kind,
    }
  }

  #[test]
  fn new_stitches_should_not_conflict() {
    let fullstitch = full(10.0);
    assert!(TEST_FULLSTITCHES.get(&fullstitch.key()).is_none());
    assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_full_stitch(&fullstitch).len(), 0);
    assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_full_stitch(&fullstitch).len(), 0);

    for petite in petites(10.0) {
      assert!(TEST_FULLSTITCHES.get(&petite.key()).is_none());
      assert!(TEST_FULLSTITCHES.find_conflicts_with_petite_stitch(&petite).is_none());
      assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_petite_stitch(&petite).len(), 0);
    }

    for half in halves(10.0) {
      assert!(TEST_PARTSTITCHES.get(&half.key()).is_none());
      assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_half_stitch(&half).len(), 0);
      assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_half_stitch(&half).len(), 0);
    }

    for quarter in quarters(10.0) {
      assert!(TEST_FULLSTITCHES.get(&quarter.key()).is_none());
      assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_quarter_stitch(&quarter).len(), 0);
      assert!(TEST_PARTSTITCHES.find_conflicts_with_quarter_stitch(&quarter).is_none());
    }

    let back = line(10.0, LineKind::Back);
    assert!(TEST_LINES.get(&back.key()).is_none());

    let straight = line(10.0, LineKind::Straight);
    assert!(TEST_LINES.get(&straight.key()).is_none());

    let frenchknot = node(10.0, NodeKind::FrenchKnot);
    assert!(TEST_NODES.get(&frenchknot.key()).is_none());

    let bead = node(10.0, NodeKind::Bead);
    assert!(TEST_NODES.get(&bead.key()).is_none());
  }

  #[test]
  fn full_stitch_conflicts_with_full_stitch() {
    let fullstitch = full(0.0);
    assert!(TEST_FULLSTITCHES.get(&fullstitch.key()).is_some());
    assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_full_stitch(&fullstitch).len(), 0);
  }

  #[test]
  fn full_stitch_conflicts_with_petite_stitches() {
    let fullstitch = full(1.0);
    assert!(TEST_FULLSTITCHES.get(&fullstitch.key()).is_none());
    assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_full_stitch(&fullstitch).len(), 4);
  }

  #[test]
  fn full_stitch_conflicts_with_half_stitches() {
    let fullstitch = full(0.0);
    assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_full_stitch(&fullstitch).len(), 2);
  }

  #[test]
  fn full_stitch_conflicts_with_quarter_stitches() {
    let fullstitch = full(1.0);
    assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_full_stitch(&fullstitch).len(), 4);
  }

  #[test]
  fn petite_stitches_conflict_with_full_stitches() {
    for petite in petites(0.0) {
      assert!(TEST_FULLSTITCHES.find_conflicts_with_petite_stitch(&petite).is_some());
    }
  }

  #[test]
  fn petite_stitches_conflict_with_petite_stitches() {
    for petite in petites(1.0) {
      assert!(TEST_FULLSTITCHES.get(&petite.key()).is_some());
      assert!(TEST_FULLSTITCHES.find_conflicts_with_petite_stitch(&petite).is_none());
    }
  }

  #[test]
  fn petite_stitches_conflict_with_half_stitches() {
    for petite in petites(0.0) {
      assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_petite_stitch(&petite).len(), 1);
    }
  }

  #[test]
  fn petite_stitches_conflict_with_quarter_stitches() {
    for petite in petites(1.0) {
      assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_petite_stitch(&petite).len(), 1);
    }
  }

  #[test]
  fn half_stitches_conflict_with_full_stitches() {
    for half in halves(0.0) {
      assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_half_stitch(&half).len(), 1);
    }
  }

  #[test]
  fn half_stitches_conflict_with_petite_stitches() {
    for half in halves(1.0) {
      assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_half_stitch(&half).len(), 2);
    }
  }

  #[test]
  fn half_stitches_conflict_with_half_stitches() {
    for half in halves(0.0) {
      assert!(TEST_PARTSTITCHES.get(&half.key()).is_some());
    }
  }

  #[test]
  fn half_stitches_conflict_with_quarter_stitches() {
    for half in halves(1.0) {
      assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_half_stitch(&half).len(), 2);
    }
  }

  #[test]
  fn quarter_stitches_conflict_with_full_stitches() {
    for quarter in quarters(0.0) {
      assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_quarter_stitch(&quarter).len(), 1);
    }
  }

  #[test]
  fn quarter_stitches_conflict_with_petite_stitches() {
    for quarter in quarters(1.0) {
      assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_quarter_stitch(&quarter).len(), 1);
    }
  }

  #[test]
  fn quarter_stitches_conflict_with_half_stitches() {
    for quarter in quarters(0.0) {
      assert!(TEST_PARTSTITCHES.find_conflicts_with_quarter_stitch(&quarter).is_some());
    }
  }

  #[test]
  fn quarter_stitches_conflict_with_quarter_stitches() {
    for quarter in quarters(1.0) {
      assert!(TEST_PARTSTITCHES.get(&quarter.key()).is_some());
    }
  }

  #[test]
  fn line_conflicts_with_line() {
    let back = line(0.0, LineKind::Back);
    assert!(TEST_LINES.get(&back.key()).is_some());
    let back = line(1.0, LineKind::Back);
    assert!(TEST_LINES.get(&back.key()).is_some());

    let straight = line(0.0, LineKind::Straight);
    assert!(TEST_LINES.get(&straight.key()).is_some());
    let straight = line(1.0, LineKind::Straight);
    assert!(TEST_LINES.get(&straight.key()).is_some());
  }

  #[test]
  fn node_conflicts_with_node() {
    let frenchknot = node(0.0, NodeKind::FrenchKnot);
    assert!(TEST_NODES.get(&frenchknot.key()).is_some());
    let frenchknot = node(1.0, NodeKind::FrenchKnot);
    assert!(TEST_NODES.get(&frenchknot.key()).is_some());

    let bead = node(0.0, NodeKind::Bead);
    assert!(TEST_NODES.get(&bead.key()).is_some());
    let bead = node(1.0, NodeKind::Bead);
    assert!(TEST_NODES.get(&bead.key()).is_some());
  }
}

use crate::pattern::*;

mod stitches {
  use super::*;
  use std::sync::LazyLock;

  static TEST_FULLSTITCHES: LazyLock<Stitches<FullStitch>> = LazyLock::new(|| {
    Stitches::from_iter([
      FullStitch {
        x: NotNan::new(0.0).unwrap(),
        y: NotNan::new(0.0).unwrap(),
        palindex: 0,
        kind: FullStitchKind::Full,
      },
      FullStitch {
        x: NotNan::new(1.0).unwrap(),
        y: NotNan::new(1.0).unwrap(),
        palindex: 0,
        kind: FullStitchKind::Petite,
      },
      FullStitch {
        x: NotNan::new(1.5).unwrap(),
        y: NotNan::new(1.0).unwrap(),
        palindex: 0,
        kind: FullStitchKind::Petite,
      },
      FullStitch {
        x: NotNan::new(1.0).unwrap(),
        y: NotNan::new(1.5).unwrap(),
        palindex: 0,
        kind: FullStitchKind::Petite,
      },
      FullStitch {
        x: NotNan::new(1.5).unwrap(),
        y: NotNan::new(1.5).unwrap(),
        palindex: 0,
        kind: FullStitchKind::Petite,
      },
    ])
  });

  static TEST_PARTSTITCHES: LazyLock<Stitches<PartStitch>> = LazyLock::new(|| {
    Stitches::from_iter([
      PartStitch {
        x: NotNan::new(0.0).unwrap(),
        y: NotNan::new(0.0).unwrap(),
        palindex: 0,
        direction: PartStitchDirection::Backward,
        kind: PartStitchKind::Half,
      },
      PartStitch {
        x: NotNan::new(0.0).unwrap(),
        y: NotNan::new(0.0).unwrap(),
        palindex: 0,
        direction: PartStitchDirection::Forward,
        kind: PartStitchKind::Half,
      },
      PartStitch {
        x: NotNan::new(1.0).unwrap(),
        y: NotNan::new(1.0).unwrap(),
        palindex: 0,
        direction: PartStitchDirection::Backward,
        kind: PartStitchKind::Quarter,
      },
      PartStitch {
        x: NotNan::new(1.5).unwrap(),
        y: NotNan::new(1.0).unwrap(),
        palindex: 0,
        direction: PartStitchDirection::Forward,
        kind: PartStitchKind::Quarter,
      },
      PartStitch {
        x: NotNan::new(1.0).unwrap(),
        y: NotNan::new(1.5).unwrap(),
        palindex: 0,
        direction: PartStitchDirection::Forward,
        kind: PartStitchKind::Quarter,
      },
      PartStitch {
        x: NotNan::new(1.5).unwrap(),
        y: NotNan::new(1.5).unwrap(),
        palindex: 0,
        direction: PartStitchDirection::Backward,
        kind: PartStitchKind::Quarter,
      },
    ])
  });

  static TEST_LINES: LazyLock<Stitches<Line>> = LazyLock::new(|| {
    Stitches::from_iter([
      Line {
        x: (NotNan::new(0.0).unwrap(), NotNan::new(1.0).unwrap()),
        y: (NotNan::new(0.0).unwrap(), NotNan::new(1.0).unwrap()),
        palindex: 0,
        kind: LineKind::Back,
      },
      Line {
        x: (NotNan::new(1.0).unwrap(), NotNan::new(2.0).unwrap()),
        y: (NotNan::new(1.0).unwrap(), NotNan::new(2.0).unwrap()),
        palindex: 0,
        kind: LineKind::Straight,
      },
    ])
  });

  static TEST_NODES: LazyLock<Stitches<Node>> = LazyLock::new(|| {
    Stitches::from_iter([
      Node {
        x: NotNan::new(0.0).unwrap(),
        y: NotNan::new(0.0).unwrap(),
        rotated: false,
        palindex: 0,
        kind: NodeKind::FrenchKnot,
      },
      Node {
        x: NotNan::new(1.0).unwrap(),
        y: NotNan::new(1.0).unwrap(),
        rotated: false,
        palindex: 0,
        kind: NodeKind::Bead,
      },
    ])
  });

  fn full(base: NotNan<f32>) -> FullStitch {
    FullStitch {
      x: base,
      y: base,
      palindex: 0,
      kind: FullStitchKind::Full,
    }
  }

  fn petites(base: NotNan<f32>) -> [FullStitch; 4] {
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

  fn halves(base: NotNan<f32>) -> [PartStitch; 2] {
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

  fn quarters(base: NotNan<f32>) -> [PartStitch; 4] {
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

  fn line(base: NotNan<f32>, kind: LineKind) -> Line {
    Line {
      x: (base, base + 1.0),
      y: (base, base + 1.0),
      palindex: 0,
      kind,
    }
  }

  fn node(base: NotNan<f32>, kind: NodeKind) -> Node {
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
    let fullstitch = full(NotNan::new(10.0).unwrap());
    assert!(TEST_FULLSTITCHES.get(&fullstitch).is_none());
    assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_full_stitch(&fullstitch).len(), 0);
    assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_full_stitch(&fullstitch).len(), 0);

    for petite in petites(NotNan::new(10.0).unwrap()) {
      assert!(TEST_FULLSTITCHES.get(&petite).is_none());
      assert!(TEST_FULLSTITCHES.find_conflicts_with_petite_stitch(&petite).is_none());
      assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_petite_stitch(&petite).len(), 0);
    }

    for half in halves(NotNan::new(10.0).unwrap()) {
      assert!(TEST_PARTSTITCHES.get(&half).is_none());
      assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_half_stitch(&half).len(), 0);
      assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_half_stitch(&half).len(), 0);
    }

    for quarter in quarters(NotNan::new(10.0).unwrap()) {
      assert!(TEST_PARTSTITCHES.get(&quarter).is_none());
      assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_quarter_stitch(&quarter).len(), 0);
      assert!(TEST_PARTSTITCHES.find_conflicts_with_quarter_stitch(&quarter).is_none());
    }

    let back = line(NotNan::new(10.0).unwrap(), LineKind::Back);
    assert!(TEST_LINES.get(&back).is_none());

    let straight = line(NotNan::new(10.0).unwrap(), LineKind::Straight);
    assert!(TEST_LINES.get(&straight).is_none());

    let frenchknot = node(NotNan::new(10.0).unwrap(), NodeKind::FrenchKnot);
    assert!(TEST_NODES.get(&frenchknot).is_none());

    let bead = node(NotNan::new(10.0).unwrap(), NodeKind::Bead);
    assert!(TEST_NODES.get(&bead).is_none());
  }

  #[test]
  fn full_stitch_conflicts_with_full_stitch() {
    let fullstitch = full(NotNan::new(0.0).unwrap());
    assert!(TEST_FULLSTITCHES.get(&fullstitch).is_some());
    assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_full_stitch(&fullstitch).len(), 0);
  }

  #[test]
  fn full_stitch_conflicts_with_petite_stitches() {
    let fullstitch = full(NotNan::new(1.0).unwrap());
    assert!(TEST_FULLSTITCHES.get(&fullstitch).is_none());
    assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_full_stitch(&fullstitch).len(), 4);
  }

  #[test]
  fn full_stitch_conflicts_with_half_stitches() {
    let fullstitch = full(NotNan::new(0.0).unwrap());
    assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_full_stitch(&fullstitch).len(), 2);
  }

  #[test]
  fn full_stitch_conflicts_with_quarter_stitches() {
    let fullstitch = full(NotNan::new(1.0).unwrap());
    assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_full_stitch(&fullstitch).len(), 4);
  }

  #[test]
  fn petite_stitches_conflict_with_full_stitches() {
    for petite in petites(NotNan::new(0.0).unwrap()) {
      assert!(TEST_FULLSTITCHES.find_conflicts_with_petite_stitch(&petite).is_some());
    }
  }

  #[test]
  fn petite_stitches_conflict_with_petite_stitches() {
    for petite in petites(NotNan::new(1.0).unwrap()) {
      assert!(TEST_FULLSTITCHES.get(&petite).is_some());
      assert!(TEST_FULLSTITCHES.find_conflicts_with_petite_stitch(&petite).is_none());
    }
  }

  #[test]
  fn petite_stitches_conflict_with_half_stitches() {
    for petite in petites(NotNan::new(0.0).unwrap()) {
      assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_petite_stitch(&petite).len(), 1);
    }
  }

  #[test]
  fn petite_stitches_conflict_with_quarter_stitches() {
    for petite in petites(NotNan::new(1.0).unwrap()) {
      assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_petite_stitch(&petite).len(), 1);
    }
  }

  #[test]
  fn half_stitches_conflict_with_full_stitches() {
    for half in halves(NotNan::new(0.0).unwrap()) {
      assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_half_stitch(&half).len(), 1);
    }
  }

  #[test]
  fn half_stitches_conflict_with_petite_stitches() {
    for half in halves(NotNan::new(1.0).unwrap()) {
      assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_half_stitch(&half).len(), 2);
    }
  }

  #[test]
  fn half_stitches_conflict_with_half_stitches() {
    for half in halves(NotNan::new(0.0).unwrap()) {
      assert!(TEST_PARTSTITCHES.get(&half).is_some());
    }
  }

  #[test]
  fn half_stitches_conflict_with_quarter_stitches() {
    for half in halves(NotNan::new(1.0).unwrap()) {
      assert_eq!(TEST_PARTSTITCHES.find_conflicts_with_half_stitch(&half).len(), 2);
    }
  }

  #[test]
  fn quarter_stitches_conflict_with_full_stitches() {
    for quarter in quarters(NotNan::new(0.0).unwrap()) {
      assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_quarter_stitch(&quarter).len(), 1);
    }
  }

  #[test]
  fn quarter_stitches_conflict_with_petite_stitches() {
    for quarter in quarters(NotNan::new(1.0).unwrap()) {
      assert_eq!(TEST_FULLSTITCHES.find_conflicts_with_quarter_stitch(&quarter).len(), 1);
    }
  }

  #[test]
  fn quarter_stitches_conflict_with_half_stitches() {
    for quarter in quarters(NotNan::new(0.0).unwrap()) {
      assert!(TEST_PARTSTITCHES.find_conflicts_with_quarter_stitch(&quarter).is_some());
    }
  }

  #[test]
  fn quarter_stitches_conflict_with_quarter_stitches() {
    for quarter in quarters(NotNan::new(1.0).unwrap()) {
      assert!(TEST_PARTSTITCHES.get(&quarter).is_some());
    }
  }

  #[test]
  fn line_conflicts_with_line() {
    let back = line(NotNan::new(0.0).unwrap(), LineKind::Back);
    assert!(TEST_LINES.get(&back).is_some());
    let back = line(NotNan::new(1.0).unwrap(), LineKind::Back);
    assert!(TEST_LINES.get(&back).is_some());

    let straight = line(NotNan::new(0.0).unwrap(), LineKind::Straight);
    assert!(TEST_LINES.get(&straight).is_some());
    let straight = line(NotNan::new(1.0).unwrap(), LineKind::Straight);
    assert!(TEST_LINES.get(&straight).is_some());
  }

  #[test]
  fn node_conflicts_with_node() {
    let frenchknot = node(NotNan::new(0.0).unwrap(), NodeKind::FrenchKnot);
    assert!(TEST_NODES.get(&frenchknot).is_some());
    let frenchknot = node(NotNan::new(1.0).unwrap(), NodeKind::FrenchKnot);
    assert!(TEST_NODES.get(&frenchknot).is_some());

    let bead = node(NotNan::new(0.0).unwrap(), NodeKind::Bead);
    assert!(TEST_NODES.get(&bead).is_some());
    let bead = node(NotNan::new(1.0).unwrap(), NodeKind::Bead);
    assert!(TEST_NODES.get(&bead).is_some());
  }
}

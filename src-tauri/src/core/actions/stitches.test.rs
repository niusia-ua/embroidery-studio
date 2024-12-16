use ordered_float::NotNan;
use tauri::test::{mock_builder, MockRuntime};
use tauri::{generate_context, App, Listener, WebviewUrl, WebviewWindowBuilder};

use super::{Action, AddStitchAction, RemoveStitchAction};
use crate::core::pattern::*;

pub fn setup_app() -> App<MockRuntime> {
  mock_builder().build(generate_context!()).unwrap()
}

fn create_pattern_project() -> PatternProject {
  let mut patproj = PatternProject::default();

  // top-left petite
  patproj.pattern.fullstitches.insert(FullStitch {
    x: NotNan::new(0.0).unwrap(),
    y: NotNan::new(0.0).unwrap(),
    palindex: 0,
    kind: FullStitchKind::Petite,
  });
  // top-right quarter
  patproj.pattern.partstitches.insert(PartStitch {
    x: NotNan::new(0.5).unwrap(),
    y: NotNan::new(0.0).unwrap(),
    palindex: 0,
    kind: PartStitchKind::Quarter,
    direction: PartStitchDirection::Forward,
  });
  // bottom-left petite
  patproj.pattern.fullstitches.insert(FullStitch {
    x: NotNan::new(0.0).unwrap(),
    y: NotNan::new(0.5).unwrap(),
    palindex: 0,
    kind: FullStitchKind::Petite,
  });
  // bottom-right quarter
  patproj.pattern.partstitches.insert(PartStitch {
    x: NotNan::new(0.5).unwrap(),
    y: NotNan::new(0.5).unwrap(),
    palindex: 0,
    kind: PartStitchKind::Quarter,
    direction: PartStitchDirection::Backward,
  });

  patproj
}

#[test]
fn test_add_stitch() {
  let app = setup_app();
  let window = WebviewWindowBuilder::new(&app, "main", WebviewUrl::default())
    .build()
    .unwrap();
  let patproj = create_pattern_project();
  let stitch = Stitch::Full(FullStitch {
    x: NotNan::new(0.0).unwrap(),
    y: NotNan::new(0.0).unwrap(),
    palindex: 0,
    kind: FullStitchKind::Full,
  });
  let action = AddStitchAction::new(stitch);

  // Test executing the command.
  {
    window.listen("stitches:add_one", move |e| {
      assert_eq!(serde_json::from_str::<Stitch>(e.payload()).unwrap(), stitch);
    });

    window.listen("stitches:remove_many", |e| {
      let conflicts: Vec<Stitch> = serde_json::from_str(e.payload()).unwrap();
      assert_eq!(conflicts.len(), 4);
    });

    let mut patproj = patproj.clone();
    action.perform(&window, &mut patproj).unwrap();

    assert_eq!(patproj.pattern.fullstitches.len(), 1);
    assert_eq!(patproj.pattern.partstitches.len(), 0);
  }

  // Test revoking the command.
  {
    window.listen("stitches:remove_one", move |e| {
      assert_eq!(serde_json::from_str::<Stitch>(e.payload()).unwrap(), stitch);
    });

    window.listen("stitches:add_many", |e| {
      let conflicts: Vec<Stitch> = serde_json::from_str(e.payload()).unwrap();
      assert_eq!(conflicts.len(), 4);
    });

    let mut patproj = patproj.clone();
    action.revoke(&window, &mut patproj).unwrap();

    assert_eq!(patproj.pattern.fullstitches.len(), 2);
    assert_eq!(patproj.pattern.partstitches.len(), 2);
  }
}

#[test]
fn test_remove_stitch() {
  let app = setup_app();
  let window = WebviewWindowBuilder::new(&app, "main", WebviewUrl::default())
    .build()
    .unwrap();
  let patproj = create_pattern_project();
  let stitch = Stitch::Full(FullStitch {
    x: NotNan::new(0.0).unwrap(),
    y: NotNan::new(0.0).unwrap(),
    palindex: 0,
    kind: FullStitchKind::Petite,
  });
  let action = RemoveStitchAction::new(stitch);

  // Test executing the command.
  {
    window.listen("stitches:remove_one", move |e| {
      assert_eq!(serde_json::from_str::<Stitch>(e.payload()).unwrap(), stitch);
    });

    let mut patproj = patproj.clone();
    action.perform(&window, &mut patproj).unwrap();

    assert_eq!(patproj.pattern.fullstitches.len(), 1);
    assert_eq!(patproj.pattern.partstitches.len(), 2);
  }

  // Test revoking the command.
  {
    window.listen("stitches:add_one", move |e| {
      assert_eq!(serde_json::from_str::<Stitch>(e.payload()).unwrap(), stitch);
    });

    let mut patproj = patproj.clone();
    action.revoke(&window, &mut patproj).unwrap();

    assert_eq!(patproj.pattern.fullstitches.len(), 2);
    assert_eq!(patproj.pattern.partstitches.len(), 2);
  }
}

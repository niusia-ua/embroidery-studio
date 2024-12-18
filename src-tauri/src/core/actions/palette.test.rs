use display::{Formats, Symbols};
use tauri::test::{mock_builder, MockRuntime};
use tauri::{generate_context, App, Listener, WebviewUrl, WebviewWindowBuilder};

use super::{Action, AddPaletteItemAction, AddedPaletteItemData, RemovePaletteItemAction};
use crate::core::parser::oxs;
use crate::core::pattern::*;

fn setup_app() -> App<MockRuntime> {
  mock_builder().build(generate_context!()).unwrap()
}

fn create_pattern_project() -> PatternProject {
  let file_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/patterns/piggies.oxs");
  oxs::parse_pattern(file_path).unwrap()
}

#[test]
fn test_add_palette_item() {
  let app = setup_app();
  let window = WebviewWindowBuilder::new(&app, "main", WebviewUrl::default())
    .build()
    .unwrap();

  let mut patproj = create_pattern_project();
  let palitem = PaletteItem {
    brand: String::from("DMC"),
    number: String::from("3825"),
    name: String::from("Pumpkin-Pale"),
    color: String::from("F5BA82"),
    blends: None,
    bead: None,
    strands: None,
  };
  let action = AddPaletteItemAction::new(palitem.clone());

  // Test executing the command.
  {
    window.listen("palette:add_palette_item", move |e| {
      assert_eq!(
        serde_json::from_str::<AddedPaletteItemData>(e.payload()).unwrap(),
        AddedPaletteItemData {
          palitem: palitem.clone(),
          palindex: 7,
          symbols: Symbols::default(),
          formats: Formats::default(),
        }
      );
    });

    assert_eq!(patproj.pattern.palette.len(), 7);
    action.perform(&window, &mut patproj).unwrap();
    assert_eq!(patproj.pattern.palette.len(), 8);
  }

  // Test revoking the command.
  {
    window.listen("palette:remove_palette_item", move |e| {
      assert_eq!(serde_json::from_str::<usize>(e.payload()).unwrap(), 7);
    });

    assert_eq!(patproj.pattern.palette.len(), 8);
    action.revoke(&window, &mut patproj).unwrap();
    assert_eq!(patproj.pattern.palette.len(), 7);
  }
}

#[test]
fn test_remove_palette_item() {
  let app = setup_app();
  let window = WebviewWindowBuilder::new(&app, "main", WebviewUrl::default())
    .build()
    .unwrap();

  let mut patproj = create_pattern_project();
  let palitem = PaletteItem {
    brand: String::from("DMC"),
    number: String::from("310"),
    name: String::from("Black"),
    color: String::from("000000"),
    blends: None,
    bead: None,
    strands: None,
  };
  let action = RemovePaletteItemAction::new(palitem.clone());

  // Test executing the command.
  {
    window.listen("palette:remove_palette_item", move |e| {
      assert_eq!(serde_json::from_str::<usize>(e.payload()).unwrap(), 2);
    });
    window.listen("stitches:remove_many", move |e| {
      assert!(!serde_json::from_str::<Vec<Stitch>>(e.payload()).unwrap().is_empty());
    });

    assert_eq!(patproj.pattern.palette.len(), 7);
    action.perform(&window, &mut patproj).unwrap();
    assert_eq!(patproj.pattern.palette.len(), 6);
  }

  // Test revoking the command.
  {
    window.listen("palette:add_palette_item", move |e| {
      assert_eq!(
        serde_json::from_str::<AddedPaletteItemData>(e.payload()).unwrap(),
        AddedPaletteItemData {
          palitem: palitem.clone(),
          palindex: 2,
          symbols: Symbols::default(),
          formats: Formats::default(),
        }
      );
    });
    window.listen("stitches:add_many", move |e| {
      assert!(!serde_json::from_str::<Vec<Stitch>>(e.payload()).unwrap().is_empty());
    });

    assert_eq!(patproj.pattern.palette.len(), 6);
    action.revoke(&window, &mut patproj).unwrap();
    assert_eq!(patproj.pattern.palette.len(), 7);
  }
}

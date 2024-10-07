use super::{parse_pattern, Software};
use crate::pattern::*;

#[test]
fn parses_oxs_v1_0_pattern() {
  let pathbuf = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/patterns/piggies.oxs");
  let pattern = parse_pattern(pathbuf.as_path(), Software::Ursa);
  assert!(pattern.is_ok());

  let pattern = pattern.unwrap();

  assert_eq!(pattern.properties, PatternProperties { width: 69, height: 73 });

  assert_eq!(
    pattern.info,
    PatternInfo {
      title: String::from(""),
      author: String::from(""),
      copyright: String::from("by Ursa Software"),
      description: String::from(""),
    }
  );

  assert_eq!(pattern.palette.len(), 7);
  assert_eq!(
    pattern.palette[0],
    PaletteItem {
      brand: String::from("DMC"),
      number: String::from("943"),
      name: String::from("Turquoise VY DK"),
      color: String::from("23725C"),
      blends: None,
    }
  );
  assert_eq!(
    pattern.palette[6],
    PaletteItem {
      brand: String::from("DMC"),
      number: String::from("367"),
      name: String::from("Pistachio Green dark"),
      color: String::from("406647"),
      blends: None,
    }
  );

  assert_eq!(
    pattern.fabric,
    Fabric {
      stitches_per_inch: (14, 14),
      kind: String::from("Aida"),
      name: String::from("cloth"),
      color: String::from("FFFFFF"),
    }
  );

  assert_eq!(pattern.fullstitches.len(), 1000);
  assert_eq!(pattern.partstitches.len(), 55);
  assert_eq!(pattern.nodes.len(), 18);
  assert_eq!(pattern.lines.len(), 1105);
}

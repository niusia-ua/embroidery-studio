use std::{fs, path::Path};

use embroidery_studio::pattern::load_pattern;

#[test]
fn parses_supported_pattern_formats() {
  let resources = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/patterns");
  let paths = fs::read_dir(resources).unwrap();

  for path in paths {
    let path = path.unwrap().path();
    assert!(load_pattern(path.to_str().unwrap()).is_ok());
  }
}

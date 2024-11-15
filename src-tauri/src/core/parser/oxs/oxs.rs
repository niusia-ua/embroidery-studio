use anyhow::{bail, Result};
use quick_xml::events::Event;

use crate::core::pattern::PatternProject;

use super::{
  utils::{process_attributes, OxsVersion, Software},
  v1_0,
};

pub fn parse_pattern(file_path: std::path::PathBuf) -> Result<PatternProject> {
  log::info!("Parsing the OXS pattern");

  let mut reader = quick_xml::Reader::from_file(&file_path)?;
  let mut buf = Vec::new();
  let (oxs_version, software) = loop {
    match reader.read_event_into(&mut buf) {
      Ok(Event::Empty(ref e)) => {
        if e.name().as_ref() == b"properties" {
          let attributes = process_attributes(e.attributes())?;
          let oxs_version: OxsVersion = attributes.get("oxsversion").unwrap().parse()?;
          let software: Software = attributes.get("software").unwrap().parse()?;
          break (oxs_version, software);
        }
      }
      // We don't expect to receive EOF here,
      // because we should have found the properties tag,
      // which is at the beginning of the file.
      Ok(Event::Eof) => bail!("Unexpected EOF"),
      Err(e) => bail!("Error at position {}: {e:?}", reader.error_position()),
      _ => {}
    }
    buf.clear();
  };

  let pattern_project = match oxs_version {
    OxsVersion::V1_0 => v1_0::parse_pattern(file_path, software)?,
  };

  Ok(pattern_project)
}

pub fn save_pattern(patproj: &PatternProject) -> Result<()> {
  log::info!("Saving the OXS pattern");
  v1_0::save_pattern(patproj)
}

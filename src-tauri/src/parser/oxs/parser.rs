use anyhow::{bail, Result};
use quick_xml::events::Event;

use crate::pattern::Pattern;

use super::{
  utils::{process_attributes, OxsVersion, Software},
  v1_0,
};

pub fn parse_pattern(path: impl AsRef<std::path::Path>) -> Result<Pattern> {
  log::trace!("Parsing the OXS pattern");

  let mut reader = quick_xml::Reader::from_file(path.as_ref())?;
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

  let pattern = match oxs_version {
    OxsVersion::V1_0 => v1_0::parse_pattern(path.as_ref(), software)?,
  };

  Ok(pattern)
}

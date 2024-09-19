use std::ffi::OsStr;

use crate::error::Error;

pub enum PatternFormat {
  Xsd,
  Oxs,
  Json,
}

impl TryFrom<Option<&OsStr>> for PatternFormat {
  type Error = Error;

  fn try_from(value: Option<&OsStr>) -> Result<Self, Self::Error> {
    if let Some(extension) = value {
      let extension = extension.to_str().unwrap();
      match extension.to_lowercase().as_str() {
        "xsd" => Ok(Self::Xsd),
        "oxs" | "xml" => Ok(Self::Oxs),
        "json" => Ok(Self::Json),
        _ => Err(Error::UnsupportedPatternType {
          extension: extension.to_uppercase(),
        }),
      }
    } else {
      Err(Error::UnsupportedPatternType {
        extension: "[no extension]".to_string(),
      })
    }
  }
}

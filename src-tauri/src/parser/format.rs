use std::ffi::OsStr;

pub enum PatternFormat {
  Xsd,
  Oxs,
  Embx,
}

impl TryFrom<Option<&OsStr>> for PatternFormat {
  type Error = anyhow::Error;

  fn try_from(value: Option<&OsStr>) -> anyhow::Result<Self, Self::Error> {
    if let Some(extension) = value {
      let extension = extension.to_str().unwrap();
      match extension.to_lowercase().as_str() {
        "xsd" => Ok(Self::Xsd),
        "oxs" | "xml" => Ok(Self::Oxs),
        "embx" => Ok(Self::Embx),
        _ => anyhow::bail!("Unsupported pattern type: {extension}."),
      }
    } else {
      anyhow::bail!("Unsupported pattern type")
    }
  }
}

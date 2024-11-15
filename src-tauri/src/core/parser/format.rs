use std::ffi::OsStr;

#[derive(Default)]
pub enum PatternFormat {
  /// Probably, stands for `Cross-Stitch Design`.
  /// Only **read-only** mode is currently available.
  Xsd,

  /// Stands for `Open Cross-Stitch`.
  /// It is just an XML document.
  /// This format is intended to be a lingua franca in the embroidery world.
  Oxs,

  /// Stands for `Embroidery Project`.
  /// It is a ZIP archive with a pack of binary files.
  /// This format is not recommended for other applications.
  #[default]
  EmbProj,
}

impl TryFrom<Option<&OsStr>> for PatternFormat {
  type Error = anyhow::Error;

  fn try_from(value: Option<&OsStr>) -> anyhow::Result<Self, Self::Error> {
    if let Some(extension) = value {
      let extension = extension.to_str().unwrap();
      match extension.to_lowercase().as_str() {
        "xsd" => Ok(Self::Xsd),
        "oxs" | "xml" => Ok(Self::Oxs),
        "embproj" => Ok(Self::EmbProj),
        _ => anyhow::bail!("Unsupported pattern type: {extension}."),
      }
    } else {
      anyhow::bail!("Unsupported pattern type")
    }
  }
}

impl std::fmt::Display for PatternFormat {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Xsd => write!(f, "xsd"),
      Self::Oxs => write!(f, "oxs"),
      Self::EmbProj => write!(f, "embproj"),
    }
  }
}

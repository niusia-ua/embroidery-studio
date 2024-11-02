//! This is a set of utilities to make working with `anyhow` and `tauri` easier.
//! It is based on https://github.com/TDiblik/anyhow-tauri.

#[derive(Debug)]
pub struct CommandError(anyhow::Error);

impl std::error::Error for CommandError {}

impl std::fmt::Display for CommandError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#}", self.0)
  }
}

impl From<anyhow::Error> for CommandError {
  fn from(error: anyhow::Error) -> Self {
    Self(error)
  }
}

impl From<std::io::Error> for CommandError {
  fn from(error: std::io::Error) -> Self {
    Self(anyhow::Error::from(error))
  }
}

impl From<tauri::Error> for CommandError {
  fn from(error: tauri::Error) -> Self {
    Self(anyhow::Error::from(error))
  }
}

impl serde::Serialize for CommandError {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("{:#}", self.0))
  }
}

pub type CommandResult<T> = std::result::Result<T, CommandError>;

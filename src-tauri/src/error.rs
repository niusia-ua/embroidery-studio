#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("The signature of Pattern Maker v4 is incorrect, so it looks like it is not an embroidery pattern file.")]
  XsdInvalidSignature,

  #[error("Unsupported pattern type: {}.", extension)]
  UnsupportedPatternType { extension: String },

  #[error(transparent)]
  Io(#[from] std::io::Error),
}

impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

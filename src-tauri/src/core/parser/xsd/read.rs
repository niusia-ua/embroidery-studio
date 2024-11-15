use std::io::{Read, Result};

use byteorder::ReadBytesExt;
use memchr::memchr;

#[cfg(test)]
#[path = "read.test.rs"]
mod tests;

/// Provides additional methods for reading XSD data.
pub trait ReadXsdExt: Read + ReadBytesExt {
  /// Reads a C-style string with a specified length.
  /// The string can be in UTF-8 or CP1251 encoding.
  fn read_cstring(&mut self, length: usize) -> Result<String> {
    let mut buf = vec![0; length + 1]; // +1 for the null terminator.
    self.read_exact(&mut buf)?;

    // It is an edge case when the string is full of trash data.
    if memchr(0, &buf).is_none() {
      return Ok(String::from(""));
    }

    // It is safe to unwrap because we have checked the presence of the null terminator.
    let cstr = std::ffi::CStr::from_bytes_until_nul(&buf).unwrap();
    let string = match cstr.to_str() {
      // The string is in UTF-8 (English).
      Ok(str) => String::from(str),

      // The string is in CP1251 (Russian).
      Err(_) => encoding_rs::WINDOWS_1251.decode(cstr.to_bytes()).0.to_string(),
    };

    Ok(string)
  }

  /// Reads a hex color as `String`.
  fn read_hex_color(&mut self) -> Result<String> {
    let mut buf: [u8; 3] = [0; 3];
    self.read_exact(&mut buf)?;
    Ok(hex::encode_upper(buf))
  }
}

/// All types that implement `Read` get methods defined in `ReadXsdExt`.
impl<R: Read + ?Sized> ReadXsdExt for R {}

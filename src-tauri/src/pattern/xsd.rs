//! A parser for the proprietary XSD pattern format.
//!
//! The specification of this format was obtained by reverse engineering several applications, including Pattern Maker.
//! Therefore, it is rather incomplete, but it contains all the knowledge to be able to extract enough data to display the pattern.

use std::{
  ffi::CStr,
  fs,
  io::{self, Read, Seek, SeekFrom},
  path::Path,
  sync::LazyLock,
};

use byteorder::{LittleEndian, ReadBytesExt};
use memchr::memchr;

use super::*;

#[cfg(test)]
#[path = "xsd.test.rs"]
mod tests;

static PM_FLOSS_BRANDS: LazyLock<std::collections::HashMap<u8, String>> = LazyLock::new(|| {
  let pm_floss_brands = include_str!("./pm_floss_brands.json");
  serde_json::from_str(pm_floss_brands).unwrap()
});

const XSD_VALID_SIGNATURE: u16 = 1296;
const XSD_COLOR_NUMBER_LENGTH: usize = 11;
const XSD_COLOR_NAME_LENGTH: usize = 41;
const XSD_PATTERN_NAME_LENGTH: usize = 41;
const XSD_AUTHOR_NAME_LENGTH: usize = 41;
const XSD_COPYRIGHT_LENGTH: usize = 201;
const XSD_PATTERN_NOTES_LENGTH: usize = 2049;
const XSD_FABRIC_COLOR_NAME_LENGTH: usize = 41;
const XSD_FABRIC_KIND_LENGTH: usize = 41;
const XSD_FORMAT_LENGTH: usize = 240;
const XSD_BLEND_COLORS_NUMBER: usize = 4;
const XSD_STITCH_TYPES_NUMBER: usize = 9;
const XSD_SPECIAL_STITCH_NAME_LENGTH: usize = 256;

#[derive(Debug, PartialEq)]
struct XsdPatternProperties {
  width: u16,
  height: u16,
  small_stitches_count: u32,
  joints_count: u16,
  stitches_per_inch: (u16, u16),
  palette_size: usize,
}

#[derive(Debug, PartialEq)]
struct XsdFabric {
  name: String,
  color: String,
}

#[derive(Debug, PartialEq)]
enum XsdSmallStitchKind {
  HalfTop,
  HalfBottom,
  QuarterTopLeft,
  QuarterBottomLeft,
  QuarterTopRight,
  QuarterBottomRight,
  PetiteTopLeft,
  PetiteBottomLeft,
  PetiteTopRight,
  PetiteBottomRight,
}

#[derive(Debug, PartialEq)]
enum XsdJointKind {
  FrenchKnot,
  Back,
  Curve,
  Special,
  Straight,
  Bead,
}

impl From<u16> for XsdJointKind {
  fn from(value: u16) -> Self {
    match value {
      1 => XsdJointKind::FrenchKnot,
      2 => XsdJointKind::Back,
      3 => XsdJointKind::Curve,
      4 => XsdJointKind::Special,
      5 => XsdJointKind::Straight,
      6 => XsdJointKind::Bead,
      _ => {
        log::warn!("Unknown joint kind {}", value);
        panic!("An unknown type of XsdJointKind was encountered.")
      }
    }
  }
}

// (significant_byte_index, bitand_arg, palindex_index, kind)
type SmallStitchData = (usize, u8, usize, XsdSmallStitchKind);

// The next two arrays are used only in the `map_stitches_data_into_stitches` function
// to map values from bytes buffer into stitches.
const PART_STITCH_DATA: [SmallStitchData; 6] = [
  (0, 1, 2, XsdSmallStitchKind::HalfTop),
  (0, 2, 3, XsdSmallStitchKind::HalfBottom),
  (0, 4, 4, XsdSmallStitchKind::QuarterTopLeft),
  (0, 8, 5, XsdSmallStitchKind::QuarterBottomLeft),
  (0, 16, 6, XsdSmallStitchKind::QuarterTopRight),
  (0, 32, 7, XsdSmallStitchKind::QuarterBottomRight),
];
const PETITE_STITCH_DATA: [SmallStitchData; 4] = [
  (1, 1, 4, XsdSmallStitchKind::PetiteTopLeft),
  (1, 2, 5, XsdSmallStitchKind::PetiteBottomLeft),
  (1, 4, 6, XsdSmallStitchKind::PetiteTopRight),
  (1, 8, 7, XsdSmallStitchKind::PetiteBottomRight),
];

type XsdRandomNumbers = [i32; 4];
type SmallStitchBuffer = [u8; 10];
type XsdDecodingNumbers = [u32; 16];

type Cursor = io::Cursor<Vec<u8>>;

/// Provides additional methods for reading XSD data.
trait XsdRead: Read + Seek {
  /// Reads a C-style string with a specified length.
  /// The string can be in UTF-8 or CP1251 encoding.
  fn read_cstring(&mut self, length: usize) -> Result<String>;

  /// Reads a hex color as `String`.
  fn read_hex_color(&mut self) -> Result<String>;

  /// Reads node and line coordinates.
  fn read_fractional_coors(&mut self) -> Result<(f64, f64)>;
}

impl XsdRead for Cursor {
  fn read_cstring(&mut self, length: usize) -> Result<String> {
    let mut buf = vec![0; length];
    self.read_exact(&mut buf)?;

    // It is an edge case when the string is full of trash data.
    if memchr(0, &buf).is_none() {
      return Ok(String::from(""));
    }

    let cstr = CStr::from_bytes_until_nul(&buf).unwrap();
    let string = match cstr.to_str() {
      // The string is in UTF-8 (English).
      Ok(str) => String::from(str),

      // The string is in CP1251 (Russian).
      Err(_) => encoding_rs::WINDOWS_1251.decode(cstr.to_bytes()).0.to_string(),
    };

    Ok(string)
  }

  fn read_hex_color(&mut self) -> Result<String> {
    let mut buf: [u8; 3] = [0; 3];
    self.read_exact(&mut buf)?;
    Ok(hex::encode_upper(buf))
  }

  fn read_fractional_coors(&mut self) -> Result<(f64, f64)> {
    // The resolution of coordinates is 1/2 of a pattern cell.
    let x = self.read_u16::<LittleEndian>()? as f64 / 2.0;
    let y = self.read_u16::<LittleEndian>()? as f64 / 2.0;
    Ok((x, y))
  }
}

pub fn parse_pattern(path: impl AsRef<Path>) -> Result<Pattern> {
  log::info!("Parsing the XSD pattern file");
  let buf = fs::read(path)?;
  let mut cursor = io::Cursor::new(buf);

  let signature = read_signature(&mut cursor)?;
  if signature != XSD_VALID_SIGNATURE {
    log::error!("The file has an invalid signature {:?}", signature);
    return Err(Error::XsdInvalidSignature);
  }

  cursor.seek(SeekFrom::Current(739))?; // Skip the unknown data.
  let xsd_pattern_properties = read_pattern_properties(&mut cursor)?;
  let palette_size = xsd_pattern_properties.palette_size;
  let palette = read_palette(&mut cursor, palette_size)?;
  cursor.seek(SeekFrom::Current((palette_size * 2) as i64))?; // Skip the palette item positions.
  skip_palette_items_notes(&mut cursor, palette_size)?;
  cursor.seek(SeekFrom::Current((palette_size * 16) as i64))?; // Skip the strands.
  cursor.seek(SeekFrom::Current((XSD_FORMAT_LENGTH * 10) as i64))?; // Skip the symbol formats.
  cursor.seek(SeekFrom::Current((XSD_FORMAT_LENGTH * 10) as i64))?; // Skip the back stitch formats.
  cursor.seek(SeekFrom::Current((XSD_FORMAT_LENGTH * 4) as i64))?; // Skip the unknown formats.
  cursor.seek(SeekFrom::Current((XSD_FORMAT_LENGTH * 10) as i64))?; // Skip the special stitch formats.
  cursor.seek(SeekFrom::Current((XSD_FORMAT_LENGTH * 10) as i64))?; // Skip the straight stitch formats.
  cursor.seek(SeekFrom::Current((XSD_FORMAT_LENGTH * 10) as i64))?; // Skip the french knot formats.
  cursor.seek(SeekFrom::Current((XSD_FORMAT_LENGTH * 10) as i64))?; // Skip the bead formats.
  cursor.seek(SeekFrom::Current((XSD_FORMAT_LENGTH * 53) as i64))?; // Skip the font formats.
  cursor.seek(SeekFrom::Current((palette_size * 12) as i64))?; // Skip the palette item symbols.
  cursor.seek(SeekFrom::Current(380))?; // Skip the pattern settings.
  cursor.seek(SeekFrom::Current(56))?; // Skip the grid settings.
  let xsd_fabric = read_fabric_info(&mut cursor)?;
  let (pattern_info, fabric_kind) = read_pattern_info(&mut cursor)?;
  cursor.seek(SeekFrom::Current(52))?; // Skip the stitch settings.
  cursor.seek(SeekFrom::Current(30))?; // Skip the symbol settings.
  cursor.seek(SeekFrom::Current(16414))?; // Skip the library info.
  cursor.seek(SeekFrom::Current(512))?; // Skip the machine export info.
  let (fullstitches, partstitches) = read_stitches(&mut cursor, &xsd_pattern_properties)?;
  skip_special_stitch_models(&mut cursor)?;
  let (nodes, lines) = read_joints(&mut cursor, xsd_pattern_properties.joints_count)?;

  Ok(Pattern {
    properties: PatternProperties {
      width: xsd_pattern_properties.width,
      height: xsd_pattern_properties.height,
    },
    info: pattern_info,
    palette,
    fabric: Fabric {
      kind: fabric_kind,
      name: xsd_fabric.name,
      color: xsd_fabric.color,
      stitches_per_inch: xsd_pattern_properties.stitches_per_inch,
    },
    fullstitches: Stitches::from_iter(fullstitches),
    partstitches: Stitches::from_iter(partstitches),
    nodes: Stitches::from_iter(nodes),
    lines: Stitches::from_iter(lines),
  })
}

/// Reads the signature of the XSD file.
/// This function is potentially underdeveloped due to lack of knowledge about the XSD format.
fn read_signature(cursor: &mut Cursor) -> Result<u16> {
  let signature = cursor.read_u16::<LittleEndian>()?;
  Ok(signature)
}

/// Reads the pattern properties that are necessarry for further parsing.
fn read_pattern_properties(cursor: &mut Cursor) -> Result<XsdPatternProperties> {
  log::trace!("Reading the pattern properties");
  let width = cursor.read_u16::<LittleEndian>()?;
  let height = cursor.read_u16::<LittleEndian>()?;
  let small_stitches_count = cursor.read_u32::<LittleEndian>()?;
  let joints_count = cursor.read_u16::<LittleEndian>()?;
  let stitches_per_inch = (cursor.read_u16::<LittleEndian>()?, cursor.read_u16::<LittleEndian>()?);
  cursor.seek(SeekFrom::Current(6))?;
  let palette_size: usize = cursor.read_u16::<LittleEndian>()?.into();
  Ok(XsdPatternProperties {
    width,
    height,
    small_stitches_count,
    joints_count,
    stitches_per_inch,
    palette_size,
  })
}

/// Reads the color palette of the pattern.
fn read_palette(cursor: &mut Cursor, palette_size: usize) -> Result<Vec<PaletteItem>> {
  log::trace!("Reading the palette with {} items", palette_size);
  let mut palette = Vec::with_capacity(palette_size);
  for _ in 0..palette_size {
    palette.push(read_palette_item(cursor)?);
  }
  Ok(palette)
}

/// Reads a single palette item.
fn read_palette_item(cursor: &mut Cursor) -> Result<PaletteItem> {
  cursor.seek(SeekFrom::Current(2))?;
  let brand_id = cursor.read_u8()?;
  let brand = PM_FLOSS_BRANDS.get(&brand_id).unwrap().to_owned();
  let number = cursor.read_cstring(XSD_COLOR_NUMBER_LENGTH)?;
  let name = cursor.read_cstring(XSD_COLOR_NAME_LENGTH)?;
  let color = cursor.read_hex_color()?;
  cursor.seek(SeekFrom::Current(1))?;
  let blends = read_blends(cursor)?;
  cursor.seek(SeekFrom::Current(10))?;
  Ok(PaletteItem {
    brand,
    name,
    number,
    color,
    blends,
  })
}

/// Reads the blend colors of the palette item.
/// Used only in the `read_palette_item` function.
fn read_blends(cursor: &mut Cursor) -> Result<Option<Vec<Blend>>> {
  let blends_count: usize = cursor.read_u16::<LittleEndian>()?.into();
  if blends_count == 0 {
    cursor.seek(SeekFrom::Current(4 * 12 + 4))?;
    return Ok(None);
  }
  let mut blends: Vec<Blend> = Vec::with_capacity(blends_count);
  for i in 0..XSD_BLEND_COLORS_NUMBER {
    let blend_color = read_blend_item(cursor)?;
    // PM reserves and stores 4 blend colors, but we do not want to store empty blends.
    // Although, we must read all blend colors to keep the cursor in the right position.
    if i < blends_count {
      blends.push(blend_color);
    }
  }
  read_blend_strands(cursor, &mut blends)?;
  Ok(Some(blends))
}

/// Reads a single blend color.
/// Used only in the `read_blends` function.
fn read_blend_item(cursor: &mut Cursor) -> Result<Blend> {
  let brand_id = cursor.read_u8()?;
  let brand_id = if brand_id == 255 { 0 } else { brand_id };
  Ok(Blend {
    brand: PM_FLOSS_BRANDS.get(&brand_id).unwrap().to_owned(),
    number: cursor.read_cstring(XSD_COLOR_NUMBER_LENGTH)?,
    strands: 0, // The actual value will be set when calling `read_blend_strands`.
  })
}

/// Reads the number of strands for each blend color.
/// Used only in the `read_blends` function.
/// The function modifies the `blends` vector in place.
fn read_blend_strands(cursor: &mut Cursor, blends: &mut [Blend]) -> Result<()> {
  for i in 0..XSD_BLEND_COLORS_NUMBER {
    let strands = cursor.read_u8()?;
    if let Some(blend_color) = blends.get_mut(i) {
      blend_color.strands = strands;
    }
  }
  Ok(())
}

/// Skips the notes of the palette items.
fn skip_palette_items_notes(cursor: &mut Cursor, palette_size: usize) -> Result<()> {
  for _ in 0..palette_size {
    for _ in 0..XSD_STITCH_TYPES_NUMBER {
      let note_length = cursor.read_u16::<LittleEndian>()?;
      cursor.seek(SeekFrom::Current(note_length.into()))?;
    }
  }
  Ok(())
}

/// Reads a part of the fabric information.
fn read_fabric_info(cursor: &mut Cursor) -> Result<XsdFabric> {
  log::trace!("Reading the fabric info");
  let fabric_info = XsdFabric {
    name: cursor.read_cstring(XSD_FABRIC_COLOR_NAME_LENGTH)?,
    color: cursor.read_hex_color()?,
  };
  cursor.seek(SeekFrom::Current(65))?;
  Ok(fabric_info)
}

/// Reads the necessarry pattern information.
fn read_pattern_info(cursor: &mut Cursor) -> Result<(PatternInfo, String)> {
  log::trace!("Reading the pattern info");
  let title = cursor.read_cstring(XSD_PATTERN_NAME_LENGTH)?;
  let author = cursor.read_cstring(XSD_AUTHOR_NAME_LENGTH)?;
  cursor.seek(SeekFrom::Current(41))?; // Skip company name.
  let copyright = cursor.read_cstring(XSD_COPYRIGHT_LENGTH)?;
  let description = cursor.read_cstring(XSD_PATTERN_NOTES_LENGTH)?;
  let pattern_info = PatternInfo {
    title,
    author,
    copyright,
    description,
  };
  cursor.seek(SeekFrom::Current(6))?;
  let fabric_kind = cursor.read_cstring(XSD_FABRIC_KIND_LENGTH)?;
  cursor.seek(SeekFrom::Current(206))?;
  Ok((pattern_info, fabric_kind))
}

/// Reads the stitches of the pattern.
fn read_stitches(
  cursor: &mut Cursor,
  xsd_pattern_properties: &XsdPatternProperties,
) -> Result<(Vec<FullStitch>, Vec<PartStitch>)> {
  log::trace!("Reading the stitches");
  let total_stitches_count = ((xsd_pattern_properties.width as u64) * (xsd_pattern_properties.height as u64)) as usize;
  let small_stitches_count = xsd_pattern_properties.small_stitches_count as usize;
  let stitches_data = read_stitches_data(cursor, total_stitches_count)?;
  let small_stitch_buffers = read_small_stitch_buffers(cursor, small_stitches_count)?;
  let stitches = map_stitches_data_into_stitches(
    stitches_data,
    small_stitch_buffers,
    xsd_pattern_properties.width as usize,
  )?;
  Ok(stitches)
}

/// Reads the bytes buffer that contains the decoded stitches data.
fn read_stitches_data(cursor: &mut Cursor, total_stitches_count: usize) -> Result<Vec<i32>> {
  log::trace!("Reading the stitches data");
  let mut stitches_data = Vec::with_capacity(total_stitches_count);
  let mut xsd_random_numbers = read_xsd_random_numbers(cursor)?;
  let (mut decoding_key, decoding_numbers) = reproduce_decoding_values(&xsd_random_numbers)?;
  let mut decoding_number_index = 0;
  let mut stitch_index = 0;

  while stitch_index < total_stitches_count {
    let stitches_data_length = cursor.read_u32::<LittleEndian>()? as usize;

    if stitches_data_length == 0 {
      continue;
    }

    let mut decoded_stitches_data = Vec::with_capacity(stitches_data_length);

    // Decoding.
    for _ in 0..stitches_data_length {
      let stitch_data = cursor.read_i32::<LittleEndian>()? ^ decoding_key ^ xsd_random_numbers[0];
      decoded_stitches_data.push(stitch_data);
      decoding_key = decoding_key.rotate_left(decoding_numbers[decoding_number_index]);
      xsd_random_numbers[0] = xsd_random_numbers[0].wrapping_add(xsd_random_numbers[1]);
      decoding_number_index = (decoding_number_index + 1) % 16;
    }

    // Copying.
    let mut stitch_data_index = 0;
    while stitch_data_index < stitches_data_length {
      let mut copy_count = 1;
      let elem = decoded_stitches_data[stitch_data_index];

      if elem & (i32::MAX / 2 + 1) != 0 {
        copy_count = (elem & (i32::MAX / 2)) >> 16;
        stitch_data_index += 1;
      }

      while copy_count > 0 {
        stitches_data.push(decoded_stitches_data[stitch_data_index]);
        stitch_index += 1;
        copy_count -= 1;
      }

      stitch_data_index += 1;
    }
  }

  Ok(stitches_data)
}

/// Reads the random numbers that are necessarry for decoding the stitches data.
fn read_xsd_random_numbers(cursor: &mut Cursor) -> Result<XsdRandomNumbers> {
  log::trace!("Reading the XSD random numbers");
  let mut xsd_random_numbers = [0; 4];
  for number in &mut xsd_random_numbers {
    *number = cursor.read_i32::<LittleEndian>()?;
  }
  Ok(xsd_random_numbers)
}

/// Reproduces the decoding values that are used for decoding the stitches data.
fn reproduce_decoding_values(xsd_random_numbers: &XsdRandomNumbers) -> Result<(i32, XsdDecodingNumbers)> {
  log::trace!("Reproducing the decoding values");
  let val1 = xsd_random_numbers[1].to_le_bytes()[1] as i32;
  let val2 = xsd_random_numbers[0] << 8;
  let val3 = (val2 | val1) << 8;
  let val4 = xsd_random_numbers[2].to_le_bytes()[2] as i32;
  let val5 = (val4 | val3) << 8;
  let val6 = xsd_random_numbers[3] & 0xFF;
  let decoding_key = val6 | val5;

  let mut decoding_buffer = [0; 16];

  for i in 0..4 {
    let buf = xsd_random_numbers[i].to_le_bytes();
    for j in 0..4 {
      decoding_buffer[i * 4 + j] = buf[j];
    }
  }

  let mut decoding_buffer = io::Cursor::new(decoding_buffer);
  let mut decoding_numbers: XsdDecodingNumbers = [0; 16];

  for i in 0..16 {
    let offset = (i / 4) * 4; // 0, 4, 8, 12.
    decoding_buffer.seek(SeekFrom::Start(offset))?;
    let shift = decoding_buffer.read_u32::<LittleEndian>()? >> (i % 4);
    decoding_numbers[i as usize] = shift % 32;
  }

  Ok((decoding_key, decoding_numbers))
}

/// Reads the small stitch buffers that are used containe the small stitches data.
fn read_small_stitch_buffers(cursor: &mut Cursor, small_stitches_count: usize) -> Result<Vec<SmallStitchBuffer>> {
  log::trace!("Reading the small stitch buffers");
  let mut small_stitch_buffers = Vec::with_capacity(small_stitches_count);
  for _ in 0..small_stitches_count {
    let mut buf = [0; 10];
    cursor.read_exact(&mut buf)?;
    small_stitch_buffers.push(buf);
  }
  Ok(small_stitch_buffers)
}

/// Maps the stitches data into the full- and partstitches .
fn map_stitches_data_into_stitches(
  stitches_data: Vec<i32>,
  small_stitch_buffers: Vec<SmallStitchBuffer>,
  pattern_width: usize,
) -> Result<(Vec<FullStitch>, Vec<PartStitch>)> {
  let mut fullstitches = Vec::new();
  let mut partstitches = Vec::new();

  log::trace!("Mapping the stitches data into stitches");
  for (i, stitch_data) in stitches_data.iter().enumerate() {
    let stitch_buffer = stitch_data.to_le_bytes();

    // Empty cell.
    if stitch_buffer[3] == 15 {
      continue;
    }

    let x = (i % pattern_width) as f64;
    let y = (i / pattern_width) as f64;

    if stitch_buffer[3] == 0 {
      fullstitches.push(FullStitch {
        x,
        y,
        palindex: stitch_buffer[2],
        kind: FullStitchKind::Full,
      });
      continue;
    }

    let position = (stitches_data[i] >> 16) & ((u16::MAX / 2) as i32);
    let small_stitch_buffer = small_stitch_buffers.get(position as usize).unwrap();

    for (significant_byte_index, bitand_arg, palindex_index, kind) in PETITE_STITCH_DATA {
      let (x, y) = calc_small_stitch_coors(x, y, &kind);
      if small_stitch_buffer[significant_byte_index] & bitand_arg != 0 {
        fullstitches.push(FullStitch {
          x,
          y,
          palindex: small_stitch_buffer[palindex_index],
          kind: FullStitchKind::Petite,
        })
      }
    }

    for (significant_byte_index, bitand_arg, palindex_index, kind) in PART_STITCH_DATA {
      if small_stitch_buffer[significant_byte_index] & bitand_arg != 0 {
        let (x, y) = calc_small_stitch_coors(x, y, &kind);
        let direction = match kind {
          XsdSmallStitchKind::HalfTop | XsdSmallStitchKind::QuarterTopLeft | XsdSmallStitchKind::QuarterBottomRight => {
            PartStitchDirection::Backward
          }
          _ => PartStitchDirection::Forward,
        };
        let kind = match kind {
          XsdSmallStitchKind::HalfTop | XsdSmallStitchKind::HalfBottom => PartStitchKind::Half,
          _ => PartStitchKind::Quarter,
        };
        partstitches.push(PartStitch {
          x,
          y,
          palindex: small_stitch_buffer[palindex_index],
          direction,
          kind,
        })
      }
    }
  }

  Ok((fullstitches, partstitches))
}

/// Calculates the coordinates of the small stitch.
/// The XSD format contains coordinates without additional offsets relative to the cell.
/// But this is important for us.
fn calc_small_stitch_coors(x: f64, y: f64, kind: &XsdSmallStitchKind) -> (f64, f64) {
  match kind {
    XsdSmallStitchKind::QuarterTopLeft | XsdSmallStitchKind::PetiteTopLeft => (x, y),
    XsdSmallStitchKind::QuarterTopRight | XsdSmallStitchKind::PetiteTopRight => (x + 0.5, y),
    XsdSmallStitchKind::QuarterBottomLeft | XsdSmallStitchKind::PetiteBottomLeft => (x, y + 0.5),
    XsdSmallStitchKind::QuarterBottomRight | XsdSmallStitchKind::PetiteBottomRight => (x + 0.5, y + 0.5),
    _ => (x, y),
  }
}

/// Skips the special stitch models.
fn skip_special_stitch_models(cursor: &mut Cursor) -> Result<()> {
  cursor.seek(SeekFrom::Current(2))?;
  let special_stith_models_count: usize = cursor.read_u16::<LittleEndian>()?.into();

  for _ in 0..special_stith_models_count {
    if cursor.read_u16::<LittleEndian>()? != 4 {
      continue;
    }

    cursor.seek(SeekFrom::Current(2))?;
    let mut special_stitch_kind_buf = vec![0; 4];
    cursor.read_exact(&mut special_stitch_kind_buf)?;

    if String::from_utf8(special_stitch_kind_buf).unwrap() != "sps1" {
      continue;
    }

    cursor.seek(SeekFrom::Current((XSD_SPECIAL_STITCH_NAME_LENGTH * 2 + 2) as i64))?;

    for _ in 0..3 {
      cursor.seek(SeekFrom::Current(10))?;

      if read_signature(cursor)? != XSD_VALID_SIGNATURE {
        break;
      }

      let joints_count = cursor.read_u16::<LittleEndian>()?;

      if joints_count == 0 {
        continue;
      }

      read_joints(cursor, joints_count)?;
    }
  }

  Ok(())
}

/// Reads the french knots, beads, back, straight and special stitches and curves that used in the pattern.
fn read_joints(cursor: &mut Cursor, joints_count: u16) -> Result<(Vec<Node>, Vec<Line>)> {
  let mut nodes = Vec::new();
  let mut lines = Vec::new();

  log::trace!("Reading the joints");
  for _ in 0..joints_count {
    let joint_kind = XsdJointKind::from(cursor.read_u16::<LittleEndian>()?);
    match joint_kind {
      XsdJointKind::FrenchKnot => {
        cursor.seek(SeekFrom::Current(2))?;
        let (x, y) = cursor.read_fractional_coors()?;
        cursor.seek(SeekFrom::Current(4))?;
        let palindex = cursor.read_u8()?;
        cursor.seek(SeekFrom::Current(1))?;
        nodes.push(Node {
          x,
          y,
          rotated: false,
          palindex,
          kind: NodeKind::FrenchKnot,
        });
      }

      XsdJointKind::Back | XsdJointKind::Straight => {
        cursor.seek(SeekFrom::Current(2))?;
        let (x1, y1) = cursor.read_fractional_coors()?;
        let (x2, y2) = cursor.read_fractional_coors()?;
        let palindex = cursor.read_u8()?;
        cursor.seek(SeekFrom::Current(1))?;
        let kind = if joint_kind == XsdJointKind::Back {
          LineKind::Back
        } else {
          LineKind::Straight
        };
        lines.push(Line {
          x: (x1, x2),
          y: (y1, y2),
          palindex,
          kind,
        });
      }

      XsdJointKind::Curve => {
        cursor.seek(SeekFrom::Current(3))?;
        let points_count: usize = cursor.read_u16::<LittleEndian>()?.into();
        cursor.seek(SeekFrom::Current((points_count * 4) as i64))?;
      }

      XsdJointKind::Special => {
        cursor.seek(SeekFrom::Current(23))?;
      }

      XsdJointKind::Bead => {
        cursor.seek(SeekFrom::Current(2))?;
        let (x, y) = cursor.read_fractional_coors()?;
        let palindex = cursor.read_u8()?;
        cursor.seek(SeekFrom::Current(1))?;
        let rotated = matches!(cursor.read_u16::<LittleEndian>()?, 90 | 270);
        nodes.push(Node {
          x,
          y,
          rotated,
          palindex,
          kind: NodeKind::Bead,
        });
      }
    }
  }
  Ok((nodes, lines))
}

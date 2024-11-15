use std::io::Cursor;

use super::*;

#[test]
fn reads_cstring() {
  let utf8_buf = vec![0x57, 0x68, 0x69, 0x74, 0x65, 0x00, 0x00, 0x00];
  assert_eq!(Cursor::new(utf8_buf).read_cstring(7).unwrap(), String::from("White"));

  let cp1251_buf = vec![0xE3, 0xEE, 0xEB, 0xF3, 0xE1, 0xEE, 0xE9, 0x00];
  assert_eq!(
    Cursor::new(cp1251_buf).read_cstring(7).unwrap(),
    String::from("голубой")
  );
}

#[test]
fn returns_empty_string_on_non_null_terminated_cstring() {
  let not_nul_terminated_buf = vec![0x43, 0x6F, 0x66, 0x66, 0x65, 0x65];
  assert_eq!(
    Cursor::new(not_nul_terminated_buf).read_cstring(5).unwrap(),
    String::from("")
  );
}

#[test]
fn reads_hex_color() {
  let black_color_buf = vec![0x00, 0x00, 0x00];
  assert_eq!(
    Cursor::new(black_color_buf).read_hex_color().unwrap(),
    String::from("000000")
  );

  let white_color_buf = vec![0xff, 0xff, 0xff];
  assert_eq!(
    Cursor::new(white_color_buf).read_hex_color().unwrap(),
    String::from("FFFFFF")
  );
}

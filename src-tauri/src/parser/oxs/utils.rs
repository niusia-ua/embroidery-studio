use std::{collections::HashMap, str::FromStr};

use quick_xml::events::attributes::Attributes;

#[derive(Debug)]
pub enum OxsVersion {
  V1_0,
}

impl FromStr for OxsVersion {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "1.0" => Ok(OxsVersion::V1_0),
      _ => Err(()),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Software {
  Ursa,
}

impl FromStr for Software {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Ursa Software" => Ok(Software::Ursa),
      _ => Err(()),
    }
  }
}

pub fn process_attributes(attributes: Attributes) -> HashMap<String, String> {
  let mut map = HashMap::new();
  for attr in attributes {
    let attr = attr.unwrap();
    let key = String::from_utf8(attr.key.as_ref().to_vec()).unwrap();
    let value = String::from_utf8(attr.value.to_vec()).unwrap();
    map.insert(key, value);
  }
  map
}

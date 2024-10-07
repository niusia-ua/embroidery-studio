use std::{collections::HashMap, str::FromStr};

use anyhow::Result;
use quick_xml::events::attributes::Attributes;

#[derive(Debug)]
pub enum OxsVersion {
  V1_0,
}

impl FromStr for OxsVersion {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "1.0" => Ok(OxsVersion::V1_0),
      _ => anyhow::bail!("Unsupported OXS version: {s}"),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Software {
  Ursa,
}

impl FromStr for Software {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Ursa Software" => Ok(Software::Ursa),
      _ => anyhow::bail!("Unsupported software: {s}"),
    }
  }
}

pub fn process_attributes(attributes: Attributes) -> Result<HashMap<String, String>> {
  let mut map = HashMap::new();
  for attr in attributes {
    let attr = attr?;
    let key = String::from_utf8(attr.key.as_ref().to_vec())?;
    let value = String::from_utf8(attr.value.to_vec())?;
    map.insert(key, value);
  }
  Ok(map)
}

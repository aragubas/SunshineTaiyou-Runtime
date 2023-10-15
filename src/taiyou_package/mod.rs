use core::fmt;

use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
pub struct Version {  
  pub major: u8,
  pub minor: u8,
  pub patch: u8
}

#[derive(Serialize, Deserialize)]
pub struct TaiyouPackage {
  pub author: String,
  pub title: String,
  pub id: String,
  pub version: Version
}

impl fmt::Display for Version {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
  }
}
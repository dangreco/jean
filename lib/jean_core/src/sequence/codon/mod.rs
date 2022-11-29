use std::fmt::Display;

use crate::{data::cut::Cut, prelude::Translate};

use super::{
  protein::AminoAcid,
  rna::{self, Rna},
  Seq,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Codon((u8, u8, u8));

impl Codon {
  pub fn new(values: (u8, u8, u8)) -> Self {
    Self(values)
  }
}

impl Display for Codon {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}{}{}",
      self.0 .0 as char, self.0 .1 as char, self.0 .2 as char
    )
  }
}

impl<T> From<(T, T, T)> for Codon
where
  T: Into<char>,
{
  fn from(values: (T, T, T)) -> Self {
    Self((
      values.0.into() as u8,
      values.1.into() as u8,
      values.2.into() as u8,
    ))
  }
}

impl<T> From<Codon> for (T, T, T)
where
  T: From<char>,
{
  fn from(codon: Codon) -> Self {
    (
      T::from(codon.0 .0 as char),
      T::from(codon.0 .1 as char),
      T::from(codon.0 .2 as char),
    )
  }
}

impl TryFrom<&str> for Codon {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value.len() {
      3 => {
        let chars = value.chars().collect::<Vec<char>>();
        Ok(Self::new((chars[0] as u8, chars[1] as u8, chars[2] as u8)))
      }
      n => Err(format!("Invalid codon length (expected 3, found {n})")),
    }
  }
}

impl TryFrom<Codon> for Rna {
  type Error = String;

  fn try_from(value: Codon) -> Result<Self, Self::Error> {
    let b0: rna::Base = match (value.0 .0 as char).try_into() {
      Ok(base) => base,
      Err(_) => {
        return Err(format!(
          "Invalid RNA base at position 0: '{}'",
          value.0 .0 as char
        ))
      }
    };
    let b1: rna::Base = match (value.0 .1 as char).try_into() {
      Ok(base) => base,
      Err(_) => {
        return Err(format!(
          "Invalid RNA base at position 1: '{}'",
          value.0 .1 as char
        ))
      }
    };

    let b2: rna::Base = match (value.0 .2 as char).try_into() {
      Ok(base) => base,
      Err(_) => {
        return Err(format!(
          "Invalid RNA base at position 2: '{}'",
          value.0 .2 as char
        ))
      }
    };

    Ok(Seq(vec![b0, b1, b2]))
  }
}

impl Translate<AminoAcid> for Codon {
  fn translate(&self, table_id: &str) -> Result<AminoAcid, String> {
    let cut = Cut::get(table_id)?;
    cut.translate(&self)
  }
}

use crate::{
  prelude::{Complement, Transcribe},
  sequence::rna,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
  A,
  C,
  G,
  T,
  GAP,
}

impl TryFrom<char> for Base {
  type Error = String;

  fn try_from(c: char) -> Result<Self, Self::Error> {
    match c.to_ascii_uppercase() {
      'A' => Ok(Self::A),
      'C' => Ok(Self::C),
      'G' => Ok(Self::G),
      'T' => Ok(Self::T),
      '-' => Ok(Self::GAP),
      _ => Err(format!("Invalid DNA base '{c}'.")),
    }
  }
}

impl From<Base> for char {
  fn from(base: Base) -> Self {
    match base {
      Base::A => 'A',
      Base::C => 'C',
      Base::G => 'G',
      Base::T => 'T',
      Base::GAP => '-',
    }
  }
}

impl Complement for Base {
  fn complement(&self) -> Self {
    match self {
      Self::A => Self::T,
      Self::C => Self::G,
      Self::G => Self::C,
      Self::T => Self::A,
      Self::GAP => Self::GAP,
    }
  }
}

impl Transcribe<rna::Base> for Base {
  fn transcribe(&self) -> rna::Base {
    match self {
      Self::A => rna::Base::A,
      Self::C => rna::Base::C,
      Self::G => rna::Base::G,
      Self::T => rna::Base::U,
      Self::GAP => rna::Base::GAP,
    }
  }
}

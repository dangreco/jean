use crate::{
  prelude::{Complement, Transcribe},
  sequence::dna,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
  A,
  C,
  G,
  U,
  GAP,
}

impl TryFrom<char> for Base {
  type Error = String;

  fn try_from(c: char) -> Result<Self, Self::Error> {
    match c.to_ascii_uppercase() {
      'A' => Ok(Self::A),
      'C' => Ok(Self::C),
      'G' => Ok(Self::G),
      'U' => Ok(Self::U),
      '-' => Ok(Self::GAP),
      _ => Err(format!("Invalid RNA base '{c}'.")),
    }
  }
}

impl From<Base> for char {
  fn from(base: Base) -> Self {
    match base {
      Base::A => 'A',
      Base::C => 'C',
      Base::G => 'G',
      Base::U => 'U',
      Base::GAP => '-',
    }
  }
}

impl Complement for Base {
  fn complement(&self) -> Self {
    match self {
      Self::A => Self::U,
      Self::C => Self::G,
      Self::G => Self::C,
      Self::U => Self::A,
      Self::GAP => Self::GAP,
    }
  }
}

impl Transcribe<dna::Base> for Base {
  fn transcribe(&self) -> dna::Base {
    match self {
      Self::A => dna::Base::A,
      Self::C => dna::Base::C,
      Self::G => dna::Base::G,
      Self::U => dna::Base::T,
      Self::GAP => dna::Base::GAP,
    }
  }
}

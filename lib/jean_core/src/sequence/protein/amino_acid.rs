use num_enum::{FromPrimitive, IntoPrimitive};

use crate::prelude::Gap;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoPrimitive, FromPrimitive)]
pub enum AminoAcid {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  Y,
  Z,
  X,
  TSTOP,

  #[default]
  GAP,
}

impl Gap for AminoAcid {
  fn gap() -> Self {
    Self::GAP
  }
}

impl TryFrom<char> for AminoAcid {
  type Error = String;

  fn try_from(c: char) -> Result<Self, Self::Error> {
    match c.to_ascii_uppercase() {
      'A' => Ok(Self::A),
      'B' => Ok(Self::B),
      'C' => Ok(Self::C),
      'D' => Ok(Self::D),
      'E' => Ok(Self::E),
      'F' => Ok(Self::F),
      'G' => Ok(Self::G),
      'H' => Ok(Self::H),
      'I' => Ok(Self::I),
      'J' => Ok(Self::J),
      'K' => Ok(Self::K),
      'L' => Ok(Self::L),
      'M' => Ok(Self::M),
      'N' => Ok(Self::N),
      'O' => Ok(Self::O),
      'P' => Ok(Self::P),
      'Q' => Ok(Self::Q),
      'R' => Ok(Self::R),
      'S' => Ok(Self::S),
      'T' => Ok(Self::T),
      'U' => Ok(Self::U),
      'V' => Ok(Self::V),
      'W' => Ok(Self::W),
      'Y' => Ok(Self::Y),
      'Z' => Ok(Self::Z),
      'X' => Ok(Self::X),
      '*' => Ok(Self::TSTOP),
      '-' => Ok(Self::GAP),
      _ => Err(format!("Invalid amino acid '{c}'.")),
    }
  }
}

impl From<AminoAcid> for char {
  fn from(amino_acid: AminoAcid) -> Self {
    match amino_acid {
      AminoAcid::A => 'A',
      AminoAcid::B => 'B',
      AminoAcid::C => 'C',
      AminoAcid::D => 'D',
      AminoAcid::E => 'E',
      AminoAcid::F => 'F',
      AminoAcid::G => 'G',
      AminoAcid::H => 'H',
      AminoAcid::I => 'I',
      AminoAcid::J => 'J',
      AminoAcid::K => 'K',
      AminoAcid::L => 'L',
      AminoAcid::M => 'M',
      AminoAcid::N => 'N',
      AminoAcid::O => 'O',
      AminoAcid::P => 'P',
      AminoAcid::Q => 'Q',
      AminoAcid::R => 'R',
      AminoAcid::S => 'S',
      AminoAcid::T => 'T',
      AminoAcid::U => 'U',
      AminoAcid::V => 'V',
      AminoAcid::W => 'W',
      AminoAcid::Y => 'Y',
      AminoAcid::Z => 'Z',
      AminoAcid::X => 'X',
      AminoAcid::TSTOP => '*',
      AminoAcid::GAP => '-',
    }
  }
}

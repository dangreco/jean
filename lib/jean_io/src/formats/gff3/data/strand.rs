#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strand {
  Forward,
  Reverse,
}

impl std::fmt::Display for Strand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let c = match self {
      Self::Forward => '+',
      Self::Reverse => '-',
    };

    write!(f, "{c}")
  }
}

impl std::str::FromStr for Strand {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim() {
      "+" => Ok(Self::Forward),
      "-" => Ok(Self::Forward),
      _ => Err(format!("Invalid strand '{s}'.")),
    }
  }
}

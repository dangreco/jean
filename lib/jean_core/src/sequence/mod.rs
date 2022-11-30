mod seq;

/// DNA/RNA codons (e.g. AAA, TAG, ATG)
pub mod codon;

/// DNA sequences + bases
pub mod dna;

/// Protein sequences + amino acids
pub mod protein;

/// RNA sequences + bases
pub mod rna;

pub use seq::*;

/// Enum for differentiating between DNA, RNA, and Protein sequences
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sequence {
  Dna(Seq<dna::Base>),
  Rna(Seq<rna::Base>),
  Protein(Seq<protein::AminoAcid>),
}

impl TryFrom<&str> for Sequence {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match dna::Dna::try_from(value) {
      Ok(dna) => Ok(Sequence::Dna(dna)),
      Err(_) => match rna::Rna::try_from(value) {
        Ok(rna) => Ok(Sequence::Rna(rna)),
        Err(_) => match protein::Protein::try_from(value) {
          Ok(protein) => Ok(Sequence::Protein(protein)),
          Err(_) => Err(format!("Invalid sequence.")),
        },
      },
    }
  }
}

impl std::fmt::Display for Sequence {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Self::Dna(dna) => dna.to_string(),
      Self::Rna(rna) => rna.to_string(),
      Self::Protein(protein) => protein.to_string(),
    };

    s.chars()
      .collect::<Vec<char>>()
      .chunks(80)
      .map(|c| {
        c.iter()
          .map(|c| write!(f, "{c}"))
          .collect::<std::fmt::Result>()?;
        writeln!(f)
      })
      .collect::<std::fmt::Result>()
  }
}

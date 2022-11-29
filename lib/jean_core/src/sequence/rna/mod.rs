mod base;

pub use base::*;

use crate::prelude::{Codons, Transcribe, Translate};

use super::{
  codon::Codon,
  dna::{self, Dna},
  protein::{AminoAcid, Protein},
  Seq,
};

pub type Rna = Seq<Base>;

impl Rna {
  pub fn rev_transcribe(&self) -> Dna {
    Dna::from(
      &self
        .0
        .iter()
        .map(|b| b.transcribe())
        .collect::<Vec<dna::Base>>()[..],
    )
  }
}

impl Codons for Rna {
  fn codons(&self) -> Seq<super::codon::Codon> {
    Seq(
      self
        .0
        .chunks_exact(3)
        .map(|chunk| {
          Codon::new((
            char::from(chunk[0]) as u8,
            char::from(chunk[1]) as u8,
            char::from(chunk[2]) as u8,
          ))
        })
        .collect(),
    )
  }
}

impl Translate<Protein> for Rna {
  fn translate(&self, table_id: &str) -> Result<Protein, String> {
    let protein = self
      .codons()
      .iter()
      .map(|codon| codon.translate(table_id))
      .collect::<Result<Vec<AminoAcid>, String>>();

    Ok(Seq(protein?))
  }
}

#[cfg(test)]
mod tests {
  use crate::prelude::Translate;

  use super::Rna;

  #[test]
  fn test_1() {
    let rna = Rna::try_from("AAGUACUUUUUAAUAGGGGAGAUUUGUCUGUUCCUGCUGGAUUUACUAACAUGAGUGCCUCUGUGACCUACGUACUUAAUUAUCUCCCUCCUCUGGCAGAGC").unwrap();
    let protein = rna.translate("1");

    assert!(protein.is_ok());
    assert_eq!(
      protein.unwrap().to_string(),
      "KYFLIGEICLFLLDLLT*VPL*PTYLIISLLWQS".to_string()
    );
  }
}

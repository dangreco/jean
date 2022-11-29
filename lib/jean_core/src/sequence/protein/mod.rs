mod amino_acid;

pub use amino_acid::*;

use crate::data::cut::Cut;

use super::{rna::Rna, Seq};

pub type Protein = Seq<AminoAcid>;

impl Protein {
  pub fn rev_translate_probabilistic(&self, table_id: &str) -> Result<Rna, String> {
    let cut = Cut::get(table_id)?;
    let mut sequence = Vec::new();

    for amino_acid in self {
      let codon = cut
        .rev_translate(amino_acid)?
        .iter()
        .max_by(|a, b| {
          cut
            .fraction(a)
            .unwrap()
            .partial_cmp(&cut.fraction(b).unwrap())
            .unwrap()
        })
        .unwrap();

      let mut rna = Rna::try_from(*codon).unwrap();

      sequence.append(&mut rna.0);
    }

    Ok(Seq(sequence))
  }
}

#[cfg(test)]
mod tests {
  use super::Protein;

  #[test]
  fn test_1() {
    let protein = Protein::try_from("ACDEFGHIKL").unwrap();
    let rna = protein.rev_translate_probabilistic("1");

    assert!(rna.is_ok());
    assert_eq!(
      rna.unwrap().to_string(),
      "GCCUGCGACGAGUUCGGCCACAUCAAGCUG".to_string()
    );
  }
}

use crate::cut::Cut;
use jean_core::{
  prelude::Codons,
  sequence::{
    codon::Codon,
    protein::{AminoAcid, Protein},
    rna::{Base, Rna},
  },
};

pub trait Translate<T> {
  fn translate(&self, cut: &Cut) -> T;
}

impl Translate<AminoAcid> for Codon {
  fn translate(&self, cut: &Cut) -> AminoAcid {
    cut.amino_acid(&self)
  }
}

impl Translate<Protein> for Rna {
  fn translate(&self, cut: &Cut) -> Protein {
    Protein::from(
      self
        .codons()
        .iter()
        .map(|c| c.translate(cut))
        .collect::<Vec<AminoAcid>>(),
    )
  }
}

pub trait RevTranslate<T> {
  fn rev_translate(&self, cut: &Cut) -> T;
}

impl RevTranslate<Codon> for AminoAcid {
  fn rev_translate(&self, cut: &Cut) -> Codon {
    let codons = cut.codons(&self);
    let codon = codons
      .iter()
      .max_by(|a, b| cut.fraction(a).partial_cmp(&cut.fraction(b)).unwrap())
      .unwrap();

    *codon
  }
}

impl RevTranslate<Rna> for Protein {
  fn rev_translate(&self, cut: &Cut) -> Rna {
    Rna::from(
      self
        .iter()
        .map(|aa| aa.rev_translate(cut))
        .map(|codon| Rna::try_from(codon).unwrap().0)
        .flatten()
        .collect::<Vec<Base>>(),
    )
  }
}

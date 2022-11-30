mod base;

pub use base::*;

use crate::prelude::{Codons, Transcribe};

use super::{
  codon::Codon,
  dna::{self, Dna},
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
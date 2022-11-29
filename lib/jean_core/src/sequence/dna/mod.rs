mod base;

pub use base::*;

use crate::prelude::{Codons, Transcribe};

use super::{
  codon::Codon,
  rna::{self, Rna},
  Seq,
};

pub type Dna = Seq<Base>;

impl Dna {}

impl Transcribe<Rna> for Dna {
  fn transcribe(&self) -> Rna {
    Rna::from(
      &self
        .0
        .iter()
        .map(|b| b.transcribe())
        .collect::<Vec<rna::Base>>()[..],
    )
  }
}

impl Codons for Dna {
  fn codons(&self) -> Seq<super::codon::Codon> {
    Seq(
      self
        .0
        .chunks_exact(3)
        .map(|chunk| Codon::new((chunk[0] as u8, chunk[1] as u8, chunk[2] as u8)))
        .collect(),
    )
  }
}

#[cfg(test)]
mod tests {

  use crate::{
    prelude::Transcribe,
    sequence::rna::{self, Rna},
  };

  use super::{Base, Dna};

  #[test]
  fn test_1() {
    let dna = Dna::from(&[Base::A, Base::C, Base::G, Base::T, Base::GAP][..]);
    let rna = dna.transcribe();

    assert_eq!(
      rna,
      Rna::from(
        &[
          rna::Base::A,
          rna::Base::C,
          rna::Base::G,
          rna::Base::U,
          rna::Base::GAP
        ][..]
      )
    );

    println!("{}", dna);
    println!("{}", rna);
  }
}

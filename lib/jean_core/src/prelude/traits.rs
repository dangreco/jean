use crate::sequence::{codon::Codon, Seq};

pub trait Complement {
  fn complement(&self) -> Self;
}

pub trait Transcribe<T> {
  fn transcribe(&self) -> T;
}

pub trait Codons {
  fn codons(&self) -> Seq<Codon>;
}

pub trait Gap {
  fn gap() -> Self;
}

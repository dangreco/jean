use crate::sequence::{codon::Codon, Seq};

pub trait Complement {
  fn complement(&self) -> Self;
}

pub trait Transcribe<T> {
  fn transcribe(&self) -> T;
}

pub trait Translate<T> {
  fn translate(&self, table_id: &str) -> Result<T, String>;
}

pub trait Codons {
  fn codons(&self) -> Seq<Codon>;
}

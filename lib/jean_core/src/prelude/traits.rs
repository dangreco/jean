use std::{
  error::Error,
  io::{Read, Write},
};

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

pub trait SequenceFile {
  fn read<F>(f: &mut F) -> Result<Self, Box<dyn Error>>
  where
    F: Read,
    Self: Sized;

  fn write<F>(&self, f: &mut F) -> Result<(), Box<dyn Error>>
  where
    F: Write;
}

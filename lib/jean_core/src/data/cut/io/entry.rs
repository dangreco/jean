use crate::sequence::{codon::Codon, protein::AminoAcid};

#[derive(Debug, Clone)]
pub struct Entry {
  pub codon: Codon,
  pub amino_acid: AminoAcid,
  pub fraction: Option<f64>,
  pub frequency: Option<f64>,
}

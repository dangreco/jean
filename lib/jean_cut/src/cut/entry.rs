use jean_core::sequence::protein::AminoAcid;

#[derive(Debug, Clone)]
pub struct Entry {
  pub amino_acid: AminoAcid,
  pub fraction: Option<f64>,
  pub frequency: Option<f64>,
  pub number: Option<u128>,
}

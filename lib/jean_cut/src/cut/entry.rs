use jean_core::sequence::protein::AminoAcid;

#[derive(Debug, Clone)]
pub struct Entry {
  pub amino_acid: AminoAcid,
  pub fraction: Option<f64>,
  pub frequency: Option<f64>,
  pub number: Option<u128>,
}

impl std::fmt::Display for Entry {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}\t\t{:.5}\t\t{:.5}\t\t{}",
      self.amino_acid,
      self
        .fraction
        .map(|f| f.to_string())
        .unwrap_or(".".to_string()),
      self
        .frequency
        .map(|f| f.to_string())
        .unwrap_or(".".to_string()),
      self
        .number
        .map(|f| f.to_string())
        .unwrap_or(".".to_string()),
    )
  }
}

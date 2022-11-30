/// Codon from string literal
#[macro_export]
macro_rules! codon {
  ($l:literal) => {{
    jean::codon::Codon::try_from($l).unwrap()
  }};
}

/// `Dna` from string literal
#[macro_export]
macro_rules! dna {
  ($l:literal) => {{
    jean::dna::Dna::try_from($l).unwrap()
  }};
}

/// `Protein` from string literal
#[macro_export]
macro_rules! protein {
  ($l:literal) => {{
    jean::protein::Protein::try_from($l).unwrap()
  }};
}

/// `Rna` from string literal
#[macro_export]
macro_rules! rna {
  ($l:literal) => {{
    jean::rna::Rna::try_from($l).unwrap()
  }};
}

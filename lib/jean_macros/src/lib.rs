#[macro_export]
macro_rules! codon {
  ($l:literal) => {{
    jean_core::sequence::codon::Codon::try_from($l).unwrap()
  }};
}

#[macro_export]
macro_rules! dna {
  ($l:literal) => {{
    jean_core::sequence::dna::Dna::try_from($l).unwrap()
  }};
}

#[macro_export]
macro_rules! protein {
  ($l:literal) => {{
    jean_core::sequence::protein::Protein::try_from($l).unwrap()
  }};
}

#[macro_export]
macro_rules! rna {
  ($l:literal) => {{
    jean_core::sequence::rna::Rna::try_from($l).unwrap()
  }};
}

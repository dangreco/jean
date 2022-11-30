mod amino_acid;

pub use amino_acid::*;

use super::Seq;

pub type Protein = Seq<AminoAcid>;

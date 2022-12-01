#[macro_use]
extern crate jean;

use jean::{alignment::global::NeedlemanWunsch, blosum::NUC_4_2, dna::Dna};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let seq1: Dna = dna!("AGACTAGTTAC");
  let seq2: Dna = dna!("CGAGACGT");

  let alignment = NeedlemanWunsch::new()
    .matrix(&NUC_4_2)
    .gap_penalty(|k| -2 * k)
    .align(&seq1, &seq2)?;

  assert_eq!(alignment.score, 16);
  assert_eq!(alignment.a, dna!("--AGACTAGTTAC"));
  assert_eq!(alignment.b, dna!("CGAGAC--G-T--"));

  Ok(())
}

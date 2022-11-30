#[macro_use]
extern crate jean;

use jean::{alignment::NeedlemanWunsch, dna::Dna};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let seq1: Dna = dna!("AGACTAGTTAC");
  let seq2: Dna = dna!("CGAGACGT");

  let (score, alignment) = NeedlemanWunsch::new()
    .similarity_matrix(vec![
      vec![5, -5, -5, -5],
      vec![-5, 5, -5, -5],
      vec![-5, -5, 5, -5],
      vec![-5, -5, -5, 5],
    ])
    .gap_penalty(-2)
    .align(&seq1, &seq2);

  assert_eq!(score, 16);
  assert_eq!(alignment.a(), &dna!("--AGACTAGTTAC"));
  assert_eq!(alignment.b(), &dna!("CGAGAC--G-T--"));

  Ok(())
}

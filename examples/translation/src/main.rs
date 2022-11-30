#[macro_use]
extern crate jean;

use jean::{cut::Cut, dna::Dna, prelude::*, protein::Protein, rna::Rna};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let path = std::path::PathBuf::from("homo_sapien.cut");
  let cut = Cut::read_file(&path)?;

  /* DNA -> RNA -> Protein */
  let d: Dna = dna!("AGGCTGGGCACC");
  let r: Rna = d.transcribe();
  let p: Protein = r.translate(&cut);

  assert_eq!(p, protein!("RLGT"));

  /* ...and back again */
  let r_ = p.rev_translate(&cut);
  let d_ = r_.rev_transcribe();

  assert_eq!(d, d_);

  Ok(())
}

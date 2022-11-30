#[macro_use]
extern crate jean;

use std::path::PathBuf;
use anyhow::Result;

use jean::Transcribe;
use jean_cut::prelude::*;
use jean_cut::cut::Cut;

#[test]
pub fn test_1() -> Result<()>
{
  let path = PathBuf::from("tests/homo_sapiens.cut");
  let cut = Cut::read_file(&path)?;
  
  /* Protein -> RNA -> DNA */
  let p = protein!("MRDTKQWDFIQAMLSVMRAMNLARILRKAIQAESEKEVGAHVNSLDFIEK");
  let r = p.rev_translate(&cut);
  let d = r.rev_transcribe();

  assert_eq!(p.len() * 3, d.len());

  /* DNA -> RNA -> Protein */
  let r_ = d.transcribe();
  let p_ = r_.translate(&cut);

  assert_eq!(p, p_);
  
  Ok(())
}
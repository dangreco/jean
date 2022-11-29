use std::path::PathBuf;

use jean_io::fasta::Fasta;

#[test]
fn test_read() {
  let path = PathBuf::from("tests/fasta/Vibrio_vulnificus.fa");
  let fasta = Fasta::read_file(&path);
  assert!(fasta.is_ok());
}
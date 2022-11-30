use std::path::PathBuf;

use jean_io::formats::gff3::Gff3;

#[test]
fn test_read() {
  let path = PathBuf::from("tests/gff3/Vibrio_vulnificus.gff3");
  let gff3 = Gff3::read_file(&path);
  assert!(gff3.is_ok());
}

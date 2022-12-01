use std::{
  collections::{
    hash_map::{Iter, IterMut},
    HashMap,
  },
  fs::File,
};

use anyhow::Result;

use self::parser::FastaParser;

mod entry;
mod lexer;
mod parser;
mod token;

pub use entry::Entry;

#[derive(Debug, Clone)]
pub struct Fasta(HashMap<String, Entry>);

impl Fasta {
  pub fn new() -> Self {
    Self(HashMap::new())
  }

  pub fn read_str(str: &str) -> Result<Self> {
    let entries = FastaParser::parse_str(str)?;
    Ok(Self(entries))
  }

  pub fn read<F>(f: &mut F) -> Result<Self>
  where
    F: std::io::Read,
  {
    let mut str = String::new();
    f.read_to_string(&mut str)?;
    Self::read_str(str.as_str())
  }

  pub fn read_file<P>(path: P) -> Result<Self>
  where
    P: AsRef<std::path::Path>,
  {
    let mut file = File::open(path)?;
    Self::read(&mut file)
  }

  pub fn write<F>(&self, f: &mut F) -> Result<()>
  where
    F: std::io::Write,
  {
    self
      .0
      .iter()
      .map(|(id, entry)| {
        writeln!(f, ">{id} {}", entry.metadata)?;
        writeln!(f, "{}", entry.sequence)?;
        Ok(())
      })
      .collect::<Result<()>>()
  }

  pub fn insert(&mut self, id: &str, entry: Entry) -> Option<Entry> {
    self.0.insert(id.to_string(), entry)
  }

  pub fn remove(&mut self, id: &str) -> Option<Entry> {
    self.0.remove(&id.to_string())
  }

  pub fn get(&mut self, id: &str) -> Option<&Entry> {
    self.0.get(&id.to_string())
  }

  pub fn iter(&self) -> Iter<String, Entry> {
    self.0.iter()
  }

  pub fn iter_mut(&mut self) -> IterMut<String, Entry> {
    self.0.iter_mut()
  }
}

impl std::ops::Index<&str> for Fasta {
  type Output = Entry;
  fn index(&self, index: &str) -> &Self::Output {
    &self.0[index]
  }
}

impl IntoIterator for Fasta {
  type Item = (String, Entry);
  type IntoIter = <HashMap<String, Entry> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a Fasta {
  type Item = (&'a String, &'a Entry);
  type IntoIter = Iter<'a, String, Entry>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut Fasta {
  type Item = (&'a String, &'a mut Entry);
  type IntoIter = IterMut<'a, String, Entry>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

#[cfg(test)]
mod tests {
  use std::io::stdout;

  use super::Fasta;

  static INPUT: &str = r#">seq1
MADQLTEEQIAEFKEAFSLFDKDGDGTITTKELGTVMRSLGQNPTEAELQDMINEVDADGNGTID
FPEFLTMMARKMKDTDSEEEIREAFRVFDKDGNGYISAAELRHVMTNLGEKLTDEEVDEMIREA
DIDGDGQVNYEEFVQMMTAK*
  
>seq2
LCLYTHIGRNIYYGSYLYSETWNTGIMLLLITMATAFMGYVLPWGQMSFWGATVITNLFSAIPYIGTNLV
EWIWGGFSVDKATLNRFFAFHFILPFTMVALAGVHLTFLHETGSNNPLGLTSDSDKIPFHPYYTIKDFLG
LLILILLLLLLALLSPDMLGDPDNHMPADPLNTPLHIKPEWYFLFAYAILRSVPNKLGGVLALFLSIVIL
GLMPFLHTSKHRSMMLRPLSQALFWTLTMDLLTLTWIGSQPVEYPYTIIGQMASILYFSIILAFLPIAGX
IENY"#;

  #[test]
  fn test_read() {
    let fasta = Fasta::read_str(INPUT);
    assert!(fasta.is_ok());
  }

  #[test]
  fn test_write() {
    let fasta = {
      let fasta = Fasta::read_str(INPUT);
      assert!(fasta.is_ok());
      fasta.unwrap()
    };

    let mut output = stdout();

    assert!(fasta.write(&mut output).is_ok());
  }
}

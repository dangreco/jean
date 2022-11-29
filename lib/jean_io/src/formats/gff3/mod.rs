mod data;
mod lexer;
mod parser;

use std::{
  collections::{
    btree_map::{Iter, IterMut},
    BTreeMap,
  },
  fs::File,
  path::PathBuf,
};

use anyhow::Result;
pub use data::*;

use self::parser::Gff3Parser;

#[derive(Debug, Clone)]
pub struct Gff3(BTreeMap<String, Vec<Entry>>);

impl Gff3 {
  pub fn new() -> Self {
    Self(BTreeMap::new())
  }

  pub fn read_str(str: &str) -> Result<Self> {
    let entries = Gff3Parser::parse_str(str)?;
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

  pub fn read_file(path: &PathBuf) -> Result<Self> {
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
      .map(|(_, entries)| {
        entries
          .iter()
          .map(|entry| write!(f, "{entry}").map_err(anyhow::Error::from))
          .collect::<Result<()>>()?;
        writeln!(f, "##").map_err(anyhow::Error::from)
      })
      .collect::<Result<()>>()
  }

  pub fn insert(&mut self, entry: Entry) {
    let e = self.0.entry(entry.id.clone()).or_insert(Vec::new());

    let mut i = 0;

    while i < e.len() && e[i].start < entry.start {
      i += 1;
    }

    e.insert(i, entry)
  }

  pub fn remove(&mut self, entry: &Entry) {
    let e = self.0.entry(entry.id.clone()).or_insert(Vec::new());
    e.retain(|e| e != entry)
  }

  pub fn get(&mut self, id: &String) -> Option<&Vec<Entry>> {
    self.0.get(id)
  }

  pub fn iter(&self) -> Iter<String, Vec<Entry>> {
    self.0.iter()
  }

  pub fn iter_mut(&mut self) -> IterMut<String, Vec<Entry>> {
    self.0.iter_mut()
  }
}

impl std::ops::Index<&str> for Gff3 {
  type Output = Vec<Entry>;

  fn index(&self, index: &str) -> &Self::Output {
    &self.0[index]
  }
}

impl IntoIterator for Gff3 {
  type Item = (String, Vec<Entry>);
  type IntoIter = <BTreeMap<String, Vec<Entry>> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a Gff3 {
  type Item = (&'a String, &'a Vec<Entry>);
  type IntoIter = Iter<'a, String, Vec<Entry>>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut Gff3 {
  type Item = (&'a String, &'a mut Vec<Entry>);
  type IntoIter = IterMut<'a, String, Vec<Entry>>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

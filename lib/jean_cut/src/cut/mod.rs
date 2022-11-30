use std::{
  collections::{BTreeMap, BTreeSet},
  path::PathBuf,
};

use anyhow::Result;
use jean_core::sequence::{codon::Codon, protein::AminoAcid};

use self::parser::CutParser;

mod entry;
mod lexer;
mod parser;

pub use entry::Entry;

#[derive(Debug, Clone)]
pub struct Cut {
  entries: BTreeMap<Codon, Entry>,
  codons: BTreeMap<AminoAcid, BTreeSet<Codon>>,
}

impl Cut {
  pub fn new() -> Self {
    Self {
      entries: BTreeMap::new(),
      codons: BTreeMap::new(),
    }
  }

  pub fn insert(&mut self, codon: &Codon, entry: Entry) {
    /* Remove from AA -> [Codon] */
    self.codons.iter_mut().for_each(|(_, codons)| {
      codons.remove(codon);
    });

    /* Add to AA -> [Codon] */
    let e = self
      .codons
      .entry(entry.amino_acid)
      .or_insert(BTreeSet::new());
    e.insert(*codon);

    /* Add entry */
    self.entries.insert(*codon, entry);
  }

  pub fn read_str(str: &str) -> Result<Self> {
    let entries: BTreeMap<Codon, Entry> = CutParser::parse_str(str)?
      .iter()
      .map(|(codon, entry)| (*codon, entry.clone()))
      .collect();

    let mut codons: BTreeMap<AminoAcid, BTreeSet<Codon>> = BTreeMap::new();

    entries.iter().for_each(|(codon, entry)| {
      let e = codons.entry(entry.amino_acid).or_insert(BTreeSet::new());
      e.insert(*codon);
    });

    Ok(Self { entries, codons })
  }

  pub fn read<F>(f: &mut F) -> Result<Self>
  where
    F: std::io::Read,
  {
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Self::read_str(buf.as_str())
  }

  pub fn read_file(path: &PathBuf) -> Result<Self> {
    let mut file = std::fs::File::open(path)?;
    Self::read(&mut file)
  }

  pub fn write<F>(&self, f: &mut F) -> Result<()>
  where
    F: std::io::Write,
  {
    writeln!(f, "CODON\tAMINO ACID\tFRACTION\tFREQUENCY\tNUMBER")?;

    self
      .entries
      .iter()
      .map(|(codon, entry)| writeln!(f, "{codon}\t{entry}").map_err(|e| e.into()))
      .collect::<Result<()>>()
  }

  pub fn frequency(&self, codon: &Codon) -> Option<f64> {
    self.entries[codon].frequency
  }

  pub fn fraction(&self, codon: &Codon) -> Option<f64> {
    self.entries[codon].fraction
  }

  pub fn number(&self, codon: &Codon) -> Option<u128> {
    self.entries[codon].number
  }

  pub fn amino_acid(&self, codon: &Codon) -> AminoAcid {
    self.entries[codon].amino_acid
  }

  pub fn codons(&self, amino_acid: &AminoAcid) -> &BTreeSet<Codon> {
    &self.codons[amino_acid]
  }
}

#[cfg(test)]
mod tests {
  use std::io::stdout;

  use anyhow::{anyhow, Result};
  use jean_core::sequence::{codon::Codon, protein::AminoAcid};

  use super::Cut;

  #[test]
  fn test1() -> Result<()> {
    let str = "AAA A 1.0 1.0 100\nCCC B . . .\nGGG A 3.0 3.0 300\nTTT B 4.0 4.0 400";
    let cut = Cut::read_str(str)?;

    let aaa = Codon::try_from("AAA").map_err(|s| anyhow!(s))?;
    let ccc = Codon::try_from("CCC").map_err(|s| anyhow!(s))?;
    let ggg = Codon::try_from("GGG").map_err(|s| anyhow!(s))?;
    let ttt = Codon::try_from("TTT").map_err(|s| anyhow!(s))?;

    assert_eq!(cut.fraction(&aaa), Some(1.0));
    assert_eq!(cut.frequency(&aaa), Some(1.0));
    assert_eq!(cut.number(&aaa), Some(100));

    assert_eq!(cut.fraction(&ccc), None);
    assert_eq!(cut.frequency(&ccc), None);
    assert_eq!(cut.number(&ccc), None);

    assert_eq!(cut.fraction(&ggg), Some(3.0));
    assert_eq!(cut.frequency(&ggg), Some(3.0));
    assert_eq!(cut.number(&ggg), Some(300));

    assert_eq!(cut.fraction(&ttt), Some(4.0));
    assert_eq!(cut.frequency(&ttt), Some(4.0));
    assert_eq!(cut.number(&ttt), Some(400));

    assert!(cut.codons(&AminoAcid::A).contains(&aaa));
    assert!(cut.codons(&AminoAcid::A).contains(&ggg));

    assert!(cut.codons(&AminoAcid::B).contains(&ccc));
    assert!(cut.codons(&AminoAcid::B).contains(&ttt));

    cut.write(&mut stdout())
  }
}

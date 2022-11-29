use std::collections::{BTreeMap, BTreeSet};

use crate::sequence::{codon::Codon, protein::AminoAcid};

use self::io::{entry::Entry, lexer::Lexer, parser::Parser};

mod io;
mod tables;

use tables::*;

#[derive(Debug, Clone)]
pub struct Cut {
  map: BTreeMap<Codon, Entry>,
  lookup: BTreeMap<AminoAcid, BTreeSet<Codon>>,
}

#[allow(dead_code)]
impl Cut {
  pub fn new() -> Self {
    Self {
      map: BTreeMap::new(),
      lookup: BTreeMap::new(),
    }
  }

  pub fn get(id: &str) -> Result<&'static Cut, String> {
    match id {
      "1" => Ok(&TABLE_1),
      "2" => Ok(&TABLE_2),
      "3" => Ok(&TABLE_3),
      "4" => Ok(&TABLE_4),
      "4a" => Ok(&TABLE_4A),
      "5" => Ok(&TABLE_5),
      "6" => Ok(&TABLE_6),
      "9" => Ok(&TABLE_9),
      "10" => Ok(&TABLE_10),
      "11" => Ok(&TABLE_11),
      "12" => Ok(&TABLE_12),
      "13" => Ok(&TABLE_13),
      "14" => Ok(&TABLE_14),
      "15" => Ok(&TABLE_15),
      _ => Err(format!("No table with ID '{id}' found.")),
    }
  }

  pub fn read_str(str: &str) -> Self {
    let tokens = Lexer::read_str(str);
    let entries = Parser::parse(&tokens);

    let mut cut = Self::new();

    for entry in entries {
      cut.insert(entry)
    }

    cut
  }

  pub fn insert(&mut self, entry: Entry) {
    match self.map.get(&entry.codon) {
      Some(e) => match self.lookup.get_mut(&e.amino_acid) {
        Some(set) => {
          set.remove(&entry.codon);
        }
        None => (),
      },
      None => (),
    }

    self
      .lookup
      .entry(entry.amino_acid)
      .or_insert(BTreeSet::new())
      .insert(entry.codon);

    self.map.insert(entry.codon, entry);
  }

  pub fn translate(&self, codon: &Codon) -> Result<AminoAcid, String> {
    match self.map.get(codon) {
      Some(entry) => Ok(entry.amino_acid),
      None => Err(format!("No amino acid for specified codon '{}'", codon)),
    }
  }

  pub fn rev_translate(&self, amino_acid: &AminoAcid) -> Result<&BTreeSet<Codon>, String> {
    match self.lookup.get(amino_acid) {
      Some(set) => Ok(set),
      None => Err(format!("Amino acid has no corresponding codons.")),
    }
  }

  pub fn fraction(&self, codon: &Codon) -> Option<f64> {
    match self.map.get(codon) {
      Some(entry) => entry.fraction,
      None => None,
    }
  }

  pub fn frequency(&self, codon: &Codon) -> Option<f64> {
    match self.map.get(codon) {
      Some(entry) => entry.frequency,
      None => None,
    }
  }
}

use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
use jean_core::sequence::{codon::Codon, protein::AminoAcid};

use super::{entry::Entry, parser::CutParser};

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

  pub fn read_file<P>(path: P) -> Result<Self>
  where
    P: AsRef<std::path::Path>,
  {
    let mut file = std::fs::File::open(path)?;
    Self::read(&mut file)
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
    assert_eq!(cut.frequency(&aaa), Some(1000.0));
    assert_eq!(cut.number(&aaa), Some(100));

    assert_eq!(cut.fraction(&ccc), None);
    assert_eq!(cut.frequency(&ccc), None);
    assert_eq!(cut.number(&ccc), None);

    assert_eq!(cut.fraction(&ggg), Some(3.0));
    assert_eq!(cut.frequency(&ggg), Some(3000.0));
    assert_eq!(cut.number(&ggg), Some(300));

    assert_eq!(cut.fraction(&ttt), Some(4.0));
    assert_eq!(cut.frequency(&ttt), Some(4000.0));
    assert_eq!(cut.number(&ttt), Some(400));

    assert!(cut.codons(&AminoAcid::A).contains(&aaa));
    assert!(cut.codons(&AminoAcid::A).contains(&ggg));

    assert!(cut.codons(&AminoAcid::B).contains(&ccc));
    assert!(cut.codons(&AminoAcid::B).contains(&ttt));

    Ok(())
  }

  #[test]
  fn test_2() {
    let str = r#"
UUU F 0.66 31.9 (  3852)  UCU S 0.24 22.0 (  2652)  UAU Y 0.58 19.1 (  2308)  UGU C 0.64 16.6 (  2000)
UUC F 0.34 16.2 (  1956)  UCC S 0.18 17.1 (  2062)  UAC Y 0.42 13.7 (  1654)  UGC C 0.36  9.3 (  1122)
UUA L 0.25 22.1 (  2674)  UCA S 0.18 16.6 (  2010)  UAA * 0.42  0.9 (   108)  UGA * 0.35  0.7 (    89)
UUG L 0.20 17.5 (  2107)  UCG S 0.11  9.7 (  1173)  UAG * 0.23  0.5 (    58)  UGG W 1.00  7.6 (   918)

CUU L 0.13 11.4 (  1380)  CCU P 0.29 13.1 (  1577)  CAU H 0.53 13.7 (  1654)  CGU R 0.12  6.1 (   740)
CUC L 0.13 11.5 (  1394)  CCC P 0.22  9.9 (  1201)  CAC H 0.47 11.9 (  1441)  CGC R 0.11  5.5 (   670)
CUA L 0.15 13.4 (  1622)  CCA P 0.26 11.5 (  1391)  CAA Q 0.56 19.8 (  2390)  CGA R 0.13  6.6 (   797)
CUG L 0.15 13.4 (  1617)  CCG P 0.23 10.2 (  1229)  CAG Q 0.44 15.6 (  1884)  CGG R 0.09  4.6 (   559)

AUU I 0.37 24.1 (  2913)  ACU T 0.27 17.3 (  2093)  AAU N 0.56 35.1 (  4242)  AGU S 0.16 14.4 (  1735)
AUC I 0.32 20.7 (  2498)  ACC T 0.23 15.0 (  1812)  AAC N 0.44 27.3 (  3301)  AGC S 0.14 12.5 (  1515)
AUA I 0.32 20.8 (  2508)  ACA T 0.30 19.8 (  2391)  AAA K 0.74 49.8 (  6017)  AGA R 0.35 17.1 (  2069)
AUG M 1.00 25.2 (  3040)  ACG T 0.20 12.9 (  1559)  AAG K 0.26 17.7 (  2140)  AGG R 0.19  9.2 (  1116)

GUU V 0.31 18.2 (  2198)  GCU A 0.29 13.9 (  1684)  GAU D 0.61 33.2 (  4006)  GGU G 0.26 10.6 (  1282)
GUC V 0.20 11.7 (  1418)  GCC A 0.27 13.3 (  1601)  GAC D 0.39 20.8 (  2512)  GGC G 0.22  9.0 (  1092)
GUA V 0.22 12.8 (  1547)  GCA A 0.25 12.3 (  1482)  GAA E 0.70 41.3 (  4985)  GGA G 0.33 13.2 (  1595)
GUG V 0.27 15.6 (  1881)  GCG A 0.19  9.3 (  1126)  GAG E 0.30 18.0 (  2172)  GGG G 0.19  7.6 (   914)
"#;

    let cut = Cut::read_str(str);

    assert!(cut.is_ok());

    println!("{:?}", cut);
  }
}

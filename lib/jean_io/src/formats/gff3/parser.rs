use std::{
  collections::{BTreeMap, HashMap},
  str::FromStr,
};

use anyhow::Result;

use crate::utils::{cast, expand};

use super::{
  lexer::{Gff3Lexer, Rule},
  Entry, Strand,
};

#[derive(Debug, Clone)]
pub struct Gff3Parser;

impl Gff3Parser {
  pub fn parse_str(str: &str) -> Result<BTreeMap<String, Vec<Entry>>> {
    let mut entries = BTreeMap::new();

    let mut lexed = Gff3Lexer::lex(str)?;
    let inner = lexed.next().unwrap().into_inner();

    inner.filter(|p| p.as_rule() == Rule::record).for_each(|p| {
      let inner = expand(&p);

      let id: String = cast(&inner[0]);

      let source: Option<String> = match inner[1].as_rule() {
        Rule::none => None,
        _ => Some(inner[1].as_str().to_string()),
      };

      let feature_type: String = cast(&inner[2]);

      let start: usize = cast(&inner[3]);
      let end: usize = cast(&inner[4]);

      let score: Option<f64> = match inner[5].as_rule() {
        Rule::float => Some(cast(&inner[5])),
        Rule::int => Some(cast::<u32, Rule>(&inner[5]) as f64),
        _ => None,
      };

      let strand: Option<Strand> = match Strand::from_str(inner[6].as_str()) {
        Ok(strand) => Some(strand),
        Err(_) => None,
      };

      let phase: Option<u8> = match inner[7].as_rule() {
        Rule::int => Some(cast(&inner[6])),
        _ => None,
      };

      let attributes: Option<HashMap<String, String>> = match inner[8].as_rule() {
        Rule::none => None,
        _ => {
          let str = inner[8].as_str();

          Some(
            str
              .split(";")
              .map(|s| {
                let i: Vec<&str> = s.split("=").collect();
                (i[0].to_string(), i[1].to_string())
              })
              .collect(),
          )
        }
      };

      let entry = Entry {
        id,
        source,
        feature_type,
        start,
        end,
        score,
        strand,
        phase,
        attributes,
      };

      let e: &mut Vec<Entry> = entries.entry(entry.id.clone()).or_insert(Vec::new());

      let mut i = 0;

      while i < e.len() && e[i].start < entry.start {
        i += 1;
      }

      e.insert(i, entry)
    });

    Ok(entries)
  }
}

#[cfg(test)]
mod tests {
  use super::Gff3Parser;

  #[test]
  fn test_1() {
    let gff = r#"##gff-version 3
ctg123 . mRNA            1300  9000  .  +  .  ID=mrna0001;Name=sonichedgehog
ctg123 . exon            1300  1500  .  +  .  ID=exon00001;Parent=mrna0001
ctg123 . exon            1050  1500  .  +  .  ID=exon00002;Parent=mrna0001
ctg123 . exon            3000  3902  .  +  .  ID=exon00003;Parent=mrna0001
ctg123 . exon            5000  5500  .  +  .  ID=exon00004;Parent=mrna0001
ctg123 . exon            7000  9000  .  +  .  ID=exon00005;Parent=mrna0001"#;

    let parsed = Gff3Parser::parse_str(gff);

    assert!(parsed.is_ok());
    assert_eq!(parsed.unwrap()["ctg123"].len(), 6);
  }
}

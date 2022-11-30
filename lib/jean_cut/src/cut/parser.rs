use anyhow::{anyhow, Context, Result};
use jean_core::sequence::{codon::Codon, protein::AminoAcid};
use pest::iterators::Pair;

use super::{
  entry::Entry,
  lexer::{CutLexer, Rule},
};

#[derive(Debug, Clone)]
pub struct CutParser;

#[allow(dead_code)]
enum Value {
  File(Vec<(Codon, Entry)>),
  Entry((Codon, Entry)),
  Codon(Codon),
  AminoAcid(AminoAcid),
  Float(f64),
  Int(u128),
  None,
  ERROR(Box<dyn std::error::Error>),
}

impl CutParser {
  fn parse<'a>(pair: Pair<'a, Rule>) -> Value {
    match pair.as_rule() {
      Rule::none => Value::None,
      Rule::int => match pair.as_str().parse::<u128>() {
        Ok(int) => Value::Int(int),
        Err(e) => Value::ERROR(e.into()),
      },
      Rule::float => match pair.as_str().parse::<f64>() {
        Ok(float) => Value::Float(float),
        Err(e) => Value::ERROR(e.into()),
      },
      Rule::amino_acid => match AminoAcid::try_from(pair.as_str()) {
        Ok(amino_acid) => Value::AminoAcid(amino_acid),
        Err(e) => Value::ERROR(e.into()),
      },
      Rule::codon => match Codon::try_from(pair.as_str()) {
        Ok(codon) => Value::Codon(codon),
        Err(e) => Value::ERROR(e.into()),
      },
      Rule::entry => {
        let inner: Vec<Pair<Rule>> = pair.into_inner().collect();
        let parsed: Vec<Value> = inner.iter().cloned().map(Self::parse).collect();

        let codon = match parsed[0] {
          Value::Codon(codon) => codon,
          _ => panic!("Invalid codon value: '{}'", inner[0].as_str()),
        };

        let amino_acid = match parsed[1] {
          Value::AminoAcid(amino_acid) => amino_acid,
          _ => panic!("Invalid amino_acid value: '{}'", inner[1].as_str()),
        };

        let fraction = match parsed[2] {
          Value::Float(fraction) => Some(fraction),
          Value::None => None,
          _ => panic!("Invalid fraction value: '{}'", inner[2].as_str()),
        };

        let frequency = match parsed[3] {
          Value::Float(frequency) => Some(frequency),
          Value::None => None,
          _ => panic!("Invalid frequency value: '{}'", inner[3].as_str()),
        };

        let number = match parsed[4] {
          Value::Int(number) => Some(number),
          Value::None => None,
          _ => panic!("Invalid number value: '{}'", inner[4].as_str()),
        };

        Value::Entry((
          codon,
          Entry {
            amino_acid,
            fraction,
            frequency,
            number,
          },
        ))
      }
      Rule::file => Value::File(
        pair
          .into_inner()
          .filter(|p| p.as_rule() == Rule::entry)
          .map(Self::parse)
          .map(|v| match v {
            Value::Entry(entry) => entry,
            _ => panic!("Invalid entry"),
          })
          .collect(),
      ),
      _ => panic!("Invalid rule: {:?}", pair.as_rule()),
    }
  }

  pub fn parse_str(str: &str) -> Result<Vec<(Codon, Entry)>> {
    let mut lexed = CutLexer::lex(str)?;
    let root = lexed.next().context("file root")?;

    match Self::parse(root) {
      Value::File(entries) => Ok(entries),
      _ => Err(anyhow!("Failed to parse.")),
    }
  }
}

#[cfg(test)]
mod tests {
  use anyhow::Result;

  use super::CutParser;

  #[test]
  fn test_1() -> Result<()> {
    let str = "CODON AMINO ACID FRACTION FREQUENCY NUMBER\nAAA A 1.0 1.0 100";
    let parsed = CutParser::parse_str(str);

    assert!(parsed.is_ok());
    assert_eq!(parsed.unwrap().len(), 1);

    Ok(())
  }
}

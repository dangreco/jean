use super::lexer::Rule;
use jean_core::sequence::protein::AminoAcid;
use pest::iterators::{Pair, Pairs};

pub struct BlosumParser;

#[derive(Debug)]
pub enum Value {
  Int(i32),
  Amino(AminoAcid),
  Header(Vec<Box<Value>>),
  Row(Box<Value>, Vec<Box<Value>>),
  File(Vec<Box<Value>>),
  Error,
}

impl Default for Value {
  fn default() -> Self {
    Self::Error
  }
}

impl BlosumParser {
  fn parse_rule<'a>(pair: Pair<'a, Rule>) -> Value {
    match pair.as_rule() {
      Rule::int => pair
        .as_str()
        .parse::<i32>()
        .map(|n| Value::Int(n))
        .unwrap_or_default(),
      Rule::amino => AminoAcid::try_from(pair.as_str())
        .map(|a| Value::Amino(a))
        .unwrap_or_default(),
      Rule::header => Value::Header(
        pair
          .into_inner()
          .map(Self::parse_rule)
          .map(|v| v.into())
          .collect(),
      ),
      Rule::row => {
        let mut inner = pair.into_inner();

        let first = inner.next().map(Self::parse_rule).unwrap_or_default();
        let rest = inner.map(Self::parse_rule).map(|v| v.into()).collect();

        Value::Row(first.into(), rest)
      }
      Rule::file => Value::File(
        pair
          .into_inner()
          .map(Self::parse_rule)
          .map(|v| v.into())
          .collect(),
      ),
      _ => Value::Error,
    }
  }

  pub fn parse<'a>(pairs: Pairs<'a, Rule>) -> [[i32; 27]; 27] {
    let parsed: Vec<Value> = pairs
      .map(Self::parse_rule)
      .filter(|v| match v {
        Value::Error => false,
        _ => true,
      })
      .collect();

    let cols: Vec<AminoAcid> = match &parsed[0] {
      Value::Header(cols) => cols
        .iter()
        .map(|v| match **v {
          Value::Amino(a) => a,
          _ => panic!("Non-amino acid found in header"),
        })
        .collect(),
      _ => panic!("File does not start with matrix header"),
    };

    let mut matrix = [[0i32; 27]; 27];

    parsed[1..].iter().for_each(|v| match v {
      Value::Row(first, rest) => {
        let i: u8 = match **first {
          Value::Amino(a) => a.into(),
          _ => panic!("Non-amino acid at start of ro"),
        };
        let i = i as usize;

        rest.iter().enumerate().for_each(|(j, v)| match **v {
          Value::Int(n) => {
            let j: u8 = cols[j].into();
            let j = j as usize;

            matrix[i][j] = n;
            matrix[j][i] = n;
          }
          _ => panic!("Non-int in matrix"),
        });
      }
      _ => panic!("Non-row encountered"),
    });

    matrix
  }
}

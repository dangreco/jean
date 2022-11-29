use std::collections::HashMap;

use crate::fasta::token::Token;

use super::{entry::Entry, lexer::FastaLexer};
use anyhow::Result;
use jean_core::sequence::Sequence;

#[derive(Debug, Clone)]
pub struct FastaParser;

#[allow(dead_code)]
impl FastaParser {
  pub fn parse_str(str: &str) -> Result<HashMap<String, Entry>> {
    let mut entries = HashMap::new();

    let lexed = FastaLexer::lex(str);

    lexed
      .windows(3)
      .map(|c| (&c[0], &c[1], &c[2]))
      .for_each(|(_0, _1, _2)| match (_0.0, _1.0, _2.0) {
        (Token::Id, Token::Metadata, Token::Sequence) => {
          entries.insert(
            _0.1.clone(),
            Entry {
              metadata: _1.1.clone(),
              sequence: Sequence::try_from(_2.1.as_str()).unwrap(),
            },
          );
        }
        (Token::Id, Token::Sequence, _) => {
          entries.insert(
            _0.1.clone(),
            Entry {
              metadata: "".to_string(),
              sequence: Sequence::try_from(_1.1.as_str()).unwrap(),
            },
          );
        }
        _ => (),
      });

    Ok(entries)
  }
}

#[cfg(test)]
mod tests {
  use super::FastaParser;

  #[test]
  fn test_1() {
    let fasta = "; Some comment!\n>seq1\nACAGCTAAG\nCGACT ACGACTG\n\n>seq2\nACGACTAGCTT\n; Some comment!\nAGTAGT";
    let parsed = FastaParser::parse_str(fasta);
    assert!(parsed.is_ok());
  }
}

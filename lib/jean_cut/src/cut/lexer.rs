use anyhow::Result;
use pest::{iterators::Pairs, Parser};

#[derive(Debug, Clone, Parser)]
#[grammar = "cut.pest"]
pub struct CutLexer;

#[allow(dead_code)]
impl CutLexer {
  pub fn lex<'a>(str: &'a str) -> Result<Pairs<'a, Rule>> {
    CutLexer::parse(Rule::file, str).map_err(|e| e.into())
  }
}

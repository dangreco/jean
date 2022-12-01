use anyhow::Result;
use pest::{iterators::Pairs, *};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/codegen/matrix.pest"]
pub struct MatrixLexer;

impl MatrixLexer {
  pub fn lex(str: &str) -> Result<Pairs<Rule>> {
    MatrixLexer::parse(Rule::file, str).map_err(|e| e.into())
  }
}

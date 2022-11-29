#[macro_export]
macro_rules! def_lexer {
  ($l:ident, $p:literal) => {
    use anyhow::Result;
    use pest::{iterators::Pairs, Parser};

    #[derive(Debug, Clone, Parser)]
    #[grammar = $p]
    pub struct $l;

    #[allow(dead_code)]
    impl $l {
      pub fn lex<'a>(str: &'a str) -> Result<Pairs<'a, Rule>> {
        $l::parse(Rule::file, str).map_err(|e| e.into())
      }
    }
  };
}

use super::token::Token;
use logos::Logos;

pub struct Lexer;

impl Lexer {
  pub fn read_str(str: &str) -> Vec<(Token, String)> {
    let mut lexer = Token::lexer(str);

    let mut toks = Vec::new();

    while let Some(token) = lexer.next() {
      if token == Token::Error {
        continue;
      }
      toks.push((token, lexer.slice().to_string()));
    }

    toks
  }
}

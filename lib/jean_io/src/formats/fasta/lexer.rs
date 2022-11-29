use super::token::Token;

#[derive(Debug, Clone)]
pub struct FastaLexer;

impl FastaLexer {
  pub fn lex(str: &str) -> Vec<(Token, String)> {
    let mut tokens = Vec::new();

    let mut buf: String = String::new();
    let mut prev = Token::None;
    let mut curr = Token::None;

    /* buf.push() | _ppp_ppppppppppppp__ppppppp_ppppppp_ppppp_pppppp_pppppp__ppppppppppppppppppppp_pppp__pppppppp_________________ppppppp__pppppppp
     * state      | SIII_MMMMMMMMMMMMM__sssssss_sssssss_sssss_ssssss_ssssss__sssssssssssssssssssssSIIII__ssssssss__ccccccccccccc__sssssss__ssssssss
     * c          | >seq some=metadata\nacagatg acgtacg acgta gtacgt agtagt\nacgacagtcagtcatgtagct>seq2\nacgacgta\n;Some comment\ngatcgta\nagcacgat
     * T          | 1   2             33                                                          1    44          5            66
     */

    for c in str.chars() {
      match (prev, curr, c) {
        /* T1 */
        (_, _, '>') => {
          if !buf.is_empty() {
            tokens.push((curr, buf.clone()));
            buf.clear();
          }
          prev = curr;
          curr = Token::Id;
        }
        /* T2 / T4 */
        (_, Token::Id, c) if c.is_whitespace() => {
          tokens.push((curr, buf.clone()));
          buf.clear();
          prev = curr;
          curr = if c == '\n' {
            Token::Sequence
          } else {
            Token::Metadata
          }
        }
        /* T3 */
        (_, Token::Metadata, '\n') => {
          tokens.push((curr, buf.clone()));
          buf.clear();
          prev = curr;
          curr = Token::Sequence;
        }
        /* T5 */
        (_, _, ';') => {
          prev = curr;
          curr = Token::Comment;
        }
        /* T6 */
        (_, Token::Comment, '\n') => {
          curr = prev;
        }
        /* Ignore Nones */
        (_, Token::None, _) => (),
        /* Ignore Whitespace in Sequence */
        (_, Token::Sequence, c) => {
          if !c.is_whitespace() {
            buf.push(c);
          }
        }
        /* Ignore Comments */
        (_, curr, c) if curr != Token::Comment => buf.push(c),
        _ => (),
      }
    }

    if !buf.is_empty() {
      tokens.push((curr, buf.clone()));
    }

    tokens
  }
}

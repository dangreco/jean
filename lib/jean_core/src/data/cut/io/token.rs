use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
pub enum Token {
  #[regex(r"[A-Z\*]")]
  AminoAcid,

  #[regex(r"[A-Z][A-Z][A-Z]")]
  Codon,

  #[regex(r"\d+\.\d+")]
  Number,

  #[error]
  #[regex(r"[ \t\n\f]+", logos::skip)]
  Error,
}

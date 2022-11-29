use crate::sequence::{codon::Codon, protein::AminoAcid};

use super::{token::Token, entry::Entry};

pub struct Parser;

impl Parser {

  pub fn parse(tokens: &Vec<(Token, String)>) -> Vec<Entry>
  {
    let mut entries = Vec::new();

    tokens.windows(4).for_each(|w| {
      match w {
        [
          (Token::Codon, codon),
          (Token::AminoAcid, amino_acid),
          (Token::Number, fraction),
          (Token::Number, frequency)
        ] => {
          let codon = Codon::try_from(codon.as_str()).unwrap();
          let amino_acid = AminoAcid::try_from(amino_acid.chars().nth(0).unwrap()).unwrap();
          let fraction = fraction.parse::<f64>().unwrap();
          let frequency = frequency.parse::<f64>().unwrap() / 1000.0;

          entries.push(Entry { codon, amino_acid, fraction: Some(fraction), frequency: Some(frequency) });
        },
        _ => () 
      }
    });

    entries
  }

}
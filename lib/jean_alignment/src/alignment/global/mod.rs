use std::borrow::Borrow;

use anyhow::Result;
use jean_core::{
  prelude::Gap,
  sequence::{protein::AminoAcid, Seq},
};

use crate::utils::{a_to, to_a};

use super::Alignment;

pub struct NeedlemanWunsch {
  matrix: Vec<Vec<i32>>,
  gap_penalty: Box<dyn Fn(i32) -> i32>,
}

static MATCH: u8 = 0b00000001;
static DELETE: u8 = 0b00000010;
static INSERT: u8 = 0b00000100;

impl NeedlemanWunsch {
  pub fn new() -> Self {
    Self {
      matrix: Vec::new(),
      gap_penalty: Box::new(|_| 0),
    }
  }

  pub fn matrix<'a, U, I>(mut self, matrix: &[I]) -> Self
  where
    U: Borrow<i32>,
    I: IntoIterator<Item = U> + Clone,
  {
    self.matrix = matrix
      .iter()
      .cloned()
      .map(|r| r.into_iter().map(|v| *v.borrow()).collect())
      .collect();
    self
  }

  pub fn gap_penalty(mut self, gap_penalty: impl Fn(i32) -> i32 + 'static) -> Self {
    self.gap_penalty = Box::new(gap_penalty);
    self
  }

  pub fn align<T>(self, a: &Seq<T>, b: &Seq<T>) -> Result<Alignment<T>>
  where
    char: From<T>,
    T: TryFrom<char> + Copy + Gap,
  {
    let a: Vec<AminoAcid> = a.0.iter().map(|t| to_a(*t)).collect();
    let b: Vec<AminoAcid> = b.0.iter().map(|t| to_a(*t)).collect();

    let n = a.len() + 1;
    let m = b.len() + 1;

    let mut scores = vec![vec![0; m]; n];
    let mut traces = vec![vec![u8::default(); m]; n];

    for i in 0..n {
      scores[i][0] = (self.gap_penalty)(i as i32);
      traces[i][0] = DELETE;
    }

    for j in 0..m {
      scores[0][j] = (self.gap_penalty)(j as i32);
      traces[0][j] = INSERT;
    }

    for i in 1..n {
      let ai = u8::from(a[i - 1]) as usize;
      for j in 1..m {
        let bj = u8::from(b[j - 1]) as usize;

        let _m = scores[i - 1][j - 1] + self.matrix[ai][bj];
        let _d = (1..i)
          .map(|k| scores[i - k][j] + (self.gap_penalty)(k as i32))
          .max()
          .unwrap_or(0);

        let _i = (1..j)
          .map(|l| scores[i][j - l] + (self.gap_penalty)(l as i32))
          .max()
          .unwrap_or(0);

        let max = _m.max(_d.max(_i));

        scores[i][j] = max;

        if _m == max {
          traces[i][j] |= MATCH;
        }

        if _d == max {
          traces[i][j] |= DELETE;
        }

        if _i == max {
          traces[i][j] |= INSERT;
        }
      }
    }

    let score = scores[n - 1][m - 1];

    let mut aa = Seq::new();
    let mut ba = Seq::new();

    let mut i = n - 1;
    let mut j = m - 1;

    while i > 0 || j > 0 {
      let t = traces[i][j];

      if i > 0 && j > 0 && t & MATCH == MATCH {
        aa.insert(0, a_to(a[i - 1]));
        ba.insert(0, a_to(b[j - 1]));
        i -= 1;
        j -= 1;
      } else if i > 0 && t & DELETE == DELETE {
        aa.insert(0, a_to(a[i - 1]));
        ba.insert(0, T::gap());
        i -= 1;
      } else {
        aa.insert(0, T::gap());
        ba.insert(0, a_to(b[j - 1]));
        j -= 1;
      }
    }

    Ok(Alignment {
      score,
      a: aa,
      b: ba,
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::alignment::global::NeedlemanWunsch;
  use anyhow::Result;
  use jean_core::sequence::dna::{self, Dna};
  use rand::Rng;

  #[test]
  fn test_1() -> Result<()> {
    let mut matrix = vec![vec![0; 27]; 27];
    for i in 0..27 {
      for j in 0..27 {
        matrix[i][j] = if i == j { 1 } else { -1 };
      }
    }

    let nw = NeedlemanWunsch::new().matrix(&matrix).gap_penalty(|k| -k);

    let seq1: Dna = Dna::try_from("GCATGCG").unwrap();
    let seq2: Dna = Dna::try_from("GATTACA").unwrap();

    let aligned = nw.align(&seq1, &seq2)?;

    let seq1_aligned: Dna = Dna::try_from("GCA-TGCG").unwrap();
    let seq2_aligned: Dna = Dna::try_from("G-ATTACA").unwrap();

    assert_eq!(aligned.score, 0);
    assert_eq!(aligned.a, seq1_aligned);
    assert_eq!(aligned.b, seq2_aligned);

    Ok(())
  }

  #[test]
  fn test_2() -> Result<()> {
    let mut matrix = vec![vec![0; 27]; 27];
    for i in 0..27 {
      for j in 0..27 {
        matrix[i][j] = if i == j { 5 } else { -5 };
      }
    }

    let seq1: Dna = Dna::try_from("AGACTAGTTAC").unwrap();
    let seq2: Dna = Dna::try_from("CGAGACGT").unwrap();

    let seq1_aligned: Dna = Dna::try_from("--AGACTAGTTAC").unwrap();
    let seq2_aligned: Dna = Dna::try_from("CGAGAC--G-T--").unwrap();

    let aligned = NeedlemanWunsch::new()
      .matrix(&matrix)
      .gap_penalty(|k| -2 * k)
      .align(&seq1, &seq2)?;

    assert_eq!(aligned.score, 16);
    assert_eq!(aligned.a, seq1_aligned);
    assert_eq!(aligned.b, seq2_aligned);

    Ok(())
  }

  #[test]
  fn test_3() -> Result<()> {
    let mut matrix = vec![vec![0; 27]; 27];
    for i in 0..27 {
      for j in 0..27 {
        matrix[i][j] = if i == j { 1 } else { -1 };
      }
    }

    let mut rng = rand::thread_rng();
    let bases = [dna::Base::A, dna::Base::C, dna::Base::G, dna::Base::T];
    let mut sample_base = || bases[rng.gen_range(0..4)];

    let seq1 = Dna::from((0..300).map(|_| sample_base()).collect::<Vec<dna::Base>>());
    let seq2 = Dna::from((0..300).map(|_| sample_base()).collect::<Vec<dna::Base>>());

    let nw = NeedlemanWunsch::new().matrix(&matrix).gap_penalty(|k| -k);

    assert!(nw.align(&seq1, &seq2).is_ok());

    Ok(())
  }
}

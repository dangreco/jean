use std::{borrow::Borrow, fmt::Debug};

use anyhow::Result;
use jean_core::{
  prelude::Gap,
  sequence::{protein::AminoAcid, Seq},
};

use crate::utils::{a_to, to_a};

use super::Alignment;

static MATCH: u8 = 0b00000001;
static DELETE: u8 = 0b00000010;
static INSERT: u8 = 0b00000100;

pub struct SmithWaterman {
  matrix: Vec<Vec<i32>>,
  gap_penalty: Box<dyn Fn(i32) -> i32>,
}

impl SmithWaterman {
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
    T: TryFrom<char> + Copy + Gap + Debug,
  {
    let a: Vec<AminoAcid> = a.0.iter().map(|t| to_a(*t)).collect();
    let b: Vec<AminoAcid> = b.0.iter().map(|t| to_a(*t)).collect();

    let n = a.len() + 1;
    let m = b.len() + 1;

    let mut scores = vec![vec![0; m]; n];
    let mut traces = vec![vec![u8::default(); m]; n];

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
          .max_by(|a, b| a.partial_cmp(b).unwrap())
          .unwrap_or(0);

        let max = _m.max(_d.max(_i.max(0)));
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

    let (i, j, score) = scores
      .iter()
      .enumerate()
      .map(|(i, row)| {
        let (j, score) = row
          .iter()
          .enumerate()
          .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
          .unwrap();
        (i, j, score)
      })
      .max_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap())
      .unwrap();

    let mut aa = Seq::new();
    let mut ba = Seq::new();

    let mut i = i;
    let mut j = j;

    while (i > 0 || j > 0) && scores[i][j] != 0 {
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
      score: *score,
      a: aa,
      b: ba,
    })
  }
}

#[cfg(test)]
mod tests {
  use anyhow::Result;
  use jean_core::sequence::dna::Dna;

  use super::SmithWaterman;

  #[test]
  fn test_1() -> Result<()> {
    let mut matrix = vec![vec![0; 27]; 27];
    for i in 0..27 {
      for j in 0..27 {
        matrix[i][j] = if i == j { 3 } else { -3 };
      }
    }

    let seq1: Dna = Dna::try_from("GGTTGACTA").unwrap();
    let seq2: Dna = Dna::try_from("TGTTACGG").unwrap();

    let sw = SmithWaterman::new().matrix(&matrix).gap_penalty(|k| -2 * k);

    let alignment = sw.align(&seq1, &seq2);

    println!("{:?}", alignment);

    Ok(())
  }
}

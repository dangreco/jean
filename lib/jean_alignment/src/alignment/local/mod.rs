use std::fmt::Debug;

use anyhow::{anyhow, Result};
use jean_core::{prelude::Gap, sequence::Seq};
use num_traits::{real::Real, FromPrimitive};
use strum::EnumCount;

use super::Alignment;

static MATCH: u8 = 0b00000001;
static DELETE: u8 = 0b00000010;
static INSERT: u8 = 0b00000100;

pub struct SmithWaterman<N> {
  similarity_matrix: Vec<Vec<N>>,
  gap_penalty: Box<dyn Fn(N) -> N>,
}

impl<N> SmithWaterman<N>
where
  N: Real + Clone + Debug + FromPrimitive,
{
  pub fn new() -> Self {
    Self {
      similarity_matrix: Vec::new(),
      gap_penalty: Box::new(|_| N::zero()),
    }
  }

  pub fn similarity_matrix<T>(mut self, similarity_matrix: Vec<Vec<N>>) -> Result<Self>
  where
    T: EnumCount + Gap,
  {
    let n = similarity_matrix.len();
    let m = similarity_matrix.get(0).unwrap_or(&vec![]).len();
    let t = T::COUNT - 1;

    if n != t {
      return Err(anyhow!(
        "Invalid similarity matrix dimension (rows): got {}, expected {}.",
        n,
        t
      ));
    }

    if m != T::COUNT - 1 {
      return Err(anyhow!(
        "Invalid similarity matrix dimension (cols): got {}, expected {}.",
        m,
        t
      ));
    }

    self.similarity_matrix = similarity_matrix;
    Ok(self)
  }

  pub fn gap_penalty(mut self, gap_penalty: impl Fn(N) -> N + 'static) -> Self {
    self.gap_penalty = Box::new(gap_penalty);
    self
  }

  pub fn align<T>(self, a: &Seq<T>, b: &Seq<T>) -> Result<Alignment<N, T>>
  where
    u8: From<T>,
    T: From<u8> + Copy + Gap + Debug,
  {
    let n = a.len() + 1;
    let m = b.len() + 1;

    let mut scores = vec![vec![N::zero(); m]; n];
    let mut traces = vec![vec![u8::default(); m]; n];

    for i in 1..n {
      let ai = u8::from(a[i - 1]) as usize;
      for j in 1..m {
        let bj = u8::from(b[j - 1]) as usize;

        let _m = scores[i - 1][j - 1] + self.similarity_matrix[ai][bj];
        let _d = (1..i)
          .map(|k| scores[i - k][j] + (self.gap_penalty)(N::from(k).unwrap()))
          .max_by(|a, b| a.partial_cmp(b).unwrap())
          .unwrap_or(N::zero());
        let _i = (1..j)
          .map(|l| scores[i][j - l] + (self.gap_penalty)(N::from(l).unwrap()))
          .max_by(|a, b| a.partial_cmp(b).unwrap())
          .unwrap_or(N::zero());

        let max = _m.max(_d.max(_i.max(N::zero())));
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

    while (i > 0 || j > 0) && scores[i][j] != N::zero() {
      let t = traces[i][j];

      if i > 0 && j > 0 && t & MATCH == MATCH {
        aa.insert(0, a[i - 1]);
        ba.insert(0, b[j - 1]);
        i -= 1;
        j -= 1;
      } else if i > 0 && t & DELETE == DELETE {
        aa.insert(0, a[i - 1]);
        ba.insert(0, T::gap());
        i -= 1;
      } else {
        aa.insert(0, T::gap());
        ba.insert(0, b[j - 1]);
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
  use jean_core::sequence::dna::{self, Dna};

  use super::SmithWaterman;

  #[test]
  fn test_1() -> Result<()> {
    let seq1: Dna = Dna::try_from("GGTTGACTA").unwrap();
    let seq2: Dna = Dna::try_from("TGTTACGG").unwrap();

    let matrix = vec![
      vec![3.0, -3.0, -3.0, -3.0],
      vec![-3.0, 3.0, -3.0, -3.0],
      vec![-3.0, -3.0, 3.0, -3.0],
      vec![-3.0, -3.0, -3.0, 3.0],
    ];

    let sw = SmithWaterman::new()
      .similarity_matrix::<dna::Base>(matrix)?
      .gap_penalty(|k| -2.0 * k);

    let alignment = sw.align(&seq1, &seq2);

    println!("{:?}", alignment);

    Ok(())
  }
}

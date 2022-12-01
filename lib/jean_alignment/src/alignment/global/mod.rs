use std::fmt::Debug;

use anyhow::{anyhow, Context, Result};
use jean_core::{prelude::Gap, sequence::Seq};
use num_traits::real::Real;
use strum::EnumCount;

use super::Alignment;

#[derive(Debug, Clone)]
pub struct NeedlemanWunsch<N>
where
  N: Real,
{
  similarity_matrix: Vec<Vec<N>>,
  gap_penalty: N,
}

static MATCH: u8 = 0b00000001;
static DELETE: u8 = 0b00000010;
static INSERT: u8 = 0b00000100;

impl<N> NeedlemanWunsch<N>
where
  N: Real + Clone + Copy,
{
  pub fn new() -> Self {
    Self {
      similarity_matrix: Vec::new(),
      gap_penalty: N::zero(),
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

  pub fn gap_penalty(mut self, gap_penalty: N) -> Self {
    self.gap_penalty = gap_penalty;
    self
  }

  pub fn align<T>(self, a: &Seq<T>, b: &Seq<T>) -> Result<Alignment<N, T>>
  where
    u8: From<T>,
    T: From<u8> + Copy + Gap,
  {
    let n = a.len() + 1;
    let m = b.len() + 1;

    let mut scores = vec![vec![N::zero(); m]; n];
    let mut traces = vec![vec![u8::default(); m]; n];

    for i in 0..n {
      scores[i][0] = N::from(i).context("usize -> N")? * self.gap_penalty;
      traces[i][0] = DELETE;
    }

    for j in 0..m {
      scores[0][j] = N::from(j).context("usize -> N")? * self.gap_penalty;
      traces[0][j] = INSERT;
    }

    for i in 1..n {
      let ai = u8::from(a[i - 1]) as usize;
      for j in 1..m {
        let bj = u8::from(b[j - 1]) as usize;

        let _m = scores[i - 1][j - 1] + self.similarity_matrix[ai][bj];
        let _d = scores[i - 1][j] + self.gap_penalty;
        let _i = scores[i][j - 1] + self.gap_penalty;

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
      score,
      a: aa,
      b: ba,
    })
  }
}

#[cfg(test)]
mod tests {
  use std::time::Instant;

  use crate::alignment::global::NeedlemanWunsch;
  use anyhow::Result;
  use jean_core::sequence::dna::{self, Dna};
  use rand::Rng;

  #[test]
  fn test_1() -> Result<()> {
    let nw = NeedlemanWunsch::new()
      .similarity_matrix::<dna::Base>(vec![
        vec![1.0, -1.0, -1.0, -1.0],
        vec![-1.0, 1.0, -1.0, -1.0],
        vec![-1.0, -1.0, 1.0, -1.0],
        vec![-1.0, -1.0, -1.0, 1.0],
      ])?
      .gap_penalty(-1.0);

    let seq1: Dna = Dna::try_from("GCATGCG").unwrap();
    let seq2: Dna = Dna::try_from("GATTACA").unwrap();

    let aligned = nw.align(&seq1, &seq2)?;

    let seq1_aligned: Dna = Dna::try_from("GCA-TGCG").unwrap();
    let seq2_aligned: Dna = Dna::try_from("G-ATTACA").unwrap();

    assert_eq!(aligned.score, 0.0);
    assert_eq!(aligned.a, seq1_aligned);
    assert_eq!(aligned.b, seq2_aligned);

    Ok(())
  }

  #[test]
  fn test_2() -> Result<()> {
    let seq1: Dna = Dna::try_from("AGACTAGTTAC").unwrap();
    let seq2: Dna = Dna::try_from("CGAGACGT").unwrap();

    let seq1_aligned: Dna = Dna::try_from("--AGACTAGTTAC").unwrap();
    let seq2_aligned: Dna = Dna::try_from("CGAGAC--G-T--").unwrap();

    let aligned = NeedlemanWunsch::new()
      .similarity_matrix::<dna::Base>(vec![
        vec![5.0, -5.0, -5.0, -5.0],
        vec![-5.0, 5.0, -5.0, -5.0],
        vec![-5.0, -5.0, 5.0, -5.0],
        vec![-5.0, -5.0, -5.0, 5.0],
      ])?
      .gap_penalty(-2.0)
      .align(&seq1, &seq2)?;

    assert_eq!(aligned.score, 16.0);
    assert_eq!(aligned.a, seq1_aligned);
    assert_eq!(aligned.b, seq2_aligned);

    Ok(())
  }

  #[test]
  fn test_3() -> Result<()> {
    let mut rng = rand::thread_rng();
    let bases = [dna::Base::A, dna::Base::C, dna::Base::G, dna::Base::T];
    let mut sample_base = || bases[rng.gen_range(0..4)];

    let seq1 = Dna::from((0..300).map(|_| sample_base()).collect::<Vec<dna::Base>>());
    let seq2 = Dna::from((0..300).map(|_| sample_base()).collect::<Vec<dna::Base>>());

    let nw = NeedlemanWunsch::new()
      .similarity_matrix::<dna::Base>(vec![
        vec![1.0, -1.0, -1.0, -1.0],
        vec![-1.0, 1.0, -1.0, -1.0],
        vec![-1.0, -1.0, 1.0, -1.0],
        vec![-1.0, -1.0, -1.0, 1.0],
      ])?
      .gap_penalty(-1.0);

    let start = Instant::now();
    nw.align(&seq1, &seq2)?;
    let duration = start.elapsed();
    assert!(duration.as_millis() < 25);
    Ok(())
  }
}

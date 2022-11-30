use std::fmt::Debug;

use jean_core::{prelude::Gap, sequence::Seq};
use num_traits::{FromPrimitive, Num};

#[derive(Debug, Clone)]
pub struct Alignment<T>((Seq<T>, Seq<T>));

impl<T> Alignment<T> {
  pub fn a(&self) -> &Seq<T> {
    &self.0 .0
  }

  pub fn b(&self) -> &Seq<T> {
    &self.0 .1
  }
}

#[derive(Debug, Clone)]
pub struct NeedlemanWunsch<N>
where
  N: Num,
{
  similarity_matrix: Vec<Vec<N>>,
  gap_penalty: N,
}

static MATCH: u8 = 0b00000001;
static DELETE: u8 = 0b00000010;
static INSERT: u8 = 0b00000100;

impl<N> NeedlemanWunsch<N>
where
  N: Num + Clone + Copy,
{
  pub fn new() -> Self {
    Self {
      similarity_matrix: Vec::new(),
      gap_penalty: N::zero(),
    }
  }

  pub fn similarity_matrix(mut self, similarity_matrix: Vec<Vec<N>>) -> Self {
    self.similarity_matrix = similarity_matrix;
    self
  }

  pub fn gap_penalty(mut self, gap_penalty: N) -> Self {
    self.gap_penalty = gap_penalty;
    self
  }

  pub fn align<T>(self, a: &Seq<T>, b: &Seq<T>) -> (N, Alignment<T>)
  where
    N: FromPrimitive + PartialOrd + Debug,
    u8: From<T>,
    T: From<u8> + Copy + Gap,
  {
    let n = a.len() + 1;
    let m = b.len() + 1;

    let mut scores = vec![vec![N::zero(); m]; n];
    let mut traces = vec![vec![u8::default(); m]; n];

    for i in 0..n {
      scores[i][0] = N::from_usize(i).unwrap() * self.gap_penalty;
      traces[i][0] = DELETE;
    }

    for j in 0..m {
      scores[0][j] = N::from_usize(j).unwrap() * self.gap_penalty;
      traces[0][j] = INSERT;
    }

    for i in 1..n {
      let ai: u8 = a[i - 1].into();
      for j in 1..m {
        let bj: u8 = b[j - 1].into();

        let _m = scores[i - 1][j - 1] + self.similarity_matrix[ai as usize][bj as usize];
        let _d = scores[i - 1][j] + self.gap_penalty;
        let _i = scores[i][j - 1] + self.gap_penalty;

        let opts = vec![_m, _d, _i];

        let max = *opts
          .iter()
          .max_by(|a, b| a.partial_cmp(b).unwrap())
          .unwrap();

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

    (score, Alignment((aa, ba)))
  }
}

#[cfg(test)]
mod tests {
  use crate::NeedlemanWunsch;
  use jean_core::sequence::dna::Dna;

  #[test]
  fn test_1() {
    let nw = NeedlemanWunsch::new()
      .similarity_matrix(vec![
        vec![1, -1, -1, -1],
        vec![-1, 1, -1, -1],
        vec![-1, -1, 1, -1],
        vec![-1, -1, -1, 1],
      ])
      .gap_penalty(-1);

    let seq1: Dna = Dna::try_from("GCATGCG").unwrap();
    let seq2: Dna = Dna::try_from("GATTACA").unwrap();

    let (score, alignment) = nw.align(&seq1, &seq2);

    let seq1_aligned: Dna = Dna::try_from("GCA-TGCG").unwrap();
    let seq2_aligned: Dna = Dna::try_from("G-ATTACA").unwrap();

    assert_eq!(score, 0);
    assert_eq!(alignment.a(), &seq1_aligned);
    assert_eq!(alignment.b(), &seq2_aligned);
  }

  #[test]
  fn test_2() {
    let seq1: Dna = Dna::try_from("AGACTAGTTAC").unwrap();
    let seq2: Dna = Dna::try_from("CGAGACGT").unwrap();

    let seq1_aligned: Dna = Dna::try_from("--AGACTAGTTAC").unwrap();
    let seq2_aligned: Dna = Dna::try_from("CGAGAC--G-T--").unwrap();

    let (score, alignment) = NeedlemanWunsch::new()
      .similarity_matrix(vec![
        vec![5, -5, -5, -5],
        vec![-5, 5, -5, -5],
        vec![-5, -5, 5, -5],
        vec![-5, -5, -5, 5],
      ])
      .gap_penalty(-2)
      .align(&seq1, &seq2);

    println!("{:?}", alignment);

    assert_eq!(score, 16);
    assert_eq!(alignment.a(), &seq1_aligned);
    assert_eq!(alignment.b(), &seq2_aligned);
  }
}

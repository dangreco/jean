use jean_core::sequence::Seq;

#[derive(Debug, Clone)]
pub struct Alignment<N, T> {
  pub score: N,
  pub a: Seq<T>,
  pub b: Seq<T>,
}

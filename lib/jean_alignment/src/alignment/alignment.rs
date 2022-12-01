use jean_core::sequence::Seq;

#[derive(Debug, Clone)]
pub struct Alignment<T> {
  pub score: i32,
  pub a: Seq<T>,
  pub b: Seq<T>,
}

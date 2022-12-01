use jean_core::sequence::Seq;

#[derive(Debug, Clone)]
pub struct Alignment<T>((Seq<T>, Seq<T>));

impl<T> Alignment<T> {
  pub fn new(a: Seq<T>, b: Seq<T>) -> Self {
    Self((a, b))
  }

  pub fn a(&self) -> &Seq<T> {
    &self.0 .0
  }

  pub fn b(&self) -> &Seq<T> {
    &self.0 .1
  }
}

use jean_core::sequence::Sequence;

#[derive(Debug, Clone)]
pub struct Entry {
  pub metadata: String,
  pub sequence: Sequence,
}

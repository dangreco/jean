use std::collections::BTreeMap;

pub trait Sequences<T> {
  fn sequences(&self) -> BTreeMap<String, jean_core::sequence::Sequence>;
}

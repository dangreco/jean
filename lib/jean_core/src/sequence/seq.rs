use std::fmt::Display;

use crate::prelude::Complement;
use num_traits::Float;

/// General struct for a linear sequence `Seq<T>` - wrapper around `Vec<T>`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Seq<T>(pub Vec<T>);

impl<T> Seq<T> {
  pub fn new() -> Self {
    Self(Vec::new())
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn iter(&self) -> std::slice::Iter<T> {
    self.0.iter()
  }

  pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
    self.0.iter_mut()
  }

  pub fn push(&mut self, t: T) {
    self.0.push(t)
  }

  pub fn insert(&mut self, index: usize, t: T) {
    self.0.insert(index, t)
  }

  pub fn frequencies<F>(&self) -> std::collections::BTreeMap<&T, F>
  where
    F: Float,
    T: Ord,
  {
    let len = F::from(self.len()).unwrap();
    let mut frequencies = std::collections::BTreeMap::new();

    self.iter().for_each(|t| {
      let e = frequencies.entry(t).or_insert(F::zero());
      *e = *e + F::one();
    });

    frequencies.iter_mut().for_each(|(_, e)| *e = *e / len);

    frequencies
  }
}

impl<T> Complement for Seq<T>
where
  T: Complement,
{
  fn complement(&self) -> Self {
    Self(self.0.iter().map(|t| t.complement()).collect())
  }
}

impl<T> IntoIterator for Seq<T> {
  type Item = T;
  type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a, T> IntoIterator for &'a Seq<T> {
  type Item = &'a T;
  type IntoIter = std::slice::Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut Seq<T> {
  type Item = &'a mut T;
  type IntoIter = std::slice::IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl<T, I: std::slice::SliceIndex<[T]>> std::ops::Index<I> for Seq<T> {
  type Output = I::Output;

  fn index(&self, index: I) -> &Self::Output {
    &self.0[index]
  }
}

impl<T, I: std::slice::SliceIndex<[T]>> std::ops::IndexMut<I> for Seq<T> {
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    &mut self.0[index]
  }
}

impl<T> From<Vec<T>> for Seq<T> {
  fn from(value: Vec<T>) -> Self {
    Self(value)
  }
}

impl<'a, T> From<&'a Vec<T>> for Seq<T>
where
  T: Clone,
{
  fn from(value: &'a Vec<T>) -> Self {
    Self(value.to_vec())
  }
}

impl<'a, T> From<&'a [T]> for Seq<T>
where
  T: Clone,
{
  fn from(value: &'a [T]) -> Self {
    Self(value.to_vec())
  }
}

impl<'a, T> TryFrom<&'a str> for Seq<T>
where
  T: TryFrom<char> + Clone,
{
  type Error = &'static str;

  fn try_from(value: &'a str) -> Result<Self, Self::Error> {
    let internal: Result<Vec<T>, _> = value.chars().map(|c| T::try_from(c)).collect();

    match internal {
      Ok(internal) => Ok(Self::from(&internal[..])),
      Err(_) => Err("failed"),
    }
  }
}

impl<T> Display for Seq<T>
where
  T: Copy,
  char: From<T>,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for t in &self.0 {
      write!(f, "{}", char::from(*t))?;
    }

    Ok(())
  }
}

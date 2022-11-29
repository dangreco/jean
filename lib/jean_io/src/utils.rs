use std::str::FromStr;

use pest::{iterators::Pair, RuleType};

pub fn cast<'a, T, R>(pair: &Pair<'a, R>) -> T
where
  T: Default + FromStr,
  R: RuleType,
{
  pair.as_str().parse::<T>().unwrap_or_default()
}

pub fn expand<'a, R>(pair: &Pair<'a, R>) -> Vec<Pair<'a, R>>
where
  R: RuleType,
{
  pair.clone().into_inner().collect::<Vec<Pair<'a, R>>>()
}

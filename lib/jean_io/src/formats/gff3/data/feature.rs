use std::collections::HashMap;

use super::Strand;

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
  pub id: String,
  pub source: Option<String>,
  pub feature_type: String,
  pub start: usize,
  pub end: usize,
  pub score: Option<f64>,
  pub strand: Option<Strand>,
  pub phase: Option<u8>,
  pub attributes: Option<HashMap<String, String>>,
}

fn maybe<T>(t: &Option<T>) -> String
where
  T: std::fmt::Display,
{
  match t {
    Some(t) => t.to_string(),
    None => ".".to_owned(),
  }
}

impl std::fmt::Display for Entry {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let empty = ".".to_string();

    /* seqid */
    let _1 = &self.id;

    /* source */
    let _2 = maybe(&self.source);

    /* type */
    let _3 = &self.feature_type;

    /* start / end  */
    let _4 = &self.start.to_string();
    let _5 = &self.end.to_string();

    /* score */
    let _6 = maybe(&self.score);

    /* strand */
    let _7 = maybe(&self.strand);

    /* phase */
    let _8 = maybe(&self.phase);

    /* attributes */
    let _9 = match &self.attributes {
      Some(attributes) => attributes
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<String>>()
        .join(";"),
      None => empty.clone(),
    };

    write!(f, "{_1}\t{_2}\t{_3}\t{_4}\t{_5}\t{_6}\t{_7}\t{_8}\t{_9}")
  }
}

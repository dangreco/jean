#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum Token {
  Id,
  Metadata,
  Sequence,
  None,
  Comment,
}

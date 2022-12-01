use jean_core::sequence::protein::AminoAcid;

pub fn to_a<T>(t: T) -> AminoAcid
where
  char: From<T>,
{
  AminoAcid::try_from(char::from(t)).unwrap()
}

pub fn a_to<T>(a: AminoAcid) -> T
where
  T: TryFrom<char>,
{
  match T::try_from(char::from(a)) {
    Ok(t) => t,
    Err(_) => panic!("Error converting {a} to T."),
  }
}
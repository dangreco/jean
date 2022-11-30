pub use jean_core::prelude::*;
pub use jean_core::sequence::*;

#[cfg(feature = "jean_macros")]
extern crate jean_macros;
#[cfg(feature = "jean_macros")]
pub use jean_macros::*;

#[cfg(feature = "jean_io")]
extern crate jean_io;
#[cfg(feature = "jean_io")]
pub mod io {
  pub use jean_io::fasta;
  pub use jean_io::gff3;
  pub use jean_io::prelude::*;
}

#[cfg(feature = "jean_alignment")]
extern crate jean_alignment;
#[cfg(feature = "jean_alignment")]
pub mod alignment {
  pub use jean_alignment::*;
}

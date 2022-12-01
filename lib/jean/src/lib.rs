#[cfg(feature = "jean_macros")]
extern crate jean_macros;

#[cfg(feature = "jean_io")]
extern crate jean_io;

#[cfg(feature = "jean_alignment")]
extern crate jean_alignment;

#[cfg(feature = "jean_blast")]
extern crate jean_blast;

/* ---- BEGIN CRATE ROOT ---- */
pub use jean_core::sequence::*;

#[cfg(feature = "jean_macros")]
pub use jean_macros::*;
/* ---- END CRATE ROOT ---- */

/// Trait definitions
pub mod prelude {
  pub use jean_core::prelude::*;
  pub use jean_cut::prelude::*;

  #[cfg(feature = "jean_io")]
  pub use jean_io::prelude::*;

  #[cfg(feature = "jean_alignment")]
  pub use jean_alignment::prelude::*;

  #[cfg(feature = "jean_blast")]
  pub use jean_blast::prelude::*;
}

/// Codon usage tables (codon frequency tables)
pub mod cut {
  pub use jean_cut::cut::*;
}

#[cfg(feature = "jean_io")]
/// I/O of common biological files
pub mod io {
  pub use jean_io::formats::*;
}

#[cfg(feature = "jean_alignment")]
/// Sequence alignment tools
pub mod alignment {
  pub use jean_alignment::alignment::*;
}

#[cfg(feature = "jean_blast")]
/// BLAST (basic local alignment search tool)
pub mod blast {
  pub use jean_blast::blast::*;
}

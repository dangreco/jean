#![allow(non_snake_case)]

#[cfg(not(feature = "codegen"))]
mod matrices;

#[cfg(not(feature = "codegen"))]
pub use matrices::*;

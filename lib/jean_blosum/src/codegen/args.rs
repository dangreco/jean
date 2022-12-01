use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
  #[arg(value_name = "MATRIX_DIR")]
  pub matrix_dir: PathBuf,
}

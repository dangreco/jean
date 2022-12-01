use args::Args;
use clap::Parser;
use lexer::MatrixLexer;
use parser::BlosumParser;
use std::io::{Read, Write};

mod args;
mod lexer;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();
  let mut dir = std::fs::read_dir(args.matrix_dir)?;
  let src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/matrices");

  let mut mods = Vec::new();

  while let Some(Ok(entry)) = dir.next() {
    let name = entry.file_name().to_str().unwrap().replace(".", "_");
    mods.push(name.clone());

    /* Read File */
    let mut buf = String::new();
    let mut file = std::fs::File::open(entry.path())?;
    file.read_to_string(&mut buf)?;

    let lexed = MatrixLexer::lex(buf.as_str())?;
    let matrix = BlosumParser::parse(lexed);

    /* Create Output */
    let out_path = src.join(format!("{}.rs", name));
    let mut out = std::fs::File::create(out_path)?;

    writeln!(out, "pub const {name}: [[i32; 27]; 27] = [")?;

    for row in matrix {
      writeln!(
        out,
        "\t[{}],",
        row
          .iter()
          .map(|i| i.to_string())
          .collect::<Vec<String>>()
          .join(", ")
      )?;
    }

    writeln!(out, "];")?;
  }

  /* Write matrices module */
  let mut module = std::fs::File::create(src.join("mod.rs"))?;

  for m in mods {
    writeln!(module, "mod {m};\npub use {m}::*;\n")?;
  }

  Ok(())
}

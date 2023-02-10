#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};
use log::{info, warn};
use grrs::find_matches;

/// Search for a pattern in a file and display the lines containing it

#[derive(Parser)]
struct Cli {
  /// pattern to look for
  pattern: String,
  /// path to the file to read
  path: std::path::PathBuf,
}

fn main() -> Result<()> {
  /// default logger initialization
  env_logger::init();
 
  /// parse the 2 arguments passed in (pattern and file)
  let args = Cli::parse();

  /// print them out
  println!("pattern: {:?}", std::env::args().nth(1).unwrap()); /// pattern
  println!("filename: {:?}", std::env::args().nth(2).unwrap()); /// path
  println!("---");

  /// read in the file, handle error if it fails
  let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("could not read file `{}`", &args.path.display()))?;

  /// find pattern matches in file provided
  grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

  Ok(())
}

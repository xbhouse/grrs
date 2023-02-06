#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};
use log::{info, warn};
use grrs::find_matches;

/// Search for a pattern in a file and display the lines containing it

#[derive(Parser)]
struct Cli {
  /// Pattern to look for
  pattern: String,
  /// Path to the file to read
  path: std::path::PathBuf,
}

fn main() -> Result<()> {
  env_logger::init();
  info!("starting up");
 
  let args = Cli::parse();

  println!("arg 1: {:?}", std::env::args().nth(1).unwrap());
  println!("arg 2: {:?}", std::env::args().nth(2).unwrap());

  let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("could not read file `{}`", &args.path.display()))?;

  grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

  Ok(())
}


#[test]
fn find_a_match() {
  let mut result = Vec::new();
  find_matches("hello world", "rld", &mut result);
  assert_eq!(result, b"hello world\n");
}

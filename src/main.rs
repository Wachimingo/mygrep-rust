#![allow(unused)]
use std::fmt;
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

impl fmt::Debug for Cli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("arguments")
            .field("pattern", &self.pattern)
            .field("path", &self.path)
            .finish()
    }
}

fn main() {
    let args = Cli::parse();
    let content = mygrep::read_lines(&args.path).expect("Reading content");
    mygrep::find_matches(content, &args.pattern, &mut std::io::stdout());
}
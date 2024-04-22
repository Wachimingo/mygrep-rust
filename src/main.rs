#![allow(unused)]
use std::fmt;
use clap::Parser;
use anyhow::{Context, Result};
use std::io::Read;
#[derive(Parser, Default)]
struct Cli {
    #[clap(short = 'w', long)]
    pattern: String,
    #[clap(short, long)]
    path: std::path::PathBuf,
    #[clap(short, long)]
    replace: Option<String>,
}

impl fmt::Debug for Cli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("arguments")
            .field("pattern", &self.pattern)
            .field("path", &self.path)
            .field("replace", &self.path)
            .finish()
    }
}

fn main() {
    let args: Cli = Cli::parse();
    let content = mygrep::read_lines(&args.path).expect("Reading content");
    

    let mut found_match_instances = mygrep::find_matches(content, &args.pattern, &args.replace).expect("Error while finding matches");
    
    let mut buffer = Vec::new();
    found_match_instances.read_to_end(&mut buffer).expect("Failed to read to buffer");

    let output = String::from_utf8(buffer).expect("Failed to convert to String");

    println!("{}", output);
}
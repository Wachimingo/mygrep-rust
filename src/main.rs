#![allow(unused)]
use std::{fmt, fs::File, io::{Error, ErrorKind, Write}};
use clap::{Parser};
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
    #[clap(short, long)]
    new_file: Option<std::path::PathBuf>,
}

impl fmt::Debug for Cli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("arguments")
            .field("pattern", &self.pattern)
            .field("path", &self.path)
            .field("replace", &self.path)
            .field("new_file", &self.new_file)
            .finish()
    }
}

fn main() {
    let args: Cli = Cli::parse();
    let content = mygrep::read_lines(&args.path).expect("Reading content");
    

    let mut found_match_instances = mygrep::find_matches(content, &args.pattern, &args.replace).expect("Error while finding matches");
    
    let mut buffer = Vec::new();
    found_match_instances.read_to_end(&mut buffer).expect("Failed to read to buffer");
    
    if args.replace.is_some() && args.new_file.is_some() {
        let mut new_file = File::create(args.new_file.unwrap()).expect("error");
        new_file.write_all(&buffer).expect("Failed to write file")
    } else {
        let output = String::from_utf8(buffer).expect("Failed to convert to String");
        println!("{}", output);
    }
}
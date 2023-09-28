#![allow(unused)]
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
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
    let lines = read_lines(&args.path).expect("Reading lines");
    for line in lines {
        if let Ok(words) = line {
            if words.contains(&args.pattern){
                println!("{}", words)
            }
        }
    }
    // println!("{:?}", args);
}

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where 
    P: AsRef<Path>, 
{
    let file = File::open(&filename)
        .with_context(|| format!("could not read file {}", filename.as_ref().display()))?;
    Ok(io::BufReader::new(file).lines())
}
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
    let content = read_lines(&args.path).expect("Reading content");
    find_matches(content, &args.pattern);
    // println!("{:?}", args);
}

fn find_matches(content: io::Lines<io::BufReader<File>>, pattern: &str){
    for lines in content {
        if let Ok(line) = lines {
            if line.contains(pattern){
                println!("{}", line)
            }
        }
    }
}

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where 
    P: AsRef<Path>, 
{
    let file = File::open(&filename)
        .with_context(|| format!("could not read file {}", filename.as_ref().display()))?;
    Ok(io::BufReader::new(file).lines())
}
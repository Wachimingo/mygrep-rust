#![allow(unused)]
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn find_matches(content: io::Lines<io::BufReader<File>>, pattern: &str, mut writer: impl std::io::Write){
    for lines in content {
        if let Ok(line) = lines {
            if line.contains(pattern){
                writeln!(writer, "{}", line);
            }
        }
    }
}

pub fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where 
    P: AsRef<Path>, 
{
    let file = File::open(&filename)
        .with_context(|| format!("could not read file {}", filename.as_ref().display()))?;
    Ok(io::BufReader::new(file).lines())
}
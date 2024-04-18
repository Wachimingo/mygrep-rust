#![allow(unused)]
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Find matches will return all the instances found with the given pattern and returned as a single string
pub fn find_matches(content: io::Lines<io::BufReader<File>>, pattern: &str, mut writer: impl std::io::Write) -> String{
    let mut match_instance: String = String::new();
    for lines in content {
        if let Ok(line) = lines {
            if line.contains(pattern){
                match_instance.push_str(&line);
                match_instance.push_str("\n");
            }
        }
    }
    return match_instance;
}

pub fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where 
    P: AsRef<Path>, 
{
    let file = File::open(&filename)
        .with_context(|| format!("could not read file {}", filename.as_ref().display()))?;
    Ok(io::BufReader::new(file).lines())
}
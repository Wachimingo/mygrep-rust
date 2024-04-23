// #![allow(unused)]
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, Cursor, Write, BufRead};
use std::path::Path;

/// Find matches will return all the instances found with the given pattern
/// Returns a buffer to allow for further data treatment e.g replacing
pub fn find_matches(content: io::Lines<io::BufReader<File>>, pattern: &str, replace: &Option<std::string::String>) -> io::Result<Cursor<Vec<u8>>> {
    let mut buffer = Vec::new();
    for lines in content {
        if let Ok(mut line) = lines {
            if line.contains(pattern){
                match replace {
                    Some(replace) => {
                        line = line.replace(&pattern, &replace);
                    }
                    None => {}
                }
                writeln!(&mut buffer, "{}", line)?            
            }
        }
    }
    Ok(Cursor::new(buffer))
}

pub fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where 
    P: AsRef<Path>, 
{
    let file = File::open(&filename)
        .with_context(|| format!("could not read file {}", filename.as_ref().display()))?;
    Ok(io::BufReader::new(file).lines())
}
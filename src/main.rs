#![allow(unused)]
use clap::Parser;
use std::fmt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli{
    //Pattern
    pattern: String,
    //Path of file to read
    path: std::path::PathBuf,
}

impl fmt::Display for Cli {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt:: Result {
        write!(f,"Looking for pattern {} in {}", self.pattern, self.path.display())    
    }
}
fn main() {
    let args = Cli::parse();
    println!("{}",args);
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    for line in content.lines() {
        if line.contains(&args.pattern){
            println!("{}", line);
        }
    }
}

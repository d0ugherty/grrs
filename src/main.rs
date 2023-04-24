#![allow(unused)]
use clap::Parser;
use std::fmt;
use std::io::{self, BufRead, BufReader, Read};
use std::fs::File;
use colored::*;
use std::io::Error;

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
        write!(f,"Looking for pattern \"{}\" in {}", self.pattern, self.path.display())    
    }
}

fn main() {
    let args = Cli::parse();
    println!("{}",args);
    let file = std::fs::File::open(&args.path).expect("could not read file");
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("could not read line");
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

}


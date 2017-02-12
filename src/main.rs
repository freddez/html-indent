#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use std::io::{Result, Read, BufRead, BufReader, Lines};

pub struct Html<A: Read> {
    lines: Lines<BufReader<A>>,
    indent_level: usize
}

impl<A: Read> Html<A> {
    fn new(fd: A) -> Html<A> {
        Html {
            lines: BufReader::new(fd).lines(),
            indent_level: 0,
        }
    }

    fn write(&mut self, s: &str) {
        print!("{}", s);
    }
    
    fn indent(mut self) {
        for (num, line) in self.lines.enumerate() {
            for word in line.unwrap().split_whitespace() {
                self.write(&word);
            }
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("No file specified");
        return;
    }
    let path = Path::new(&args[1]);
    let display = path.display();
    println!("The first argument is {}", args[1]);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };
    let buf = BufReader::new(file);
    let mut htmlp = Html::new(buf);
}

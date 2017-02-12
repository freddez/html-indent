#[macro_use] extern crate lazy_static;
extern crate regex;
// use regex::Regex;
use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::{BufRead, BufReader};

pub struct Html {
    path: String,
    indent_level: usize
}

impl Html {
    fn new(path: String) -> Html {
        Html {
            path: path,
            indent_level: 0,
        }
    }

    fn write(&self, s: &str) {
        print!("{}", s);
    }
    
    fn indent(&self) {
        let path = Path::new(&self.path);
        let display = path.display();
        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };
        let buf = BufReader::new(&file);
        for (num, line) in buf.lines().enumerate() {
            for word in line.unwrap().split_whitespace() {
                self.write(&word);
                self.write(" ");
            }
            self.write("\n");
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("No file specified");
        return;
    }
    let path = args[1].clone();
    println!("The first argument is {}",path);
    let htmlp = Html::new(path);
    htmlp.indent();
}

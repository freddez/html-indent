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
    
    fn write_indent(&self) {
        for i in 0..self.indent_level {
            print!(" ");
        }
    }
    
    fn indent(&mut self) {
        let mut tag_stack = Vec::<String>::new();
        let path = Path::new(&self.path);
        let display = path.display();
        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };
        let buf = BufReader::new(&file);
        let lines = buf.lines();
        for line in lines {
            self.write_indent();
            for word in line.unwrap().split_whitespace() {
                let mut chars = word.chars();
                if chars.next() == Some('<') {
                    if chars.next() == Some('/') {
                        self.indent_level -= 2;
                    }
                    else {
                        tag_stack.push(word.parse().unwrap());
                        self.indent_level += 2;
                    }
                }
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
    let mut htmlp = Html::new(path);
    htmlp.indent();
}

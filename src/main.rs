#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Html {
    s: String,
    i: usize,
    len: usize,
    indent_level: usize
}

impl Html {
    fn new(path: &Path) -> Html {
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };
        //let mut content = String::new();
        //let mut content;
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Err(why) => panic!("couldn't read {}: {}", display,
                               why.description()),
            Ok(_) => Html {
                s: content,
                i: 0,
                len:0,
                indent_level: 0,
            },
        }
        
    }

    fn indent(&mut self) {
        self.len = self.s.len();
        //let out = String::new();
        // let mut stack = Vec::new();
        while self.i < self.len {
            self.i = self.ltrim();
            println!("i : {}",self.i);
            
            break;
        }
        // return &out;
    }

    fn ltrim(&mut self) -> usize {
        lazy_static! {
            static ref RSPACE: Regex = Regex::new(r"[^ ]").unwrap();
        }
        let nospace = RSPACE.find(&self.s[self.i..]).unwrap();

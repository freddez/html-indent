#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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
        lazy_static! {
            static ref TAG: Regex = Regex::new("<(\"[^\"]*\"|'[^']*'|[^'\">])*>").unwrap();
        }
        let mut tag_stack = Vec::<String>::new();
        let mut i = 0;
        let path = Path::new(&self.path);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let mut start = 0;
        let mut end = 0;
        for tag in TAG.find_iter(&content) {
            start = tag.start();
            end = tag.end();
            println!("Tag : {} {} {}", start, end, tag.as_str());
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

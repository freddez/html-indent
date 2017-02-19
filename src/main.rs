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
    fn writeln(&self, s: &str) {
        println!("{}", s);
    }
    
    fn write_indent(&self) {
        for _ in 0..self.indent_level {
            print!("_");
        }
    }
    fn write_indent2(&self) {
        for _ in 0..self.indent_level {
            print!(".");
        }
    }
    
    fn indent(&mut self) {
        lazy_static! {
            static ref TAG: Regex = Regex::new(
                "<(?P<closing>/)?(?P<name>\\w+)(?P<attrs>(\"[^\"]*\"|'[^']*'|[^'\">])*)?>"
            ).unwrap();
        }
        let path = Path::new(&self.path);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let mut i=0;
        let mut after_newline = false;
        for tag in TAG.captures_iter(&content) {
            let tag_start = tag.get(0).unwrap().start();
            let tag_end = tag.name("attrs").unwrap().end() + 1;
            let txt = &content[i..tag_start].to_string();
            let mut iter_lines = txt.split("\n");
            let mut line = iter_lines.next();
            loop {
                let next = iter_lines.next();
                match line {                    
                    Some("") => {
                        print!("");
                        match(next) {
                            Some(expr) => {
                                self.writeln("");
                                after_newline = true;
                                line = next;
                            },
                            None => {
                                break;
                            }
                        }
                    }
                    Some(expr) => {
                        if after_newline {
                            self.write_indent2();
                            after_newline = false;
                        }
                        self.write(expr.trim());
                        match(next) {
                            Some(expr) => {
                                self.writeln("");
                                after_newline = true;
                                line = next;
                            },
                            None => {
                                break;
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
            
            if tag.name("closing").is_none() {
                if after_newline {
                    self.write_indent();
                }
                self.write(&content[tag_start..tag_end].to_string().trim());
                self.indent_level = self.indent_level + 2;
            }
            else {
                self.indent_level = self.indent_level - 2;
                if after_newline {
                    self.write_indent();
                }
                self.write(&content[tag_start..tag_end].to_string().trim());
            }
            after_newline = false;
            i = tag_end;
        }
        self.writeln("");
    }
}

fn main() {
    // let args: Vec<_> = env::args().collect();
    // if args.len() <= 1 {
    //     println!("No file specified");
    //     return;
    // }
    // let path = args[1].clone();
    let path = "/home/fredz/src/html-indent/test.kk";
    println!("The first argument is {}",path);
    let mut htmlp = Html::new(path.to_string());
    htmlp.indent();
}

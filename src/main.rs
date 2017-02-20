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
    after_newline: bool
}


impl Html {
    fn new(path: String) -> Html {
        Html {
            path: path,
            after_newline: false,
        }
    }

    fn write(&self, s: &str) {
        print!("{}", s);
    }

    fn writeln(&self, s: &str) {
        println!("{}", s);
    }

    fn write_indent(&self, level: usize) {
        for _ in 0..level {
            print!(" ");
        }
    }

    fn indent_lines(&mut self, str: &str, indent_level: usize, in_tag: bool) {
        let mut level = indent_level;
        let txt = str.to_string();
        let mut iter_lines = txt.split("\n");
        let mut line = iter_lines.next();
        let mut first_iter = true;
        loop {
            let next = iter_lines.next();
            if !line.is_some() {
                break;
            }
            let tline = line.unwrap().trim();
            if tline == "" {
                match next {
                    Some(_) => {
                        self.writeln("");
                        self.after_newline = true;
                        line = next;
                    },
                    None => {
                        break;
                    }
                }
            }
            else {
                if self.after_newline {
                    self.write_indent(level);
                    self.after_newline = false;
                }
                self.write(tline);
                match next {
                    Some(_) => {
                        self.writeln("");
                        self.after_newline = true;
                        line = next;
                    },
                    None => {
                        break;
                    }
                }
            }
            if first_iter {
                first_iter = false;
                if in_tag {
                    level += 6;
                }
            }
        }
    }

    fn indent(&mut self) {
        lazy_static! {
            static ref TAG: Regex = Regex::new(
                "<(?P<closing>/)?(?P<name>\\w+)(?P<attrs>(\"[^\"]*\"|'[^']*'|[^'\">])*)?>"
            ).unwrap();
        }
        let p = self.path.clone();
        let path = Path::new(&p);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let mut i=0;
        let mut indent_level = 0;
        for tag in TAG.captures_iter(&content) {
            let tag_start = tag.get(0).unwrap().start();
            let tag_end = tag.name("attrs").unwrap().end() + 1;
            self.indent_lines(&content[i..tag_start], indent_level, false);
            if tag.name("closing").is_none() {
                if self.after_newline {
                    self.write_indent(indent_level);
                }
                self.indent_lines(&content[tag_start..tag_end], indent_level, true);
                indent_level += 2;
            }
            else {
                indent_level -= 2;
                if self.after_newline {
                    self.write_indent(indent_level);
                }
                self.indent_lines(&content[tag_start..tag_end], indent_level, true);
            }
            self.after_newline = false;
            i = tag_end;
        }
        self.writeln("");
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("No file specified");
        return;
    }
    let path = args[1].clone();
    // let path = "/home/fredz/src/html-indent/test.kk";
    println!("The first argument is {}",path);
    let mut htmlp = Html::new(path.to_string());
    htmlp.indent();
}

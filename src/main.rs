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
            print!(" ");
        }
    }
    
    fn indent(&mut self) {
        lazy_static! {
            static ref TAG: Regex = Regex::new(
                "<(?P<closing>/)?(?P<name>\\w+)(?P<attrs>(\"[^\"]*\"|'[^']*'|[^'\">])*)?>"
            ).unwrap();
        }
        // let mut tag_stack = Vec::<String>::new();
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
        for tag in TAG.captures_iter(&content) {
            let tag_start = tag.get(0).unwrap().start();
            let tag_end = tag.name("attrs").unwrap().end() + 1;
            let txt = &content[i..tag_start].to_string();
            // println!("TXT [{}]", txt);
            // println!("TAG [{}]", &content[tag_start..tag_end].to_string());
            // println!("-->> {:?}", tag.name("name").unwrap().as_str());
            // println!("->> {} -> {} ", tag_start, tag_end);
            let mut iter_lines = txt.lines();
            //let mut line = iter_lines.next();
            let mut n = 0;
            loop { // UGLY, I DON'T KNOW HOW TO CATCH LAST ITERATION
                print!("{}", n);
                n += 1;
                let line = iter_lines.next();
                match line {
			        Some(expr) => self.write(expr.trim()),
			        None => break,
		        }
                self.writeln("$");
                self.write_indent();

                // match line {
			    //     Some(expr) => {
                //         println!("");
                //         self.write_indent();
                //     },
                //     None => continue,
		        // }
            }                
            if tag.name("closing").is_none() {
                self.indent_level = self.indent_level + 2;
            }
            else {
                self.indent_level = self.indent_level - 2;
            }
            print!("(");
            self.write(&content[tag_start..tag_end].to_string().trim());
            print!(")");
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
    println!("The first argument is {}",path);
    let mut htmlp = Html::new(path);
    htmlp.indent();
}

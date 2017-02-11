use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::Chars;

pub struct Html<'a> {
    iter: Chars<'a>,
    indent_level: usize
}

impl<'a> Html<'a> {
    fn new(path: &Path) -> Html {
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Err(why) => panic!("couldn't read {}: {}", display,
                               why.description()),
            Ok(_) => Html {
                iter: content.chars(),
                indent_level: 0,
            },
        }
        
    }

    fn indent(&mut self) -> &String {
        let out = String::new();
        // let mut stack = Vec::new();
        return &out;
    }

    fn ltrim(&mut self) {
        loop {
            match self.iter.next() {
                Some(x) => println!("{}", x),
                None => break,
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
    println!("The first argument is {}", args[1]);
    let mut htmlp = Html::new(path);
}

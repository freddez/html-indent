#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate env_logger;

extern crate regex;
extern crate walkdir;
use std::process;
use regex::Regex;
//use regex::bytes::Regex;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ascii::AsciiExt;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

pub struct Html {
    path: String,
    after_newline: bool,
    tag_stack: Vec<String>
}


impl Html {
    fn new(path: String) -> Html {
        Html {
            path: path,
            after_newline: false,
            tag_stack: Vec::new()
        }
    }

    fn write(&self, s: &str) {
        print!("{}", s);
    }

    // fn writet(&self, s: &str) {
    //     print!("[[{}]]", s);
    // }

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
                    level += 2;
                }
            }
        }
    }

    fn indent_tags(&mut self, content: &str, mut indent_level: usize) -> usize {
        lazy_static! {
            static ref TAG: Regex = Regex::new(
                "<(?P<closing>/)?(?P<name>\\w+)(?P<attrs>(\"[^\"]*\"|'[^']*'|[^'\">])*)?>"
            ).unwrap();
        }
        let self_closing_tags = vec![
            "area", "base", "br", "col", "command", "embed", "hr", "img", "input",
            "keygen", "link", "meta", "param", "source", "track", "wbr"
        ];
        let mut i=0;
        let mut tag_end = 0;
        for tag in TAG.captures_iter(&content) {
            let tag_start = tag.get(0).unwrap().start();
            tag_end = tag.name("attrs").unwrap().end() + 1;
            self.indent_lines(&content[i..tag_start], indent_level, false);
            let tag_name = tag.name("name").unwrap().as_str().clone().to_string();
            if tag.name("closing").is_none() {
                self.indent_lines(&content[tag_start..tag_end], indent_level, true);
                let mut self_closing = false;
                for self_closing_tag in &self_closing_tags {
                    if tag_name.eq_ignore_ascii_case(self_closing_tag) {
                        self_closing = true;
                        break;
                    }
                }
                if !self_closing {
                    let end_tag = &content[tag_start..tag_end-1].trim_right();
                    if end_tag.ends_with("/") {
                        self_closing = true;
                    }
                    if !self_closing {
                        self.tag_stack.push(tag_name.clone().to_string());
                        indent_level += 2;
                    }
                }
            }
            else {
                match self.tag_stack.pop() {
                    Some(open_tag) => if open_tag != tag_name {
                        println!("Expected </{}>, found </{}>", tag_name, open_tag);
                        process::exit(1);
                    },
                    None => {
                        println!("Missing closing tag for {}", tag_name);
                        process::exit(1)
                    }
                }
                indent_level -= 2;
                self.indent_lines(&content[tag_start..tag_end], indent_level, true);
            }
            self.after_newline = false;
            i = tag_end;
        }
        self.indent_lines(&content[tag_end..], indent_level, false);
        return indent_level;
    }
    
    fn indent_scripts(&mut self, content: &str, mut indent_level: usize) -> usize {
        lazy_static! {
            static ref SCRIPT: Regex = Regex::new(
                "(<script)(?P<attrs>(\"[^\"]*\"|'[^']*'|[^'\">])*)?>(?P<content>(.|\n)*)</script>"
            ).unwrap();
        }
        let mut i=0;
        let mut script_end = 0;
        for script in SCRIPT.captures_iter(&content) {
            let capture = script.get(0).unwrap();
            let script_start = capture.start();
            script_end = capture.end() + 1;
            indent_level = self.indent_tags(&content[i..script_start], indent_level);
            self.write_indent(indent_level);
            self.write("<script");
            if let Some(attrs) = script.name("attrs") {
                self.write(&attrs.as_str());
            }
            self.write(">");
            if let Some(content) = script.name("content") {            
                self.indent_lines(&content.as_str(), indent_level, true);
            }
            self.write_indent(indent_level);
            self.writeln("</script>");
            i = script_end;
        }
        indent_level = self.indent_tags(&content[script_end..], indent_level);
        return indent_level;
    }
    
    fn indent_comments(&mut self, content: &str) {
        lazy_static! {
            static ref COMMENT: Regex = Regex::new(
                "<!--(.|\n)*-->"
            ).unwrap();
        }
        let mut indent_level = 0;
        let mut i=0;
        let mut comment_end = 0;
        for comment in COMMENT.find_iter(&content) {
            let comment_start = comment.start();
            comment_end = comment.end() + 1;
            indent_level = self.indent_scripts(&content[i..comment_start], indent_level);
            self.write(&content[comment_start..comment_end]);
            i = comment_end;
        }
        self.indent_tags(&content[comment_end..], indent_level);
    }
    
    fn indent(&mut self) {
        let p = self.path.clone();
        let path = Path::new(&p);
        let display = path.display();
        info!("Processing {:?}", path);
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        self.indent_comments(&content);
    }
}


fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn process_dir(dirname: String) {
    let file_pattern = Regex::new("^(.*\\.html)$").unwrap(); // TODO : input wildcard to regex

    for entry in WalkDir::new(dirname).into_iter().filter_entry(|e| !is_hidden(e)) {
    debug!("Ben quoi?");
        let entry = match entry {
            Ok(f) => f,
            Err(e) => {
                warn!("Error while walking directories: {}", e);
                continue;
            }
        };
        let path = entry.path();
        debug!("Processing entry {:?}", path);
        if file_pattern.is_match(entry.path().to_str().unwrap()) {
            if let Some(filename) = path.to_str() {
                let mut htmlp = Html::new(filename.to_string());
                htmlp.indent();
            }
        }
    }
}

fn main() {
    env_logger::init().unwrap();
    warn!("starting up");
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("No file specified");
        return;
    }
    if args[1] == "-r" {
        match args.len() > 2 {
            true => process_dir(args[2].clone()),
            false => {
                match env::current_dir().unwrap().to_str() {
                    Some(dirname) => process_dir(dirname.to_string()),
                    None => error!("Can't get current working directory")
                }
            }
        }
        
    }
    else {
        let path = args[1].clone();
        let mut htmlp = Html::new(path.to_string());
        htmlp.indent();
    }
}

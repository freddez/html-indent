#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate getopts;
use getopts::Options;
extern crate regex;
extern crate walkdir;
use std::process;
use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::ascii::AsciiExt;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

pub struct Html {
    output: String,
    line_number: usize,
    after_newline: bool,
    tag_stack: Vec<String>,
    dry_run: bool,
    print: bool
}


impl Html {
    fn new(dry_run:bool, print: bool) -> Html {
        Html {
            output: String::new(),
            line_number: 1,
            after_newline: false,
            tag_stack: Vec::new(),
            dry_run: dry_run,
            print: print
        }
    }

    fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn writeln(&mut self, s: &str) {
        self.write(s);
        self.output.push_str("\n");
        self.line_number += 1
    }

    fn write_indent(&mut self, level: usize) {
        for _ in 0..level {
            self.output.push_str(" ");
        }
    }

    fn print_output(&self) {
        if self.print {
            print!("{}", self.output);
        }
    }

    fn indent_lines(&mut self, str: &str, indent_level: usize, in_tag: bool, keep_indent: bool) {
        lazy_static! {
            static ref NON_W: Regex = Regex::new(r"\S").unwrap();
        }
        let mut level = indent_level;
        let txt = str.to_string();
        let mut iter_lines = txt.split("\n");
        let mut line = iter_lines.next();
        let mut block_position = 0;
        let mut first_iter = true;
        loop {
            let next = iter_lines.next();
            if !line.is_some() {
                break;
            }
            let tline = line.unwrap().trim_right();
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
                let mut nw_position = NON_W.find(&tline).unwrap().start();
                if keep_indent {
                    if block_position == 0 {
                        block_position = match NON_W.find(&tline) {
                            Some(r) => r.start(),
                            None => 0
                        };
                    }
                    if nw_position >= block_position {
                        nw_position -= nw_position - block_position;
                    }
                }
                self.write(&tline[nw_position..]);
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
            self.indent_lines(&content[i..tag_start], indent_level, false, false);
            let tag_name = tag.name("name").unwrap().as_str().clone().to_string();
            if tag.name("closing").is_none() {
                self.indent_lines(&content[tag_start..tag_end], indent_level, true, false);
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
                        self.print_output();
                        error!("Line {}, expected </{}>, found </{}>",
                               self.line_number, open_tag, tag_name);
                        process::exit(1);
                    },
                    None => {
                        self.print_output();
                        error!("Missing closing tag for {}", tag_name);
                        process::exit(1)
                    }
                }
                indent_level -= 2;
                self.indent_lines(&content[tag_start..tag_end], indent_level, true, false);
            }
            self.after_newline = false;
            i = tag_end;
        }
        self.indent_lines(&content[tag_end..], indent_level, false, false);
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
                self.indent_lines(&content.as_str(), indent_level, true, true);
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
            static ref COMMENT: Regex = Regex::new(r"<!--[\s\S]*?-->").unwrap();
        }
        let mut indent_level = 0;
        let mut i=0;
        let mut comment_end = 0;
        self.output = String::with_capacity(content.len());
        for comment in COMMENT.find_iter(&content) {
            let comment_start = comment.start();
            comment_end = comment.end();
            indent_level = self.indent_scripts(&content[i..comment_start], indent_level);
            //self.write(&content[comment_start..comment_end]);
            self.indent_lines(&content[comment_start..comment_end], indent_level, true, true);
            i = comment_end;
        }
        self.indent_tags(&content[comment_end..], indent_level);
        self.print_output();
    }

    fn indent(&mut self, path: Option<String>) {
        match path {
            Some(file_path) => {
                let p = file_path.clone();
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
                if !self.dry_run && self.tag_stack.is_empty() {
                    let mut file = match File::create(&file_path) {
                        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
                        Ok(file) => file,
                    };
                    match file.write(self.output.as_bytes()) {
                        Err(why) => panic!("couldn't write {}: {}", display, why.description()),
                        Ok(_) => {},
                    };
                }
            },
            None => {
                let mut content = String::new();
                io::stdin().read_to_string(&mut content);
                self.indent_comments(&content);
                print!("{}", self.output);
            }
        };
        for tag in self.tag_stack.pop() {
            error!("Missing closing tag for {}", tag);
        }
    }
}


fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn process_dir(dirname: String, file_ext: &str, dry_run: bool, print: bool) {
    for entry in WalkDir::new(dirname).into_iter().filter_entry(|e| !is_hidden(e)) {
        let entry = match entry {
            Ok(f) => f,
            Err(e) => {
                warn!("Error while walking directories: {}", e);
                continue;
            }
        };
        let path = entry.path();
        if path.to_str().unwrap().ends_with(file_ext) {
            debug!("Processing entry {:?}", path);
            if let Some(filename) = path.to_str() {
                let mut htmlp = Html::new(dry_run, print);
                htmlp.indent(Some(filename.to_string()));
            }
        }
    }
}

fn print_usage(opts: Options) {
    let brief = format!("Usage: html-indent [FILE] [options]");
    print!("{}", opts.usage(&brief));
}

fn main() {
    env_logger::init().unwrap();
    debug!("starting up");
    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("r", "recursive", "process all files in directory tree");
    opts.optopt("e", "extension", "file extension for recursive processing", "ext");
    opts.optflag("n", "dry-run", "dry run, don't write files");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("p", "print", "print html result to stdout");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }
    let print = matches.opt_present("p");
    let dry_run = matches.opt_present("n");
    let recursive = matches.opt_present("r");
    let extension = matches.opt_str("e");

    if recursive {
        let path = if !matches.free.is_empty() {
            matches.free[0].clone()
        } else {
            match env::current_dir().unwrap().to_str() {
                Some(dirname) => dirname.to_string(),
                None => {
                    error!("Can't get current working directory");
                    return;
                }
            }
        };
        match extension {
            Some(ext) => process_dir(path, ext.as_str(), dry_run, print),
            None => process_dir(path, "html", dry_run, print),
        }
    }
    else {
        let path: Option<String> = if !matches.free.is_empty() {
            Some(matches.free[0].clone())
        } else {
            None
        };
        let mut htmlp = Html::new(dry_run, print);
        htmlp.indent(path);
    }
}

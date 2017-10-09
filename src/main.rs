mod log_line;
mod tor_log;

use std::env;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::vec::Vec;
use std::str::Lines;

use log_line::LogLine;

/* ---------------------------------------- */

fn process_file(filename: &String) {
    let path = Path::new(filename);
    let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open file {}: {}", path.display(), e.description()),
        Ok(file) => file
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(e) => panic!("Unable to read file {}: {}", path.display(), e.description()),
        Ok(_) => (),
    }

    process_lines(content.lines())
}

fn process_lines(lines: Lines) {
    let mut vec: Vec<LogLine> = Vec::new();

    for s in lines {
        vec.push(LogLine::new(s.to_string()));
    }

    process_log_lines(vec)
}

fn process_log_lines(lines: Vec<LogLine>) {
    for line in lines {
        println!("{}", line);
    }
}

/* ---------------------------------------- */

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("Usage: {} <filename>", &args[0]);
        },
        _ => {
            let filename = &args[1];
            process_file(filename);
        }
    }
}

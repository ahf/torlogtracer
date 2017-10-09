use std::env;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Read;

fn process_file(filename: &String) {
    let path = Path::new(filename);
    let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open file {}: {}", path.display(), e.description()),
        Ok(file) => file
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(e) => panic!("Unable to read file {}: {}", path.display(), e.description()),
        Ok(_) => println!("Content '{}'", content)
    }
}

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

#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
mod compiler;
mod errors;
#[allow(unused_imports)]
use compiler::{parser, tokenizer, tokens};
use std::path::Path;
#[macro_use]
mod native_interface;

fn parse_args() -> Vec<String> {
    std::env::args().collect::<Vec<String>>()
}

fn main() {
    //TODO: Parallelize multiple files/ Libs to compile
    let args = parse_args();
    match args.get(1) {
        Some(s) if Path::new(s).is_dir() => {
            std::env::set_current_dir(Path::new(s)).expect("Can't change to desired directory!");
            let current_directory =
                std::env::current_dir().expect("Can't determine current working directory!");
            println!("{:?}", current_directory);
            //TODO: check for commands or files repl etc.
        }
        Some(command) if Path::new("glisp.toml").is_file() => {
            println!("Hello {}", { command });
        }
        Some(filename) if Path::new(filename).is_file() => {
            let contents =
                std::fs::read_to_string(filename).expect("Something went wrong reading the file");
            match tokenizer::tokenize(filename, contents) {
                Ok(t) => {
                    println!("{:?}", t);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        _ => {
            println!("Incorrect arguments supplied!");
        }
    }
}

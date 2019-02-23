#![allow(dead_code)]
mod compiler;
mod errors;
#[allow(unused_imports)]
use compiler::{parser, tokenizer, tokens};

#[macro_use]
mod native_interface;

fn parse_args() -> Vec<String> {
    std::env::args().collect::<Vec<String>>()
}

fn main() {
    //TODO: Parallelize multiple files/ Libs to compile
    let args = parse_args();
    if let Some(filename) = args.get(1) {
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
    } else {
        println!("Incorrect number of arguments supplied!");
    }
}

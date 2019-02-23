#![allow(dead_code)]
mod errors;
mod parser;
mod tokenizer;
mod tokens;

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
        let tokens = tokenizer::tokenize(contents);
        match tokens {
            Ok(t) => println!("{:?}", t),
            Err(_) => {}
        }
    } else {
        println!("Incorrect number of arguments supplied!");
    }
}

mod errors;
mod parser;

use parser::{construct_ast};
#[macro_use]
mod native_interface;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if let Some(filename) = args.get(1) {
        let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
        match construct_ast(contents) {
            Ok(node) => {
                println!("{}", node);
            }
            Err(errors::ErrorCode::GENERAL(a)) => {
                println!("{}", a);
            }
        };
    } else {
        println!("Incorrect number of arguments supplied!");
    }
}

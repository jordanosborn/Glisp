mod parser;
use parser::{construct_ast, Node};
#[macro_use]
mod native_interface;

c_function!(i32, test);

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if let Some(filename) = args.get(1) {
        let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
        println!("{}", contents);
        construct_ast(contents);
    } else {
        println!("Incorrect number of arguments supplied!");
    }
    let p = Node::new()
        .value("Hello")
        .add_child(Node::new().value("HI"))
        .add_children(&mut vec![
            Node::new().value("Heath"),
            Node::new().add_child(Node::new().value("Hi")),
        ]);
    println!("{}", p);
    unsafe {
        println!("{}", test());
    }
}

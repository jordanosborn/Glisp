#![allow(dead_code, unused_variables)]
use std::fs;
use std::env;

struct Node {
    value: String,
    children: Vec<Node>
}

impl Node {
    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self
    }
    pub fn add_child(mut self, child: Node) -> Self {
        self.children.append(&mut vec![child]);
        self
    }
    pub fn add_children(mut self, child: &mut Vec<Node>) -> Self {
        self.children.append(child);
        self
    }
    pub fn new() -> Node {
        Node {
            value: String::from(""),
            children: Vec::new()
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} ", self.value)?;
        self.children.iter().for_each(|x| {
            write!(f, "{}", x).expect("");
        });
        write!(f, ")")
    }
}

fn build(contents: String) -> Node {
    Node::new()
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("{}", contents);
    build(contents);
    let p = Node::new().value(String::from("Hello")).add_child(Node::new().value(String::from("HI"))).add_child(Node::new().value(String::from("Heasthr")));
    println!("{}", p);
}

use std::collections::LinkedList;
use crate::tokens::*;

pub struct Node {
    pub value: String,
    pub children: Vec<Node>,
}
use crate::errors::ErrorCode;

#[allow(dead_code)]
impl Node {
    pub fn value(mut self, value: &str) -> Self {
        self.value = String::from(value);
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
            value: String::from("()"),
            children: Vec::new(),
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

pub fn parse(_tokens: LinkedList<(Token, MetaData)>) -> Result<Node, ErrorCode> {
    Ok(Node::new())
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn t() {
    //     assert!(6 == 6)
    // }
}

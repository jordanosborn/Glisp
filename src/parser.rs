pub struct Node {
    value: String,
    children: Vec<Node>,
}

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

pub fn construct_ast(contents: String) -> Node {
    Node::new()
}

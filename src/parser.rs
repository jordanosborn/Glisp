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

pub fn construct_ast(contents: String) -> Result<Node, ErrorCode> {
    let ast = Node::new();
    let mut inside_comment = false;
    let mut inside_string = false;
    let mut nesting_level: i64 = 0;
    let mut previous_char: char = '\0';

    let mut line_number: usize = 1;
    let mut line_beginning: usize = 0;
    for (index, c) in contents.char_indices() {
        if ! c.is_ascii() {
            return Err(ErrorCode::GENERAL(format!("Error at {}:{} -> Non-Ascii character {} ", line_number, index - line_beginning + 1, c)));
        } else if c == '\n' {
            line_number += 1;
            line_beginning = index + 1;
        }
    }

    let mut line_number: usize = 1;
    let mut line_beginning: usize = 0;
    let contents = contents.as_bytes();

    for (index, c) in contents.iter().enumerate() {
        if inside_comment && *c as char != '\n' {
            continue;
        }
        match *c as char {
            '(' if previous_char != '\\' => {
                nesting_level += 1;
            }
            ')' if previous_char != '\\' => {
                nesting_level -= 1;
                if nesting_level < 0 {
                    return Err(
                        ErrorCode::GENERAL(format!("Error at {}:{} -> Unmatched brace ", line_number, index - line_beginning + 1))
                    );
                }
            }
            '\n' if ! inside_string => {
                inside_comment = false;
                line_number += 1;
                line_beginning = index + 1;
            }
            ';' if previous_char != '\\' && ! inside_string && ! inside_comment => {
                inside_comment = true;
                continue;
            }
            '"' if previous_char != '\\' => {
                if inside_string {
                    inside_string = false;

                } else {

                }
            }
            _ => {

            }
        }
        previous_char = *c as char;
        println!("{}: {}", index, previous_char);
    }
    if nesting_level == 0 {
        Ok(ast)
    } else {
        Err(ErrorCode::GENERAL(format!("Error at {}:{} -> Unmatched brace ", line_number, line_beginning + 1)))
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn t() {
    //     assert!(6 == 6)
    // }
}
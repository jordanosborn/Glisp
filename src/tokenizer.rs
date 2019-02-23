use crate::errors::ErrorCode;
use crate::types::*;
use std::collections::LinkedList;

#[derive(Debug)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    Ident,
    Type(Type),
    Builtin(Keyword),
    String(String),
    Comment(String),
    Rational(Rational),
    Newline,
}

#[derive(Debug)]
pub struct MetaData {
    pub line_no: usize,
    pub start: usize,
    pub end: usize,
}

pub fn tokenizer(contents: String) -> LinkedList<(Token, MetaData)> {
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut token_stack = LinkedList::new();
    let mut inside_string = false;
    let mut previous_character = '\0';
    for (line_no, line) in lines.iter().enumerate() {
        let mut inside_comment = false;
        let mut comment_string = String::from("");
        let mut comment_metadata = MetaData {
            line_no,
            start: 0,
            end: 0,
        };
        for (c_index, character) in (*line).char_indices() {
            match character {
                c if inside_comment => {
                    comment_string.push(c);
                }
                '(' => token_stack.push_back((
                    Token::OpenBrace,
                    MetaData {
                        line_no,
                        start: c_index,
                        end: c_index,
                    },
                )),
                ')' => token_stack.push_back((
                    Token::CloseBrace,
                    MetaData {
                        line_no,
                        start: c_index,
                        end: c_index,
                    },
                )),
                ';' if !(inside_comment || inside_string) && previous_character != '\\' => {
                    inside_comment = true;
                    comment_metadata.start = c_index;
                }
                _ => {}
            }
        }
        if comment_string.len() != 0 {
            token_stack.push_back((
                Token::Comment(String::from(comment_string)),
                MetaData {
                    end: line.len() - 1,
                    ..comment_metadata
                },
            ))
        }
        token_stack.push_back((
            Token::Newline,
            MetaData {
                line_no,
                start: line.len(),
                end: line.len(),
            },
        ));
    }
    token_stack
}
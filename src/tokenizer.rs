use crate::errors::ErrorCode;
use crate::tokens::*;
use std::collections::LinkedList;

//TODO: Check for matching closing tokens report errors
fn check_closing_tokens(_tokens: &LinkedList<(Token, MetaData)>) -> Option<Vec<ErrorCode>> {
    None
}

pub fn tokenize(contents: String) -> Result<LinkedList<(Token, MetaData)>, Vec<ErrorCode>> {
    let lines = contents.split('\n').collect::<Vec<&str>>();
    let mut token_stack = LinkedList::new();
    let mut inside_string = false;
    let mut string_string = String::from("");
    let mut string_metadata = MetaData {
        line_no: 0,
        start: 0,
        end: 0,
        line_no_end: None,
    };
    let mut previous_character = '\0';
    for (line_no, line) in lines.iter().enumerate() {
        let mut other_string = String::from("");
        let mut other_string_metadata = MetaData {
            line_no,
            start: 0,
            end: 0,
            line_no_end: None,
        };
        let mut inside_comment = false;
        let mut comment_string = String::from("");
        let mut comment_metadata = MetaData {
            line_no,
            start: 0,
            end: 0,
            line_no_end: None,
        };
        for (c_index, character) in (*line).char_indices() {
            match character {
                ';' if !(inside_comment || inside_string) && previous_character != '\\' => {
                    inside_comment = true;
                    comment_metadata.start = c_index;
                }
                '"' if !inside_comment && previous_character != '\\' => {
                    if inside_string {
                        token_stack.push_back((
                            Token::String(string_string),
                            MetaData {
                                end: c_index,
                                line_no_end: Some(line_no),
                                ..string_metadata
                            },
                        ));
                        string_string = String::from("");
                        string_metadata = MetaData {
                            line_no: 0,
                            start: 0,
                            end: 0,
                            line_no_end: None,
                        };
                        token_stack.push_back((
                            Token::CloseQuote,
                            MetaData {
                                line_no,
                                start: c_index,
                                end: c_index,
                                line_no_end: None,
                            },
                        ));
                    } else {
                        string_metadata = MetaData {
                            line_no,
                            start: c_index,
                            end: 0,
                            line_no_end: None,
                        };
                        token_stack.push_back((
                            Token::OpenQuote,
                            MetaData {
                                line_no,
                                start: c_index,
                                end: c_index,
                                line_no_end: None,
                            },
                        ));
                    }
                    inside_string = !inside_string;
                }
                c if inside_comment => {
                    comment_string.push(c);
                }
                c if inside_string => string_string.push(c),
                '(' if previous_character != '\\' => token_stack.push_back((
                    Token::OpenBrace,
                    MetaData {
                        line_no,
                        start: c_index,
                        end: c_index,
                        line_no_end: None,
                    },
                )),
                ')' if previous_character != '\\' => token_stack.push_back((
                    Token::CloseBrace,
                    MetaData {
                        line_no,
                        start: c_index,
                        end: c_index,
                        line_no_end: None,
                    },
                )),
                ' ' | '\t' => {
                    if !other_string.is_empty() {
                        token_stack.push_back((
                            Token::Other(other_string),
                            MetaData {
                                end: c_index - 1,
                                ..other_string_metadata
                            },
                        ));
                        other_string = String::from("");
                        other_string_metadata = MetaData {
                            line_no,
                            start: 0,
                            end: 0,
                            line_no_end: None,
                        };
                    }
                }
                c if !(inside_string || inside_comment) => {
                    if other_string.is_empty() {
                        other_string_metadata = MetaData {
                            start: c_index,
                            line_no,
                            end: c_index,
                            line_no_end: None,
                        };
                    }
                    other_string.push(c);
                }
                _ => {}
            }
            previous_character = character;
        }
        if !comment_string.is_empty() {
            token_stack.push_back((
                Token::Comment(comment_string),
                MetaData {
                    end: line.len() - 1,
                    ..comment_metadata
                },
            ))
        }
        if !other_string.is_empty() {
            token_stack.push_back((
                Token::Other(other_string),
                MetaData {
                    end: line.len() - 1,
                    ..other_string_metadata
                },
            ));
        }
        if inside_string {
            string_string.push('\n');
        } else {
            token_stack.push_back((
                Token::Newline,
                MetaData {
                    line_no,
                    start: line.len(),
                    end: line.len(),
                    line_no_end: None,
                },
            ));
        }
    }
    if let Some(errs) = check_closing_tokens(&token_stack) {
        Err(errs)
    } else {
        tokenize_pass2(token_stack)
    }
}

fn tokenize_pass2(
    tokens: LinkedList<(Token, MetaData)>,
) -> Result<LinkedList<(Token, MetaData)>, Vec<ErrorCode>> {
    Ok(tokens)
}


#[cfg(test)]
mod tests {
    // #[test]
    // fn t() {
    //     assert!(6 == 6)
    // }
}

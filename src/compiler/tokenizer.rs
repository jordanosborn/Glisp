use super::syntax;
use super::tokens::*;
use crate::errors::{ErrorCode, ErrorCodeList};
use std::collections::LinkedList;

//TODO: chars and multi-line comments

fn check_closing_tokens<'a>(
    tokens: LinkedList<(Token, MetaData<'a>)>,
) -> (Option<ErrorCodeList>, LinkedList<(Token, MetaData<'a>)>) {
    let mut errs = ErrorCodeList::new();
    let mut unmatched_tokens = Vec::new();
    for (token, metadata) in tokens.iter() {
        match token {
            Token::OpenBrace
            | Token::OpenQuote
            | Token::OpenSquareBrace
            | Token::OpenCurlyBrace
            | Token::OpenAngularBrace => {
                unmatched_tokens.push((token, metadata));
            }
            Token::CloseQuote => {
                if let Some(t) = unmatched_tokens.last() {
                    if *t.0 == Token::OpenQuote {
                        unmatched_tokens.pop();
                    } else {
                        errs.push(ErrorCode::SyntaxError(
                            String::from("Unmatched closing quote '\"'!"),
                            Token::CloseQuote,
                            *metadata,
                        ));
                    }
                } else {
                    errs.push(ErrorCode::SyntaxError(
                        String::from("Unmatched closing quote '\"'!"),
                        Token::CloseQuote,
                        *metadata,
                    ));
                }
            }
            Token::CloseBrace => {
                if let Some(t) = unmatched_tokens.last() {
                    if *t.0 == Token::OpenBrace {
                        unmatched_tokens.pop();
                    } else {
                        errs.push(ErrorCode::SyntaxError(
                            String::from("Unmatched closing brace ')' !"),
                            Token::CloseBrace,
                            *metadata,
                        ));
                    }
                } else {
                    errs.push(ErrorCode::SyntaxError(
                        String::from("Unmatched closing brace ')' !"),
                        Token::CloseBrace,
                        *metadata,
                    ));
                }
            }
            Token::CloseSquareBrace => {
                if let Some(t) = unmatched_tokens.last() {
                    if *t.0 == Token::OpenSquareBrace {
                        unmatched_tokens.pop();
                    } else {
                        errs.push(ErrorCode::SyntaxError(
                            String::from("Unmatched closing square brace ']' !"),
                            Token::CloseSquareBrace,
                            *metadata,
                        ));
                    }
                } else {
                    errs.push(ErrorCode::SyntaxError(
                        String::from("Unmatched closing square brace ']' !"),
                        Token::CloseSquareBrace,
                        *metadata,
                    ));
                }
            }
            Token::CloseCurlyBrace => {
                if let Some(t) = unmatched_tokens.last() {
                    if *t.0 == Token::OpenCurlyBrace {
                        unmatched_tokens.pop();
                    } else {
                        errs.push(ErrorCode::SyntaxError(
                            String::from("Unmatched closing curly brace '}' !"),
                            Token::CloseCurlyBrace,
                            *metadata,
                        ));
                    }
                } else {
                    errs.push(ErrorCode::SyntaxError(
                        String::from("Unmatched closing curly brace '}' !"),
                        Token::CloseSquareBrace,
                        *metadata,
                    ));
                }
            }
            Token::CloseAngularBrace => {
                if let Some(t) = unmatched_tokens.last() {
                    if *t.0 == Token::OpenAngularBrace {
                        unmatched_tokens.pop();
                    } else {
                        errs.push(ErrorCode::SyntaxError(
                            String::from("Unmatched closing angular brace '>' !"),
                            Token::CloseAngularBrace,
                            *metadata,
                        ));
                    }
                } else {
                    errs.push(ErrorCode::SyntaxError(
                        String::from("Unmatched closing angular brace '>' !"),
                        Token::CloseAngularBrace,
                        *metadata,
                    ));
                }
            }
            _ => {}
        }
    }
    if !unmatched_tokens.is_empty() {
        for (token, metadata) in unmatched_tokens.iter() {
            match token {
                Token::OpenQuote => {
                    errs.push(ErrorCode::SyntaxError(
                        String::from("Unmatched opening quote '\"'!"),
                        Token::OpenQuote,
                        **metadata,
                    ));
                }
                Token::OpenBrace => {
                    errs.push(ErrorCode::SyntaxError(
                        String::from("Unmatched opening brace '(' !"),
                        Token::OpenBrace,
                        **metadata,
                    ));
                }
                Token::OpenSquareBrace => {
                    errs.push(ErrorCode::SyntaxError(
                        String::from("Unmatched opening square brace '[' !"),
                        Token::OpenSquareBrace,
                        **metadata,
                    ));
                }
                Token::OpenCurlyBrace => {
                    errs.push(ErrorCode::SyntaxError(
                        String::from("Unmatched opening curly brace '{' !"),
                        Token::OpenCurlyBrace,
                        **metadata,
                    ));
                }
                Token::OpenAngularBrace => {
                    errs.push(ErrorCode::SyntaxError(
                        String::from("Unmatched opening angular brace '<' !"),
                        Token::OpenAngularBrace,
                        **metadata,
                    ));
                }
                _ => {}
            }
        }
    }
    if errs.is_empty() {
        (None, tokens)
    } else {
        (Some(errs), tokens)
    }
}

#[allow(clippy::cyclomatic_complexity)]
pub fn tokenize<'a>(
    filename: &'a str,
    contents: String,
) -> Result<LinkedList<(Token, MetaData<'a>)>, ErrorCodeList> {
    let lines = contents.split('\n').collect::<Vec<&str>>();
    let mut token_stack = LinkedList::new();
    let mut inside_string = false;
    let mut string_string = String::from("");
    let mut string_metadata = MetaData {
        line_no: 0,
        start: 0,
        end: 0,
        line_no_end: None,
        filename,
    };
    let mut previous_character = '\0';
    for (line_no, line) in lines.iter().enumerate() {
        let mut start_of_line = true;
        let mut space_indent_count = 0;
        let mut tab_indent_count = 0;

        let mut other_string = String::from("");
        let mut other_string_metadata = MetaData {
            line_no,
            start: 0,
            end: 0,
            line_no_end: None,
            filename,
        };
        let mut inside_comment = false;
        let mut comment_string = String::from("");
        let mut comment_metadata = MetaData {
            line_no,
            start: 0,
            end: 0,
            line_no_end: None,
            filename,
        };
        for (c_index, character) in (*line).char_indices() {
            if start_of_line && character != ' ' && character != '\t' {
                start_of_line = false;
                if space_indent_count != 0 || tab_indent_count != 0 {
                    token_stack.push_back((
                        Token::Indentation((space_indent_count, tab_indent_count)),
                        MetaData {
                            start: 0,
                            end: (space_indent_count + tab_indent_count - 1) as usize,
                            line_no,
                            line_no_end: None,
                            filename,
                        },
                    ));
                }
            }
            match character {
                ' ' if start_of_line => {
                    space_indent_count += 1;
                }
                '\t' if start_of_line => {
                    tab_indent_count += 1;
                }
                c if c == syntax::comment()
                    && !(inside_comment || inside_string)
                    && previous_character != '\\' =>
                {
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
                            filename,
                        };
                        token_stack.push_back((
                            Token::CloseQuote,
                            MetaData {
                                line_no,
                                start: c_index,
                                end: c_index,
                                line_no_end: None,
                                filename,
                            },
                        ));
                    } else {
                        string_metadata = MetaData {
                            line_no,
                            start: c_index,
                            end: 0,
                            line_no_end: None,
                            filename,
                        };
                        token_stack.push_back((
                            Token::OpenQuote,
                            MetaData {
                                line_no,
                                start: c_index,
                                end: c_index,
                                line_no_end: None,
                                filename,
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
                        filename,
                    },
                )),
                ')' if previous_character != '\\' => token_stack.push_back((
                    Token::CloseBrace,
                    MetaData {
                        line_no,
                        start: c_index,
                        end: c_index,
                        line_no_end: None,
                        filename,
                    },
                )),
                '[' if previous_character != '\\' => token_stack.push_back((
                    Token::OpenSquareBrace,
                    MetaData {
                        line_no,
                        start: c_index,
                        end: c_index,
                        line_no_end: None,
                        filename,
                    },
                )),
                ']' if previous_character != '\\' => token_stack.push_back((
                    Token::CloseSquareBrace,
                    MetaData {
                        line_no,
                        start: c_index,
                        end: c_index,
                        line_no_end: None,
                        filename,
                    },
                )),
                '{' if previous_character != '\\' => token_stack.push_back((
                    Token::OpenCurlyBrace,
                    MetaData {
                        line_no,
                        start: c_index,
                        end: c_index,
                        line_no_end: None,
                        filename,
                    },
                )),
                '}' if previous_character != '\\' => token_stack.push_back((
                    Token::CloseCurlyBrace,
                    MetaData {
                        line_no,
                        start: c_index,
                        end: c_index,
                        line_no_end: None,
                        filename,
                    },
                )),
                '<' if previous_character != '\\' => token_stack.push_back((
                    Token::OpenAngularBrace,
                    MetaData {
                        line_no,
                        start: c_index,
                        end: c_index,
                        line_no_end: None,
                        filename,
                    },
                )),
                '>' if previous_character != '\\' => token_stack.push_back((
                    Token::CloseAngularBrace,
                    MetaData {
                        line_no,
                        start: c_index,
                        end: c_index,
                        line_no_end: None,
                        filename,
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
                            filename,
                        };
                    }
                }
                c if syntax::is_literal(&c.to_string()) => token_stack.push_back((
                    Token::Literal(c),
                    MetaData {
                        line_no,
                        start: c_index,
                        end: c_index,
                        line_no_end: None,
                        filename,
                    },
                )),
                c if !(inside_string || inside_comment) => {
                    if other_string.is_empty() {
                        other_string_metadata = MetaData {
                            start: c_index,
                            line_no,
                            end: c_index,
                            line_no_end: None,
                            filename,
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
                    filename,
                },
            ));
        }
    }

    let (valid_closing_tokens, token_stack) = check_closing_tokens(token_stack);
    if let Some(errs) = valid_closing_tokens {
        Err(errs)
    } else {
        tokenize_pass2(token_stack)
    }
}

fn tokenize_pass2<'a>(
    tokens: LinkedList<(Token, MetaData<'a>)>,
) -> Result<LinkedList<(Token, MetaData<'a>)>, ErrorCodeList> {
    let mut token_stack = LinkedList::new();
    for t in tokens.iter() {
        match t {
            (Token::Other(s), metadata) => {
                //TODO: finish second pass convert Other tokens in to other types
                match s {
                    _ => token_stack.push_back((Token::Other(s.clone()), *metadata)),
                }
            }
            t => {
                token_stack.push_back(t.clone());
            }
        }
    }
    Ok(token_stack)
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn t() {
    //     assert!(6 == 6)
    // }
}

use crate::tokens;
#[derive(Debug)]
pub enum ErrorCode {
    General(String),
    SyntaxError(String, tokens::Token, tokens::MetaData),
}

pub struct ErrorCodeList(Vec<ErrorCode>);

impl ErrorCodeList {
    pub fn new() -> ErrorCodeList {
        ErrorCodeList(Vec::new())
    }

    pub fn from(v: Vec<ErrorCode>) -> ErrorCodeList {
        ErrorCodeList(v)
    }

    pub fn push(&mut self, err: ErrorCode) {
        self.0.push(err);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorCode::General(s) => write!(f, "{}", s),
            ErrorCode::SyntaxError(string, _, metadata) => {
                if let Some(end_line) = metadata.line_no_end {
                    write!(
                        f,
                        "Error at {}: {}, {}: {} -> {}",
                        metadata.line_no, metadata.start, end_line, metadata.end, string
                    )
                } else {
                    write!(
                        f,
                        "Error at {}: {}, {}: {} -> {}",
                        metadata.line_no, metadata.start, metadata.line_no, metadata.end, string
                    )
                }
            }
        }
    }
}

impl std::fmt::Display for ErrorCodeList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (index, err) in self.0.iter().enumerate() {
            writeln!(f, "({}):\t {}", index, err)?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn t() {
    //     assert!(6 == 6)
    // }
}

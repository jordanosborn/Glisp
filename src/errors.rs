use crate::tokens;
#[derive(Debug, Clone)]
pub enum ErrorCode<'a> {
    General(String),
    SyntaxError(String, tokens::Token, tokens::MetaData<'a>),
}

pub struct ErrorCodeList<'a>(Vec<ErrorCode<'a>>);

impl<'a> ErrorCodeList<'a> {
    pub fn new() -> ErrorCodeList<'a> {
        ErrorCodeList(Vec::new())
    }

    pub fn from(v: Vec<ErrorCode>) -> ErrorCodeList {
        ErrorCodeList(v)
    }

    pub fn push(&mut self, err: ErrorCode<'a>) {
        self.0.push(err);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> std::fmt::Display for ErrorCode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorCode::General(s) => write!(f, "{}", s),
            ErrorCode::SyntaxError(string, _, metadata) => {
                if let Some(end_line) = metadata.line_no_end {
                    write!(
                        f,
                        "{} at {}: {}, {}: {} ->\n\t\t{}",
                        metadata.filename,
                        metadata.line_no,
                        metadata.start,
                        end_line,
                        metadata.end,
                        string
                    )
                } else {
                    write!(
                        f,
                        "{} at {}: {}, {}: {} ->\n\t{}",
                        metadata.filename,
                        metadata.line_no,
                        metadata.start,
                        metadata.line_no,
                        metadata.end,
                        string
                    )
                }
            }
        }
    }
}

impl<'a> std::fmt::Display for ErrorCodeList<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Errors")?;
        for (index, err) in self.0.iter().enumerate() {
            writeln!(f, "({}): {}", index, err)?;
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

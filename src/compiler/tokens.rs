#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Type {
    RATIONAL,
    INTEGER,
    FLOAT,
    DECIMAL,
    STRING,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rational {
    pub numerator: i64,
    pub denominator: i64,
}

impl Rational {}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenSquareBrace,
    CloseSquareBrace,
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenAngularBrace,
    CloseAngularBrace,
    OpenQuote,
    CloseQuote,
    OpenMultilineComment,
    CloseMultilineComment,
    Literal(char),
    Indentation((u64, u64)),
    Identifier(String),
    Type(Type),
    Keyword(String),
    Comment(String),
    MultilineComment(String),
    String(String),
    Integer(String),
    Float(String),
    Rational(Rational),
    Character(String),
    Other(String),
    Newline,
}

#[derive(Debug, Copy, Clone)]
pub struct MetaData<'a> {
    pub line_no: usize,
    pub start: usize,
    pub end: usize,
    pub line_no_end: Option<usize>,
    pub filename: &'a str,
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn t() {
    //     assert!(6 == 6)
    // }
}

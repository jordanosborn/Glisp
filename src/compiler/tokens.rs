#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Type {
    RATIONAL,
    INTEGER,
    FLOAT,
    STRING,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Keyword {
    FUNC,
    PRINT,
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
    Ident,
    Type(Type),
    Builtin(Keyword),
    String(String),
    Comment(String),
    Rational(Rational),
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

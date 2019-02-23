#[derive(Debug)]
pub enum Type {
    RATIONAL,
    INTEGER,
    FLOAT,
    STRING,
}

#[derive(Debug)]
pub enum Keyword {
    FUNC,
    PRINT,
}

#[derive(Debug)]
pub struct Rational {
    pub numerator: i64,
    pub denominator: i64,
}

impl Rational {}

#[derive(Debug)]
pub enum Token {
    OpenBrace,
    CloseBrace,
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

#[derive(Debug)]
pub struct MetaData {
    pub line_no: usize,
    pub start: usize,
    pub end: usize,
    pub line_no_end: Option<usize>,
}

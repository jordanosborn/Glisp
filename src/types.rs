#[derive(Debug)]
pub enum Type {
    RATIONAL,
    INTEGER,
    FLOAT,
    STRING,
}

#[derive(Debug)]
pub enum Keyword {
    DEFUN,
    PRINT,
}

#[derive(Debug)]
pub struct Rational {
    pub numerator: i64,
    pub denominator: i64,
}

impl Rational {}

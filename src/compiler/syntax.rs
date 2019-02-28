use regex::Regex;

pub const fn comment() -> char {
    '#'
}

pub fn is_literal(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[+-:|/%^&*$;,.'!@?=]$").unwrap();
    }
    RE.is_match(text)
}

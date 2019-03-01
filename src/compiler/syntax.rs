use regex::Regex;

pub const fn comment() -> char {
    '#'
}

pub fn is_character(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^.$").unwrap();
    }
    RE.is_match(text)
}

pub fn is_literal(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[-+:|/%^&*'$;,.!@?=]$").unwrap();
    }
    RE.is_match(text)
}

pub fn is_integer(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("").unwrap();
    }
    RE.is_match(text)
}

pub fn is_non_ident_character(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[^ \t\"\\(\\)\\[\\]\\(\\)<>\\{\\}]$").unwrap();
    }
    RE.is_match(text) && is_literal(text)
}

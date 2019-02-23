use crate::tokens;
#[derive(Debug)]
pub enum ErrorCode {
    General(String),
    SyntaxError(String, tokens::Token, tokens::MetaData),
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn t() {
    //     assert!(6 == 6)
    // }
}

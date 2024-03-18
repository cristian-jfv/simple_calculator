use std::fmt;

#[derive(Debug)]
pub enum TokenKind {
    Plus,
    Minus,
    Times,
    Divide,
    OpenParenthesis,
    CloseParenthesis,
    Number,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Times => write!(f, "*"),
            TokenKind::Divide => write!(f, "/"),
            TokenKind::OpenParenthesis => write!(f, "("),
            TokenKind::CloseParenthesis => write!(f, ")"),
            TokenKind::Number => write!(f, "5"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: f64,
}

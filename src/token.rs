use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    //Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    //One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String_,
    Number,

    //Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String_(String),
    Number(f64),
    Bool(bool),
    Nil
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    type_: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, literal: Option<Literal>, line: usize) -> Self {
        Self {
            type_,
            lexeme: lexeme.to_owned(),
            literal,
            line
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.type_, self.lexeme, self.literal)
    }
}
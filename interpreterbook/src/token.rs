use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq)]
pub enum Token {
    Assign,
    Asterisk,
    Bang,
    Comma,
    Else,
    Eof,
    Eq,
    False,
    Function,
    Gt,
    Ident(String),
    If,
    Illegal(char),
    Int(String),
    Lbrace,
    Let,
    Lparen,
    Lt,
    Minus,
    NotEq,
    Plus,
    Rbrace,
    Return,
    Rparen,
    Semicolon,
    Slash,
    True,
}

pub const KEYWORDS: [(&str, Token); 7] = [
    ("else", Token::Else),
    ("false", Token::False),
    ("fn", Token::Function),
    ("if", Token::If),
    ("let", Token::Let),
    ("return", Token::Return),
    ("true", Token::True),
];

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Token::Assign => write!(f, "="),
            Token::Asterisk => write!(f, "*"),
            Token::Bang => write!(f, "!"),
            Token::Comma => write!(f, ","),
            Token::Else => write!(f, "ELSE"),
            Token::Eof => write!(f, "EOF"),
            Token::Eq => write!(f, "=="),
            Token::False => write!(f, "FALSE"),
            Token::Function => write!(f, "FUNCTION"),
            Token::Gt => write!(f, ">"),
            Token::Ident(value) => write!(f, "IDENT {}", value),
            Token::If => write!(f, "IF"),
            Token::Illegal(value) => write!(f, "ILLEGAL {}", value),
            Token::Int(value) => write!(f, "INT {}", value),
            Token::Lbrace => write!(f, "{{"),
            Token::Let => write!(f, "LET"),
            Token::Lparen => write!(f, "("),
            Token::Lt => write!(f, "<"),
            Token::Minus => write!(f, "-"),
            Token::NotEq => write!(f, "!="),
            Token::Plus => write!(f, "+"),
            Token::Rbrace => write!(f, "}}"),
            Token::Return => write!(f, "RETURN"),
            Token::Rparen => write!(f, ")"),
            Token::Semicolon => write!(f, ";"),
            Token::Slash => write!(f, "/"),
            Token::True => write!(f, "TRUE"),
        }
    }
}

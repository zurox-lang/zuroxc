use crate::utils;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    DataType(usize, usize, String),
    Identifier(usize, usize, String),
    Separator(usize, usize, String),
    Operator(usize, usize, String),
    Keyword(usize, usize, String),

    IntLiteral(usize, usize, String),
    FloatLiteral(usize, usize, String),
    StringLiteral(usize, usize, String),
    CharLiteral(usize, usize, String),

    /// Error token
    Error(utils::LexerError),

    /// End of the file
    Eof, 
}

use std::fmt;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::DataType(line, col, ref value) => write!(f, "DataType(line: {}, col: {}, value: {})", line, col, value),
            Token::Identifier(line, col, ref value) => write!(f, "Identifier(line: {}, col: {}, value: {})", line, col, value),
            Token::Separator(line, col, ref value) => write!(f, "Separator(line: {}, col: {}, value: {})", line, col, value),
            Token::Operator(line, col, ref value) => write!(f, "Operator(line: {}, col: {}, value: {})", line, col, value),
            Token::Keyword(line, col, ref value) => write!(f, "Keyword(line: {}, col: {}, value: {})", line, col, value),
            Token::IntLiteral(line, col, ref value) => write!(f, "IntLiteral(line: {}, col: {}, value: {})", line, col, value),
            Token::FloatLiteral(line, col, ref value) => write!(f, "FloatLiteral(line: {}, col: {}, value: {})", line, col, value),
            Token::StringLiteral(line, col, ref value) => write!(f, "StringLiteral(line: {}, col: {}, value: {})", line, col, value),
            Token::CharLiteral(line, col, ref value) => write!(f, "CharLiteral(line: {}, col: {}, value: {})", line, col, value),
            Token::Error(ref err) => write!(f, "Error: {}", err),
            Token::Eof => write!(f, "End of File"),
        }
    }
}
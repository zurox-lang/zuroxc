use crate::utils;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

impl Token {
    pub fn get_line(&self) -> usize {
        let line: usize;
        match &self {
            Self::DataType(x, _, _)
            | Self::Identifier(x, _, _)
            | Self::Separator(x, _, _)
            | Self::Operator(x, _, _)
            | Self::Keyword(x, _, _)
            | Self::IntLiteral(x, _, _)
            | Self::CharLiteral(x, _, _)
            | Self::FloatLiteral(x, _, _)
            | Self::StringLiteral(x, _, _) => {
                line = *x;
            }
            _ => return 0,
        }
        line
    }

    pub fn get_col(&self) -> usize {
        let col: usize;
        match &self {
            Self::DataType(_, x, _)
            | Self::Identifier(_, x, _)
            | Self::Separator(_, x, _)
            | Self::Operator(_, x, _)
            | Self::Keyword(_, x, _)
            | Self::IntLiteral(_, x, _)
            | Self::CharLiteral(_, x, _)
            | Self::FloatLiteral(_, x, _)
            | Self::StringLiteral(_, x, _) => {
                col = *x;
            }
            _ => return 0,
        }
        col
    }

    pub fn get_lexeme(&self) -> &str {
        let string: &str;
        match &self {
            Self::DataType(_, _, x)
            | Self::Identifier(_, _, x)
            | Self::Separator(_, _, x)
            | Self::Operator(_, _, x)
            | Self::Keyword(_, _, x)
            | Self::IntLiteral(_, _, x)
            | Self::CharLiteral(_, _, x)
            | Self::FloatLiteral(_, _, x)
            | Self::StringLiteral(_, _, x) => {
                string = &x;
            }
            _ => return "",
        }
        string
    }
}

use std::fmt;
/* To be used for debugging only. */
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::DataType(line, col, ref value) => write!(
                f,
                "DataType(line: {}, col: {}, value: {})",
                line, col, value
            ),
            Token::Identifier(line, col, ref value) => write!(
                f,
                "Identifier(line: {}, col: {}, value: {})",
                line, col, value
            ),
            Token::Separator(line, col, ref value) => write!(
                f,
                "Separator(line: {}, col: {}, value: {})",
                line, col, value
            ),
            Token::Operator(line, col, ref value) => write!(
                f,
                "Operator(line: {}, col: {}, value: {})",
                line, col, value
            ),
            Token::Keyword(line, col, ref value) => {
                write!(f, "Keyword(line: {}, col: {}, value: {})", line, col, value)
            }
            Token::IntLiteral(line, col, ref value) => write!(
                f,
                "IntLiteral(line: {}, col: {}, value: {})",
                line, col, value
            ),
            Token::FloatLiteral(line, col, ref value) => write!(
                f,
                "FloatLiteral(line: {}, col: {}, value: {})",
                line, col, value
            ),
            Token::StringLiteral(line, col, ref value) => write!(
                f,
                "StringLiteral(line: {}, col: {}, value: {})",
                line, col, value
            ),
            Token::CharLiteral(line, col, ref value) => write!(
                f,
                "CharLiteral(line: {}, col: {}, value: {})",
                line, col, value
            ),
            Token::Error(ref err) => write!(f, "Error: {}", err),
            Token::Eof => write!(f, "End of File"),
        }
    }
}

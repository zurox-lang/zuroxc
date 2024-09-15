use crate::utils;
use serde::{Deserialize, Serialize};

/// Represents a token in the lexical analysis phase. 
/// Each token stores its line, column, and lexeme value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Token {
    /// Data type token: (line, column, value)
    DataType(usize, usize, String),
    /// Identifier token: (line, column, value)
    Identifier(usize, usize, String),
    /// Separator token: (line, column, value)
    Separator(usize, usize, String),
    /// Operator token: (line, column, value)
    Operator(usize, usize, String),
    /// Keyword token: (line, column, value)
    Keyword(usize, usize, String),

    /// Integer literal token: (line, column, value)
    IntLiteral(usize, usize, String),
    /// Floating-point literal token: (line, column, value)
    FloatLiteral(usize, usize, String),
    /// String literal token: (line, column, value)
    StringLiteral(usize, usize, String),
    /// Character literal token: (line, column, value)
    CharLiteral(usize, usize, String),

    /// Error token, representing an invalid or unrecognized token
    Error(utils::LexerError),

    /// End of the file (EOF) token, signifies the end of input
    Eof,
}

impl Token {
    /// Returns the line number where the token occurs.
    pub fn get_line(&self) -> usize {
        match &self {
            Self::DataType(line, _, _)
            | Self::Identifier(line, _, _)
            | Self::Separator(line, _, _)
            | Self::Operator(line, _, _)
            | Self::Keyword(line, _, _)
            | Self::IntLiteral(line, _, _)
            | Self::CharLiteral(line, _, _)
            | Self::FloatLiteral(line, _, _)
            | Self::StringLiteral(line, _, _) => *line,
            _ => 0, // Return 0 if token type does not contain line information
        }
    }

    /// Returns the column number where the token occurs.
    pub fn get_col(&self) -> usize {
        match &self {
            Self::DataType(_, col, _)
            | Self::Identifier(_, col, _)
            | Self::Separator(_, col, _)
            | Self::Operator(_, col, _)
            | Self::Keyword(_, col, _)
            | Self::IntLiteral(_, col, _)
            | Self::CharLiteral(_, col, _)
            | Self::FloatLiteral(_, col, _)
            | Self::StringLiteral(_, col, _) => *col,
            _ => 0, // Return 0 if token type does not contain column information
        }
    }

    /// Returns the lexeme (value) of the token as a string slice.
    pub fn get_lexeme(&self) -> &str {
        match &self {
            Self::DataType(_, _, lexeme)
            | Self::Identifier(_, _, lexeme)
            | Self::Separator(_, _, lexeme)
            | Self::Operator(_, _, lexeme)
            | Self::Keyword(_, _, lexeme)
            | Self::IntLiteral(_, _, lexeme)
            | Self::CharLiteral(_, _, lexeme)
            | Self::FloatLiteral(_, _, lexeme)
            | Self::StringLiteral(_, _, lexeme) => lexeme,
            _ => "", // Return empty string if token type does not contain a lexeme
        }
    }
}

use std::fmt;

/// Implements the `Display` trait for `Token`, providing a human-readable 
/// string representation of each token. This is especially useful for debugging.
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
            Token::Keyword(line, col, ref value) => write!(
                f,
                "Keyword(line: {}, col: {}, value: {})",
                line, col, value
            ),
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

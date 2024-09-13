use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LexerError {
    UnexpectedEOF(usize, usize, String),
    InvalidBinary(usize, usize, String),
    InvalidOctal(usize, usize, String),
    InvalidDecimal(usize, usize, String),
    InvalidHexaDecimal(usize, usize, String),
    InvalidFloat(usize, usize, String),
    UnclosedString(usize, usize, String),
    UnclosedCharacter(usize, usize, String),
    UnclosedComment(usize, usize, String),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedEOF(line, col, value) => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Unexpected EOF at".red().bold(),
                    format!("line {}, col {}", line, col).yellow(),
                    "->".cyan(),
                    value.blue()
                )
            }
            LexerError::InvalidBinary(line, col, value) => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Invalid binary number at".red().bold(),
                    format!("line {}, col {}", line, col).yellow(),
                    "->".cyan(),
                    value.blue()
                )
            }
            LexerError::InvalidOctal(line, col, value) => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Invalid octal number at".red().bold(),
                    format!("line {}, col {}", line, col).yellow(),
                    "->".cyan(),
                    value.blue()
                )
            }
            LexerError::InvalidDecimal(line, col, value) => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Invalid decimal number at".red().bold(),
                    format!("line {}, col {}", line, col).yellow(),
                    "->".cyan(),
                    value.blue()
                )
            }
            LexerError::InvalidHexaDecimal(line, col, value) => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Invalid hexadecimal number at".red().bold(),
                    format!("line {}, col {}", line, col).yellow(),
                    "->".cyan(),
                    value.blue()
                )
            }
            LexerError::InvalidFloat(line, col, value) => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Invalid float number at".red().bold(),
                    format!("line {}, col {}", line, col).yellow(),
                    "->".cyan(),
                    value.blue()
                )
            }
            LexerError::UnclosedString(line, col, value) => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Unclosed string literal at".red().bold(),
                    format!("line {}, col {}", line, col).yellow(),
                    "->".cyan(),
                    value.blue()
                )
            }
            LexerError::UnclosedCharacter(line, col, value) => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Unclosed character at".red().bold(),
                    format!("line {}, col {}", line, col).yellow(),
                    "->".cyan(),
                    value.blue()
                )
            }
            LexerError::UnclosedComment(line, col, value) => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Unclosed comment at".red().bold(),
                    format!("line {}, col {}", line, col).yellow(),
                    "->".cyan(),
                    value.blue()
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ParserError {
    UnexpectedToken(usize, usize, String),
    MissingToken(usize, usize, String),
    InvalidSyntax(usize, usize, String),
    UnexpectedEOF(usize, usize, String),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken(line, col, token) => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Unexpected token at".red().bold(),
                    format!("line {}, col {}", line, col).yellow(),
                    "->".cyan(),
                    token.blue()
                )
            }
            ParserError::MissingToken(line, col, expected) => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Missing expected token at".red().bold(),
                    format!("line {}, col {}", line, col).yellow(),
                    "->".cyan(),
                    expected.blue()
                )
            }
            ParserError::InvalidSyntax(line, col, message) => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Invalid syntax at".red().bold(),
                    format!("line {}, col {}", line, col).yellow(),
                    "->".cyan(),
                    message.blue()
                )
            }
            ParserError::UnexpectedEOF(line, col, message) => {
                write!(
                    f,
                    "{} {} {} {}",
                    "Unexpected EOF while parsing at".red().bold(),
                    format!("line {}, col {}", line, col).yellow(),
                    "->".cyan(),
                    message.blue()
                )
            }
        }
    }
}

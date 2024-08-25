use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum LexerError {
    UnexpectedEOF(usize, usize, String),
    //UnexpectedCharacter(usize, usize, String),
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
                write!(f, "Unexpected EOF at line {}, col {}: {}", line, col, value)
            }
            /* 
            LexerError::UnexpectedCharacter(line, col, value) => {
                write!(
                    f,
                    "Unexpected character at line {}, col {}: {}",
                    line, col, value
                )
            }*/
            LexerError::InvalidBinary(line, col, value) => {
                write!(
                    f,
                    "Invalid binary number at line {}, col {}: {}",
                    line, col, value
                )
            }
            LexerError::InvalidOctal(line, col, value) => {
                write!(
                    f,
                    "Invalid octal number at line {}, col {}: {}",
                    line, col, value
                )
            }
            LexerError::InvalidDecimal(line, col, value) => {
                write!(
                    f,
                    "Invalid decimal number at line {}, col {}: {}",
                    line, col, value
                )
            }
            LexerError::InvalidHexaDecimal(line, col, value) => {
                write!(
                    f,
                    "Invalid hexadecimal number at line {}, col {}: {}",
                    line, col, value
                )
            }
            LexerError::InvalidFloat(line, col, value) => {
                write!(
                    f,
                    "Invalid float number at line {}, col {}: {}",
                    line, col, value
                )
            }
            LexerError::UnclosedString(line, col, value) => {
                write!(f, "Unclosed string at line {}, col {}: {}", line, col, value)
            }
            LexerError::UnclosedCharacter(line, col, value) => {
                write!(
                    f,
                    "Unclosed character at line {}, col {}: {}",
                    line, col, value
                )
            }
            LexerError::UnclosedComment(line, col, value) => {
                write!(f, "Unclosed comment at line {}, col {}: {}", line, col, value)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParserError {

}

#[derive(Clone, Debug, PartialEq)]
pub enum SemanticError {}

#[derive(Clone, Debug, PartialEq)]
pub enum CodeGenError {}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    LexErr(LexerError),
    ParseErr(ParserError),
    SemErr(SemanticError),
    CodeGenErr(CodeGenError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::LexErr(le) => {
                write!(f, "Encountered the following error during lexical analysis: {}", le)
            }
            Error::ParseErr(pe) => {
                //write!(f, "Encountered the following error during parsing: {}", pe)
                write!(f, "Encountered the following error during parsing: ")
            }
            Error::SemErr(se) => {
                //write!(f, "Encountered the following error during semantic analysis: {}", se)
                write!(f, "Encountered the following error during semantic analysis: ")
            }
            Error::CodeGenErr(ce) => {
                //write!(f, "Encountered the following error code generation: {}", ce)
                write!(f, "Encountered the following error during code generation: ")
            }
        }
    }
}
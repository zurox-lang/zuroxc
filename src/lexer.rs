use crate::token::{self, Token};
use crate::utils::{self, LexerError};

pub struct Lexer<'a> {
    line: usize,
    col: usize,
    input: &'a str,
    tokens: Vec<Token>,
    has_error: bool,
}

pub const DATA_TYPES: [&str; 16] = [
    "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64", "u128", "i128", "f80",
    "f128", "char", "bool",
];

pub const MAX_DATA_TYPE_LEN: usize = {
    let mut max_len = 0;
    let mut i = 0;
    while i < DATA_TYPES.len() {
        let len = DATA_TYPES[i].len();
        if len > max_len {
            max_len = len;
        }
        i += 1;
    }
    max_len
};

pub const KEYWORDS: [&str; 27] = [
    "asm", "if", "elif", "else", "loop", "fn", "ret", "true", "false", "ref", "deref", "impl",
    "struct", "async", "enum", "void", "volatile", "null", "import", "llvm", "break", "continue",
    "match", "def", "pub", "const", "default",
];

pub const MAX_KEYWORDS_LEN: usize = {
    let mut max_len = 0;
    let mut i = 0;
    while i < KEYWORDS.len() {
        let len = KEYWORDS[i].len();
        if len > max_len {
            max_len = len;
        }
        i += 1;
    }
    max_len
};

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            line: 1,
            col: 0,
            input,
            tokens: Vec::new(),
            has_error: false,
        }
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }

    fn find_dt(&self, x: &str) -> Option<usize> {
        DATA_TYPES.iter().position(|&s| s == x).map(|pos| pos)
    }

    fn find_keyword(&self, x: &str) -> Option<usize> {
        KEYWORDS.iter().position(|&s| s == x).map(|pos| pos)
    }

    fn current(&self) -> Option<char> {
        self.input[self.col..].chars().next()
    }

    fn peek(&self) -> Option<char> {
        self.input[self.col + 1..].chars().next()
    }

    fn advance(&mut self) {
        if let Some(c) = self.current() {
            self.col += c.len_utf8();
        }
    }

    fn eof(&self) -> bool {
        self.col >= self.input.len()
    }

    pub fn lex(&mut self) -> Vec<token::Token> {
        while self.col < self.input.len() {
            let c = self.current().unwrap_or('\0');
            if c.is_numeric() {
                self.number();
            } else if self.is_separator(c) {
                self.tokens
                    .push(Token::Separator(self.line, self.col, c.to_string()));
                self.advance();
            } else if self.is_operator(c) {
                self.handle_operator();
            } else if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                }
                self.advance();
            } else if c == '"' {
                self.handle_string_literal();
            } else if c == '\'' {
                self.handle_char_literal();
            } else {
                self.keyword_or_datatype_or_identifier();
            }
        }
        self.tokens.push(Token::Eof);
        self.tokens.clone()
    }

    fn keyword_or_datatype_or_identifier(&mut self) {
        let mut str = String::new();
        str.reserve(8);

        while let Some(c) = self.current() {
            if self.is_operator(c) || self.is_separator(c) || c.is_whitespace() {
                break;
            }
            str.push(c);
            self.advance();
        }

        let token = if self.find_dt(&str).is_some() {
            Token::DataType(self.line, self.col - str.len(), str)
        } else if self.find_keyword(&str).is_some() {
            Token::Keyword(self.line, self.col - str.len(), str)
        } else {
            Token::Identifier(self.line, self.col - str.len(), str)
        };

        self.tokens.push(token);
    }

    fn number(&mut self) {
        let mut str = String::with_capacity(8);

        if let Some(c) = self.current() {
            if c == '0' {
                self.advance();
                if let Some(next_c) = self.current() {
                    match next_c {
                        'x' | 'X' => {
                            str.push('0');
                            str.push(next_c);
                            self.advance();
                            while let Some(c) = self.current() {
                                if c.is_digit(16) {
                                    str.push(c);
                                    self.advance();
                                } else {
                                    break;
                                }
                            }

                            if let Err(_) = u64::from_str_radix(&str[2..], 16) {
                                self.has_error = true;
                                self.tokens.push(Token::Error(
                                    utils::LexerError::InvalidHexaDecimal(
                                        self.line,
                                        self.col - str.len(),
                                        str,
                                    ),
                                ));
                            } else {
                                self.tokens.push(Token::IntLiteral(
                                    self.line,
                                    self.col - str.len(),
                                    str,
                                ));
                            }
                            return;
                        }
                        'o' | 'O' => {
                            str.push('0');
                            str.push(next_c);
                            self.advance();
                            while let Some(c) = self.current() {
                                if c.is_digit(8) {
                                    str.push(c);
                                    self.advance();
                                } else {
                                    break;
                                }
                            }

                            if let Err(_) = u64::from_str_radix(&str[2..], 8) {
                                self.has_error = true;
                                self.tokens
                                    .push(Token::Error(utils::LexerError::InvalidOctal(
                                        self.line,
                                        self.col - str.len(),
                                        str,
                                    )));
                            } else {
                                self.tokens.push(Token::IntLiteral(
                                    self.line,
                                    self.col - str.len(),
                                    str,
                                ));
                            }
                            return;
                        }
                        'b' | 'B' => {
                            str.push('0');
                            str.push(next_c);
                            self.advance();
                            while let Some(c) = self.current() {
                                if c == '0' || c == '1' {
                                    str.push(c);
                                    self.advance();
                                } else {
                                    break;
                                }
                            }

                            if let Err(_) = u64::from_str_radix(&str[2..], 2) {
                                self.has_error = true;
                                self.tokens
                                    .push(Token::Error(utils::LexerError::InvalidBinary(
                                        self.line,
                                        self.col - str.len(),
                                        str,
                                    )));
                            } else {
                                self.tokens.push(Token::IntLiteral(
                                    self.line,
                                    self.col - str.len(),
                                    str,
                                ));
                            }
                            return;
                        }
                        _ => {}
                    }
                }
            }

            // Handle decimal or float
            while let Some(c) = self.current() {
                if c.is_numeric() {
                    str.push(c);
                    self.advance();
                } else {
                    break;
                }
            }

            let mut is_float = false;

            if let Some(c) = self.current() {
                if c == '.' {
                    is_float = true;
                    str.push(c);
                    self.advance();
                    while let Some(c) = self.current() {
                        if c.is_numeric() {
                            str.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                if let Some(next_c) = self.current() {
                    if next_c.to_ascii_lowercase() == 'e' {
                        is_float = true;
                        str.push(next_c);
                        self.advance();
                        if let Some(c) = self.current() {
                            if c == '+' || c == '-' {
                                str.push(c);
                                self.advance();
                            }
                        }
                        while let Some(c) = self.current() {
                            if c.is_numeric() {
                                str.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                }
            }

            if is_float {
                if let Err(_) = str.parse::<f64>() {
                    self.has_error = true;
                    self.tokens
                        .push(Token::Error(utils::LexerError::InvalidFloat(
                            self.line,
                            self.col - str.len(),
                            str,
                        )));
                } else {
                    self.tokens
                        .push(Token::FloatLiteral(self.line, self.col - str.len(), str));
                }
            } else {
                if let Err(_) = str.parse::<u64>() {
                    self.has_error = true;
                    self.tokens
                        .push(Token::Error(utils::LexerError::InvalidDecimal(
                            self.line,
                            self.col - str.len(),
                            str,
                        )));
                } else {
                    self.tokens
                        .push(Token::IntLiteral(self.line, self.col - str.len(), str));
                }
            }
        }
    }

    fn is_separator(&self, c: char) -> bool {
        matches!(c, ';' | ',' | '{' | '}' | '[' | ']' | '(' | ')')
    }

    fn is_operator(&self, c: char) -> bool {
        matches!(
            c,
            '>' | '<' | '=' | '!' | '^' | '|' | '&' | '~' | '+' | '-' | '*' | '/' | '%' | '.'
        )
    }

    fn handle_operator(&mut self) {
        let mut op = String::with_capacity(1);

        if let Some(c) = self.current() {
            if c == '/' {
                if let Some(next_c) = self.peek() {
                    if next_c == '/' || next_c == '*' {
                        self.handle_comment();
                        return;
                    }
                }
            }

            op.push(c);
            self.tokens.push(Token::Operator(self.line, self.col, op));
            self.advance();
        }
    }

    fn handle_comment(&mut self) {
        let mut comment = String::new();
        comment.reserve(128);

        if let Some(c) = self.current() {
            if c == '/' {
                comment.push(c);
                self.advance();
                if let Some(next_c) = self.current() {
                    if next_c == '/' {
                        comment.push(next_c);
                        self.advance();
                        while let Some(c) = self.current() {
                            if c == '\n' {
                                break;
                            }
                            comment.push(c);
                            self.advance();
                        }
                        return;
                    } else if next_c == '*' {
                        comment.push(next_c);
                        self.advance();
                        while let Some(c) = self.current() {
                            if c == '*' {
                                if let Some(next_c) = self.peek() {
                                    if next_c == '/' {
                                        comment.push(c);
                                        comment.push(next_c);
                                        self.advance();
                                        self.advance();
                                        return;
                                    }
                                }
                            }
                            comment.push(c);
                            self.advance();
                        }
                        self.has_error = true;
                        self.tokens
                            .push(Token::Error(LexerError::UnterminatedComment(
                                self.line, self.col, comment,
                            )));
                    }
                }
            }
        }
    }

    fn handle_string_literal(&mut self) {
        let mut literal = String::with_capacity(128);

        if let Some(c) = self.current() {
            literal.push(c);
            self.advance();

            while let Some(c) = self.current() {
                if c == '"' {
                    let x = literal
                        .chars()
                        .last()
                        .expect("Unable to fetch last character from memory.");
                    literal.push(c);
                    self.advance();
                    if x != '\\' {
                        break;
                    }
                }
                if self.eof() {
                    self.has_error = true;
                    self.tokens.push(Token::Error(LexerError::UnexpectedEOF(
                        self.line,
                        self.col - literal.len(),
                        literal,
                    )));
                    return;
                }
                literal.push(c);
                self.advance();
            }

            if literal.chars().last().expect("Unable to fetch character.") != '"' {
                self.has_error = true;
                self.tokens
                    .push(Token::Error(LexerError::UnterminatedStringLiteral(
                        self.line,
                        self.col - literal.len(),
                        literal,
                    )));
                return;
            }

            self.tokens.push(Token::StringLiteral(
                self.line,
                self.col - literal.len(),
                literal,
            ));
        }
    }

    fn handle_char_literal(&mut self) {
        let mut literal = String::with_capacity(4);

        if let Some(c) = self.current() {
            literal.push(c);
            self.advance();

            while let Some(c) = self.current() {
                if c == '\'' {
                    let x = literal
                        .chars()
                        .last()
                        .expect("Unable to fetch last character from memory.");
                    literal.push(c);
                    self.advance();
                    println!("X: {}", x);
                    if x != '\\' {
                        break;
                    }
                }
                if self.eof() {
                    self.has_error = true;
                    self.tokens.push(Token::Error(LexerError::UnexpectedEOF(
                        self.line,
                        self.col - literal.len(),
                        literal,
                    )));
                    return;
                }
                literal.push(c);
                self.advance();
            }

            if literal.chars().last().expect("Unable to fetch character.") != '\'' {
                self.has_error = true;
                self.tokens
                    .push(Token::Error(LexerError::UnterminatedCharacterLiteral(
                        self.line,
                        self.col - literal.len(),
                        literal,
                    )));
                return;
            }

            self.tokens.push(Token::CharLiteral(
                self.line,
                self.col - literal.len(),
                literal,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{Distribution, Uniform};
    use rand::Rng;
    use std::time::Instant;

    fn generate_random_number() -> String {
        let mut rng = rand::thread_rng();
        let choice = rng.gen_range(0..2);

        if choice == 0 {
            rng.gen_range(0..1_000_000).to_string()
        } else {
            let max_float = 1.7976931348623157e+308;
            let random_float = rng.gen_range(0.0..max_float);
            format!("{:.10}", random_float)
        }
    }

    fn generate_random_identifier() -> String {
        let mut rng = rand::thread_rng();
        let len_dist = Uniform::from(1..1000);
        let len = len_dist.sample(&mut rng);

        let char_dist = Uniform::from(b'a'..=b'z');
        (0..len)
            .map(|_| char_dist.sample(&mut rng) as char)
            .collect()
    }

    #[test]
    fn test_keywords() {
        let mut input = String::new();
        for string in KEYWORDS {
            input.push_str(string);
            input.push_str(" ");
        }
        let mut lexer = Lexer::new(&input);

        let tokens = lexer.lex();
        assert_eq!(tokens.len(), KEYWORDS.len() + 1);

        for tok in tokens.iter().take(KEYWORDS.len()) {
            match tok {
                Token::Keyword(_, _, word) => {
                    if !KEYWORDS
                        .iter()
                        .position(|&s| s == word)
                        .map(|pos| pos)
                        .is_some()
                    {
                        panic!("Expected a keyword, got {:?}", tok);
                    }
                }
                _ => panic!("Expected a keyword, got {:?}", tok),
            }
        }

        assert_eq!(tokens[KEYWORDS.len()], Token::Eof);
    }

    #[test]
    fn test_data_types() {
        let mut input = String::new();
        for string in DATA_TYPES {
            input.push_str(string);
            input.push_str(" ");
        }
        let mut lexer = Lexer::new(&input);
        let tokens = lexer.lex();

        assert_eq!(tokens.len(), DATA_TYPES.len() + 1); // Ensure correct number of tokens
        for token in tokens.iter().take(DATA_TYPES.len()) {
            match token {
                Token::DataType(_, _, _) => {}
                _ => panic!("Expected a data type, got {:?}", token),
            }
        }
        assert_eq!(tokens[DATA_TYPES.len()], Token::Eof);
    }

    #[test]
    fn number_method_test() {
        let mut lexer = Lexer::new("0xAE 0x7E 0xe7 0b01 0o100 23.000535 1.05e+27 -100 100");
        let tokens = lexer.lex();
        for tok in tokens.iter() {
            match tok {
                Token::FloatLiteral(_, _, _)
                | Token::IntLiteral(_, _, _)
                | Token::Identifier(_, _, _) => {}
                _ => {}
            }
        }
        assert_eq!(tokens.len(), 11);

        for tok in tokens.iter().take(9) {
            match tok {
                Token::IntLiteral(_, _, _)
                | Token::FloatLiteral(_, _, _)
                | Token::Operator(_, _, _) => {}
                _ => panic!("Expected an integer or float, got {:?}", tok),
            }
        }

        assert_eq!(tokens[10], Token::Eof);
    }

    #[test]
    fn benchmark_number() {
        let mut large_input = String::new();
        for _ in 0..60_000 {
            large_input.push_str(&generate_random_number());
            large_input.push(' ');
        }

        let mut lexer = Lexer::new(&large_input);

        let start_time = Instant::now();
        let tokens = lexer.lex();
        let duration = start_time.elapsed();

        for i in 0..tokens.len() - 1 {
            match tokens[i] {
                Token::FloatLiteral(_, _, _) | Token::IntLiteral(_, _, _) => {}
                _ => panic!("Expected a float or integer literal, found {}", &tokens[i]),
            }
        }

        assert_eq!(tokens.last().unwrap(), &Token::Eof);

        let input_size_bytes = large_input.len() as f64;
        let input_size_mb = input_size_bytes / (1024.0 * 1024.0);

        println!(
            "Lexer took {} ms to lex random numbers.",
            duration.as_millis()
        );
        println!(
            "Avg. characters/ms: {}",
            input_size_bytes / duration.as_millis() as f64
        );
        println!("Input size: {} MB", input_size_mb);

        let duration_seconds = duration.as_secs_f64();
        println!("Throughput: {} MB/s", input_size_mb / duration_seconds); // MB/s
    }

    #[test]
    fn benchmark_identifier() {
        let mut large_input = String::new();
        for _ in 0..1_000_000 {
            large_input.push_str(&generate_random_identifier());
            large_input.push(' ');
        }

        let mut lexer = Lexer::new(&large_input);

        let start_time = Instant::now();
        let tokens = lexer.lex();
        let duration = start_time.elapsed();

        for i in 0..tokens.len() - 1 {
            match tokens.get(i).expect("Expected a token.") {
                Token::Identifier(_, _, id) => {
                    if KEYWORDS
                        .iter()
                        .position(|&s| s == id)
                        .map(|pos| pos)
                        .is_some()
                        || DATA_TYPES
                            .iter()
                            .position(|&s| s == id)
                            .map(|pos| pos)
                            .is_some()
                    {
                        panic!("Expected a identifier, found keyword or data type.")
                    }
                    assert!(!id.is_empty());
                }
                _ => {}
            }
        }

        assert_eq!(tokens.last().unwrap(), &Token::Eof); // Ensure the last token is EOF

        let input_size_bytes = large_input.len() as f64;
        let input_size_mb = input_size_bytes / (1024.0 * 1024.0); // Convert bytes to MB

        println!(
            "Lexer took {} ms to lex random identifiers.",
            duration.as_millis()
        );
        println!(
            "Avg. characters/ms: {}",
            input_size_bytes / duration.as_millis() as f64
        );
        println!("Input size: {} MB", input_size_mb);

        let duration_seconds = duration.as_secs_f64();
        println!("Throughput: {} MB/s", input_size_mb / duration_seconds); // MB/s
    }
}

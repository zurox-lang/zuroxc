use crate::{ast::*, token::Token, utils::ParserError};

pub struct Parser {
    tokens: Vec<Token>, // Data from the lexer is to be moved here.
    index: usize,
    has_error: bool,
}

/*
 * All functions that start with parse (except parse() itself)
 * should set the current index to be whatever next token it did not parse.
 * In other words, the index should point to the next un-proccessed token.
 *
 * The parser is designed on the principle of "fail fast".
 */
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            index: 0,
            has_error: false,
        }
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }

    fn eof(&self) -> bool {
        self.index >= self.tokens.len()
    }

    fn current(&self) -> Token {
        self.tokens[self.index].clone()
    }

    fn check(&self, lexeme: &str) -> bool {
        self.current().get_lexeme() == lexeme
    }

    fn advance(&mut self) {
        self.index += 1
    }

    fn parse_identifier(&mut self) -> Box<Identifier> {
        if self.eof() {
            let x = Box::new(Identifier {
                id: None,
                error: Some(ParserError::UnexpectedEOF(
                    self.current().get_line(),
                    self.current().get_col(),
                    self.current().get_lexeme().to_string(),
                )),
            });
            return x;
        }
        match self.current() {
            Token::Identifier(_, _, _) => {
                self.advance();
                Box::new(Identifier {
                    id: Some(self.current()),
                    error: None,
                })
            }
            _ => {
                self.advance();
                Box::new(Identifier {
                    id: None,
                    error: Some(ParserError::InvalidSyntax(
                        self.current().get_line(),
                        self.current().get_col(),
                        self.current().get_lexeme().to_string(),
                    )),
                })
            }
        }
    }

    fn parse_generic_parameters(&mut self) -> Option<Box<GenericParameters>> {
        if !self.check("<") {
            return None;
        }
        let mut gp = Box::new(GenericParameters {
            generics: Vec::new(),
            error: None,
        });
        while !self.check(">") {
            if !self.check("type") {
                gp.error = Some(ParserError::InvalidSyntax(
                    self.current().get_line(),
                    self.current().get_line(),
                    String::from(format!(
                        "Expected a 'type' keyword, found '{}'.",
                        self.current().get_lexeme()
                    )),
                ));
                return Some(gp);
            }

            let id = self.parse_identifier();
            let id2: Option<Box<Identifier>>;
            if !self.check("impl") {
                if !self.check(",") {
                    gp.error = Some(ParserError::InvalidSyntax(
                        self.current().get_line(),
                        self.current().get_col(),
                        String::from(format!(
                            "Expected a keyword 'impl' or a separator ',', found '{}'.",
                            self.current().get_lexeme()
                        )),
                    ));
                    return Some(gp);
                }
            } else {
                id2 = Some(self.parse_identifier());
            }

            if !self.check(",") {
                gp.error = Some(ParserError::InvalidSyntax(
                    self.current().get_line(),
                    self.current().get_col(),
                    format!(
                        "Expected a separator ',' , found '{}'.",
                        self.current().get_lexeme()
                    ),
                ));
            }
        }

        return Some(gp);
    }

    fn parse_fn_parameters(&mut self) -> Option<Vec<(Box<Type>, Box<Identifier>)>> {
        todo!();
    }

    fn parse_block(&mut self) -> Result<Box<Block>, ParserError> {
        todo!();
    }

    fn parse_fn(&mut self, is_pub: bool, is_const: bool) -> Box<FunctionDeclaration> {
        self.advance(); // skip 'fn'
        let id = self.parse_identifier();
        if id.error.is_some() {
            return Box::new(FunctionDeclaration {
                id: id.clone(),
                is_pub,
                is_const,
                generics: None,
                parameters: None,
                block: Box::new(Block {
                    statements: Vec::new(),
                }),
                error: id.error,
            });
        }
        let generics = self.parse_generic_parameters();
        if generics.is_some() && generics.as_ref().unwrap().error.is_some() {
            return Box::new(FunctionDeclaration {
                id,
                is_pub,
                is_const,
                generics: None,
                parameters: None,
                block: Box::new(Block {
                    statements: Vec::new(),
                }),
                error: generics.unwrap().error,
            });
        }
        let parameters = self.parse_fn_parameters();
        let block = self.parse_block();
        Box::new(FunctionDeclaration {
            id,
            is_pub,
            is_const,
            generics,
            parameters,
            block: block.expect(""),
            error: None,
        })
    }

    fn parse_declaration(&mut self) -> Box<Declaration> {
        let is_pub = self.check("pub");
        if is_pub {
            self.advance();
        }

        let is_const = self.check("const");

        if self.check("fn") {
            return Box::new(Declaration::Function(self.parse_fn(is_pub, is_const)));
        }

        if self.check("enum") {
            if is_const {
                return Box::new(Declaration::Error(ParserError::InvalidSyntax(
                    self.current().get_line(),
                    self.current().get_col(),
                    String::from("The `const` keyword cannot be used with `enum` types."),
                )));
            }
        }

        if self.check("struct") {
            if is_const {
                return Box::new(Declaration::Error(ParserError::InvalidSyntax(
                    self.current().get_line(),
                    self.current().get_col(),
                    String::from("The `const` keyword cannot be used with `struct` types."),
                )));
            }
        }

        if self.check("intf") {
            if is_const {
                return Box::new(Declaration::Error(ParserError::InvalidSyntax(
                    self.current().get_line(),
                    self.current().get_col(),
                    String::from("The `const` keyword cannot be used with `intf` types."),
                )));
            }
        }

        Box::new(Declaration::Error(
            crate::utils::ParserError::UnexpectedToken(
                self.current().get_line(),
                self.current().get_col(),
                self.current().get_lexeme().to_string(),
            ),
        ))
    }

    pub fn parse(&mut self) -> Box<AST> {
        let mut ast = Box::new(AST {
            declarations: Vec::new(),
        });

        while !self.eof() {
            ast.declarations.push(self.parse_declaration());
            self.index += 1;
        }

        ast
    }
}

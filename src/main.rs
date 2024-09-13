use lexer::Lexer;
use token::Token;
use utils::ParserError;
mod ast;
mod lexer;
mod parser;
mod token;
mod utils;

fn lexer_errors(tokens: &Vec<Token>) {
    for tok in tokens {
        match &tok {
            Token::Error(e) => {
                eprintln!("{}", e);
            }
            _ => {}
        }
    }
}

fn parser_errors(ast: &Box<ast::AST>) {
    for decl in &ast.declarations {
        match decl.as_ref() {
            ast::Declaration::Error(e) => {
                eprintln!("{}", e);
            }
            _ => {}
        }
    }
}

fn main() {
    let mut lexer = lexer::Lexer::new(
        "\nif go then 數據無法訪問 run {+=x} \n \"數據無法訪問\\\"\" \n 數據無法訪問\"",
    );
    //benchmark_number();
    //benchmark_identifier();
    let tokens = lexer.lex();
    if lexer.has_error() {
        lexer_errors(&tokens);
        return;
    }

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();
    if parser.has_error() {
        parser_errors(&ast);
        // TODO: Write error handler.
    }
}

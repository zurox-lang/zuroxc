mod utils;
mod token;
mod lexer;
mod ast;
mod parser;

fn main() {
    let mut lexer = lexer::Lexer::new("if go then run {+=x} \n \"數據無法訪問\\\"\" \n 數據無法訪問");
    let tokens = lexer.lex();
    for tok in tokens.iter() {
        println!("Token: {}", tok);
    }
}

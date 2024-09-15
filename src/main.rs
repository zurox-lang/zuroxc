use clap::Parser;
use clap_derive::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::path::{Path, PathBuf};

mod ast;
mod cache;
mod lexer;
mod parser;
mod token;
mod utils;

#[derive(Parser, Debug)]
#[command(name = "zuroxc")]
#[command(version = "0.1.0")]
#[command(propagate_version = true)]
#[command(
    about = "Zurox Programming Language Compiler",
    long_about = "zuroxc is the compiler used to compile the new, simple and fast language Zuroxc. Check the documentation for more information."
)]
struct Cli {
    /// The files to operate on
    #[arg(short, long, value_name = "{FILES}")]
    files: Vec<PathBuf>,

    /// The name/path of the file to which the output should be saved.
    #[arg(short, long, value_name = "[OUTPUT_FILE]")]
    output: Option<PathBuf>,

    /// The path to store the cache in.
    #[arg(short, long)]
    cache_dir: Option<PathBuf>,

    /// The level of optimization that should be performed.
    #[arg(short, long, value_enum)]
    optimization: Optimization,

    /// Target CPU microarchitecture
    #[arg(long)]
    target_cpu: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Optimization {
    /// Enable very minimal optimizations (inlining).
    O0,
    /// Enable minimal optimizations.
    O1,
    /// Enable default optimizations.
    O2,
    /// Enable expensive optimizations. Their performance should be proportional to the percent increase in build times.
    O3,
    /// Enable debugging, no optimizations.
    Og,
    /// Optimize binaries for size, not performance.
    Oz,
}

impl ToString for Optimization {
    fn to_string(&self) -> String {
        match self {
            Optimization::O0 => "O0",
            Optimization::O1 => "O1",
            Optimization::O2 => "O2",
            Optimization::O3 => "O3",
            Optimization::Og => "Og",
            Optimization::Oz => "Oz",
        }
        .to_string()
    }
}

#[derive(Subcommand)]
enum Commands {
    Link {},
    Compile {},
    Check {},
    EmitVMCode {},
    ClearCache {},
}

fn highlight(file: &str, line: usize, col: usize, value: &str) {}

fn lexer_errors(tokens: &Vec<token::Token>) {
    for tok in tokens {
        match &tok {
            token::Token::Error(e) => {
                eprintln!("{}", e);
                match e {
                    _ => {}
                }
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

fn get_cache_dir(cli_cache_dir: Option<PathBuf>) -> PathBuf {
    match cli_cache_dir {
        Some(path) => {
            if !path.exists() {
                // Attempt to create the directory if it doesn't exist
                if let Err(e) = fs::create_dir_all(&path) {
                    eprintln!("Error creating cache directory: {}", e);
                    std::process::exit(1);
                }
            }
            path
        }
        None => {
            // Default to "./.zuroxc/cache/" if no cache_dir is provided
            let default_cache_dir: PathBuf = [".", ".zuroxc", "cache"].iter().collect();

            if !default_cache_dir.exists() {
                if let Err(e) = fs::create_dir_all(&default_cache_dir) {
                    eprintln!("Error creating default cache directory: {}", e);
                    std::process::exit(1);
                }
            }

            default_cache_dir
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.files.is_empty() {
        eprintln!("Error: No input files specified.");
        std::process::exit(1);
    }

    let cache_dir = get_cache_dir(cli.cache_dir);

    for file in cli.files {
        let file_path_str = file
            .to_str()
            .expect("Failed to convert file path to string.");

        // Check if the file exists in the cache, using the cache directory
        if !cache::file_exists_in_cache(
            cache::get_hash(file_path_str).unwrap().as_str(),
            cache_dir.to_str().expect("Invalid cache directory"),
        ) {
            // Lexer
            let mut lexer = lexer::Lexer::new(
                "\nif go then 數據無法訪問 run {+=x} \n \"數據無法訪問\\\"\" \n 數據無法訪問\"",
            );

            let tokens = lexer.lex();
            if lexer.has_error() {
                lexer_errors(&tokens);
                return;
            }

            // Parser
            let mut parser = parser::Parser::new(tokens);
            let ast = parser.parse();
            if parser.has_error() {
                parser_errors(&ast);
                // TODO: Write error handler.
            }
        }
    }
}

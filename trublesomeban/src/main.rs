mod parser;

use nexer::lexer;
use std::{env, fs::read_to_string};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv = env::args().collect::<Vec<String>>();
    let s = read_to_string(&argv[1])?;
    let tokens = lexer::TokenStream::new(
        &s,
        &["func", "if", "else", "when", "do"],
        &["=", "+", "-", "*", "/", "%", "!", "|", "&","&&", "||", ":", ":=", "!="],
        &['"'],
        &['#'],
    );
    let ast = parser::Parser::new(
        tokens,
        &[
            (":=", 1, parser::OpType::Binary),
            ("||", 2, parser::OpType::Binary),
            ("&&", 3, parser::OpType::Binary),
            ("=", 4, parser::OpType::Binary),
            ("!=", 4, parser::OpType::Binary),
            ("|", 5, parser::OpType::Binary),
            ("+", 5, parser::OpType::Binary),
            ("-", 5, parser::OpType::Binary),
            ("*", 6, parser::OpType::Binary),
            ("/", 6, parser::OpType::Binary),
            ("%", 6, parser::OpType::Binary),
            ("!", 7, parser::OpType::Unary),
        ],
    )
    .parse();
    Ok(())
}

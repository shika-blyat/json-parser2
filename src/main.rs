mod ast;
mod errors;
mod lexer;
mod parser;
mod tokens;

use std::convert::TryInto;

use errors::SyntaxErr;
use lexer::Lexer;
use parser::Parser;
use tokens::Token;
const CODE: &'static str = r#"
{
    "a": ["a", "b", 1],
    "caca": {
        "b": true,
    }
}
"#;

fn main() {
    let lexer = Lexer::new(CODE);
    let tokens: Result<Vec<Token>, SyntaxErr> = lexer.try_into();
    println!("{:#?}", tokens);
    let parser = Parser {
        tokens: tokens.unwrap().into_iter().peekable(),
    };
    println!("{:#?}", parser.json());
}

#![allow(dead_code)]

mod lexer;
mod shared;
mod token;

fn main() {
    let code = "let x = 5 + 5;".to_owned();

    let characters: Vec<char> = code.chars().collect();
    let lex = lexer::Lexer::new(characters);
}

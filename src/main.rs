#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

mod lexer;
mod shared;
mod token;

fn main() {
    let code = "let five = 5;".to_owned();

    let characters: Vec<char> = code.chars().collect();
    let mut lex = lexer::Lexer::new(characters);
    let tokens = lex.run();
    for t in tokens {
        println!("{}", t);
    }
}

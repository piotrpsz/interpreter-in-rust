#![allow(dead_code)]

use crate::token;
use crate::token::{Token, TokenType};

const ZERO_CHAR: char = 0 as char;

pub struct Lexer {
    input: Vec<char>,
    size: usize,
    pos: usize,
    read_pos: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer {
        let size = input.len();
        let mut lexer = Lexer {
            input,
            size,
            pos: 0,
            read_pos: 0,
            ch: ZERO_CHAR,
        };
        lexer.read_char();
        lexer
    }

    pub fn run(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let t = self.next_token();
            let stop = t.is_eof();
            tokens.push(t);
            if stop {
                break;
            }
        }
        tokens
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(token::EQ, "==".to_string())
                } else {
                    Token::new(token::ASSIGN, '='.to_string())
                }
            }
            '+' => Token::new(token::PLUS, '+'.to_string()),
            '-' => Token::new(token::MINUS, '-'.to_string()),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(token::NOT_EQ, "!=".to_string())
                } else {
                    Token::new(token::BANG, '!'.to_string())
                }
            }
            '/' => Token::new(token::SLASH, '/'.to_string()),
            '*' => Token::new(token::ASTRISK, '*'.to_string()),
            '<' => Token::new(token::LT, '<'.to_string()),
            '>' => Token::new(token::GT, '>'.to_string()),
            ';' => Token::new(token::SEMICOLON, ';'.to_string()),
            ',' => Token::new(token::COMMA, ','.to_string()),
            '{' => Token::new(token::LBRACE, '{'.to_string()),
            '}' => Token::new(token::RBRACE, '}'.to_string()),
            '(' => Token::new(token::LPAREN, '('.to_string()),
            ')' => Token::new(token::RPAREN, '('.to_string()),
            ZERO_CHAR => Token::new(token::EOF, "".to_string()),
            _ => {
                if self.ch.is_alphabetic() {
                    let literal = self.read_identifier();
                    let name = token::lookup(&literal);
                    Token::new(name, literal)
                } else if self.ch.is_digit(10) {
                    Token::new(token::INT, self.read_number())
                } else {
                    Token::new(token::ILLEGAL, self.ch.to_string())
                }
            }
        };

        self.read_char();
        token
    }

    fn read_char(&mut self) {
        self.ch = if self.read_pos < self.size {
            self.input[self.read_pos]
        } else {
            ZERO_CHAR
        };
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek_char(&mut self) -> char {
        if self.read_pos < self.size {
            return self.input[self.read_pos];
        }
        ZERO_CHAR
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.pos;
        while self.ch.is_alphabetic() {
            self.read_char();
        }
        let text: String = self.input[pos..self.pos].iter().collect();
        text
    }

    fn read_number(&mut self) -> String {
        let pos = self.pos;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        let text: String = self.input[pos..self.pos].iter().collect();
        text
    }

    fn skip_whitespaces(&mut self) {
        while self.is_whitespace() {
            self.read_char();
        }
    }

    #[inline]
    fn is_whitespace(&self) -> bool {
        match self.ch {
            ' ' | '\t' | '\n' | '\r' => true,
            _ => false,
        }
    }

    fn new_token(&self, name: TokenType, vc: char) -> Token {
        Token::new(name, vc.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r"let five = 5;
let ten = 10;
let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
";

        #[derive(Debug)]
        struct Result {
            expected_name: TokenType,
            expected_literal: &'static str,
        }

        let tests = [
            Result {
                expected_name: token::LET,
                expected_literal: "let",
            },
            Result {
                expected_name: token::IDENT,
                expected_literal: "five",
            },
            Result {
                expected_name: token::ASSIGN,
                expected_literal: "=",
            },
            Result {
                expected_name: token::INT,
                expected_literal: "5",
            },
            Result {
                expected_name: token::SEMICOLON,
                expected_literal: ";",
            },
        ];

        let mut lex = Lexer::new(input.chars().collect());
        for t in &tests {
            let retval = lex.next_token();
            println!("{}, {:?}", retval, t);
            assert_eq!(retval.name(), t.expected_name);
            assert_eq!(retval.literal(), t.expected_literal);
        }
    }
}

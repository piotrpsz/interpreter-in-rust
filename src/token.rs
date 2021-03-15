/***
* Project: parser-in- rust
* File   : token.rs
* Autor  : Piotr Pszczółkowski (piotr@beesoft.pl)
* Licence: MIT
*/

use std::collections::HashMap;
use std::fmt;

pub type TokenType = &'static str;

pub const ILLEGAL: &'static str = "ILLEGAL";
pub const EOF: &'static str = "EOF";
pub const IDENT: &'static str = "IDENT";
pub const INT: &'static str = "INT";
pub const ASSIGN: TokenType = "=";
pub const PLUS: &'static str = "+";
pub const MINUS: &'static str = "-";
pub const BANG: &'static str = "!";
pub const ASTRISK: &'static str = "*";
pub const SLASH: &'static str = "/";
pub const LT: &'static str = "<";
pub const GT: &'static str = ">";
pub const EQ: TokenType = "==";
pub const NOT_EQ: &'static str = "==";
pub const COMMA: &'static str = ",";
pub const SEMICOLON: &'static str = ";";
pub const LPAREN: &'static str = "(";
pub const RPAREN: &'static str = ")";
pub const LBRACE: &'static str = "{";
pub const RBRACE: &'static str = "}";
pub const FUNCTION: &'static str = "FUNCTION";
pub const LET: &'static str = "LET";
pub const TRUE: &'static str = "TRUE";
pub const FALSE: &'static str = "FALSE";
pub const IF: &'static str = "IF";
pub const ELSE: &'static str = "ELSE";
pub const RETURN: &'static str = "RETURN";

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, &'static str> = {
        let mut keys = HashMap::new();
        keys.insert("fn", FUNCTION);
        keys.insert("let", LET);
        keys.insert("true", TRUE);
        keys.insert("false", FALSE);
        keys.insert("if", IF);
        keys.insert("else", ELSE);
        keys.insert("return", RETURN);
        keys
    };
}

#[derive(Debug, PartialEq)]
pub struct Token {
    name: TokenType,
    literal: String,
}

impl Token {
    pub fn new(name: TokenType, literal: String) -> Token {
        Token { name, literal }
    }

    #[inline]
    pub fn name(&self) -> TokenType {
        return self.name;
    }

    #[inline]
    pub fn literal(&self) -> String {
        return self.literal.clone();
    }

    #[inline]
    pub fn is_eof(&self) -> bool {
        self.name == EOF
    }

    #[inline]
    pub fn is_illegal(&self) -> bool {
        self.name == ILLEGAL
    }
}

pub fn lookup(ident: &str) -> TokenType {
    if let Some(token) = KEYWORDS.get(ident) {
        return token;
    }
    IDENT
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token<{} |{}|>", self.name, self.literal)
    }
}

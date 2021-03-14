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

pub struct Token {
    name: TokenType,
    literal: String,
}

impl Token {
    pub fn new(name: TokenType, literal: String) -> Token {
        Token { name, literal }
    }
}

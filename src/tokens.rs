use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Delim {
    Paren,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operator::Plus => write!(f, "Operator < Plus `+` >"),
            Operator::Minus => write!(f, "Operator < Minus `-` >"),
            Operator::Star => write!(f, "Operator < Star `*` >"),
            Operator::Slash => write!(f, "Operator < Slash `/` >"),
            Operator::Percent => write!(f, "Operator < Percent `%` >"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Eof,
    Def,
    Extern,
    Ident(String),
    Number(f64),

    // Structural tokens
    OpenDelim(Delim),
    CloseDelim(Delim),
    Comma,
    Semicolon,

    // Expression tokens
    Eq,
    EqEq,
    Lt,
    Le,
    Gt,
    Ge,
    Ne,
    AndAnd,
    OrOr,
    Bang,

    BinOp(Operator),
    BinOpEq(Operator),

    // Useless tokens
    Whitespace,
    Comment,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Eof => write!(f, "Token < End-of-file >"),
            Token::Def => write!(f, "Token < Function Def >"),
            Token::Extern => write!(f, "Token < Extern >"),
            Token::Ident(ref s) => write!(f, "Token < Identifier: `{}` >", s),
            Token::Number(ref val) => write!(f, "Token < Number: `{}` >", val),
            Token::OpenDelim(_) => write!(f, "Token < Open Delimiter: Paren `(` >"),
            Token::CloseDelim(_) => write!(f, "Token < Closing Delimiter: Paren `)` >"),
            Token::Eq => write!(f, "Token < Eq `=` >"),
            Token::EqEq => write!(f, "Token < EqEq `==` >"),
            Token::Lt => write!(f, "Token < Lt `<` >"),
            Token::Le => write!(f, "Token < Le `<=` >"),
            Token::Gt => write!(f, "Token < Gt `>` >"),
            Token::Ge => write!(f, "Token < Ge `>=` >"),
            Token::Ne => write!(f, "Token < Ne `!=` >"),
            Token::AndAnd => write!(f, "Token < AndAnd `&&` >"),
            Token::OrOr => write!(f, "Token < OrOr `||` >"),
            Token::Bang => write!(f, "Token <Bang `!` >"),
            Token::BinOp(ref op) => write!(f, "Token < Binop: {} >", op),
            Token::BinOpEq(ref op) => write!(f, "Token < BinopEq: {} >", op),
            Token::Whitespace => write!(f, "Token < Whitespace >"),
            Token::Comment => write!(f, "Token < Comment >"),
            Token::Semicolon => write!(f, "Token < Semicolon >"),
            Token::Comma => write!(f, "Token < Comma >"),
        }
    }
}

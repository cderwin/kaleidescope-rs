use ast::BinOp;
use tokens::{Token, Operator};


pub enum Op {
    ///  `+`
    Add,

    ///  `-`
    Subtract,

    ///  `*`
    Multiply,

    ///  `/`
    Divide,

    ///  `%`
    Modulus,

    ///  `&&`
    And,

    ///  `||`
    Or,

    ///  `==`
    Equal,

    ///  `!=`
    NotEqual,

    ///  `>`
    Greater,

    ///  `>=`
    GreaterEqual,

    ///  `<`
    Less,

    ///  `<=`
    LessEqual,
}

impl Op {
    pub fn from_token(tok: &Token) -> Option<Op> {
        use self::Op::*;
        match *tok {
            Token::BinOp(Operator::Plus) => Some(Add),
            Token::BinOp(Operator::Minus) => Some(Subtract),
            Token::BinOp(Operator::Star) => Some(Multiply),
            Token::BinOp(Operator::Slash) => Some(Divide),
            Token::BinOp(Operator::Percent) => Some(Modulus),
            Token::AndAnd => Some(And),
            Token::OrOr => Some(Or),
            Token::EqEq => Some(Equal),
            Token::Ne => Some(NotEqual),
            Token::Gt => Some(Greater),
            Token::Ge => Some(GreaterEqual),
            Token::Lt => Some(Less),
            Token::Le => Some(LessEqual),
            _ => None
        }
    }

    pub fn precedence(&self) -> usize {
        use self::Op::*;
        match *self {
            Multiply | Divide | Modulus => 40,
            Add | Subtract => 30,

            Equal | NotEqual | Greater | GreaterEqual | Less | LessEqual => 10,

            And => 8,
            Or => 4,
        }
    }

    pub fn to_ast_binop(&self) -> BinOp {
        use self::Op::*;
        match *self {
            Add => BinOp::Add,
            Subtract => BinOp::Sub,
            Multiply => BinOp::Mul,
            Divide => BinOp::Div,
            Modulus => BinOp::Rem,
            And => BinOp::And,
            Or => BinOp::Or,
            Equal => BinOp::Eq,
            NotEqual => BinOp::Ne,
            Greater => BinOp::Gt,
            GreaterEqual => BinOp::Ge,
            Less => BinOp::Lt,
            LessEqual => BinOp::Le
        }
    }
}

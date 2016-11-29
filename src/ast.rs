#[derive(Debug, RustcEncodable)]
pub enum Expr {
    // f64 literal
    Number(f64),

    // variable usage
    Name(String),

    // Binary expression, i.e. `a+b` or `a*b`
    Binary(BinOp, Box<Expr>, Box<Expr>),

    // Unary expression, `!a` or `-36`
    Unary(UnOp, Box<Expr>),

    // Function call, `f(a, b, c+d)`
    Call(String, Vec<Box<Expr>>),

    // Parenthesized expression, `(a + 3*b)`
    Paren(Box<Expr>),
}

#[derive(Debug, RustcEncodable)]
pub struct FuncProto(pub String, pub Vec<String>);

#[derive(Debug, RustcEncodable)]
pub enum Item {
    Function(Box<FuncProto>, Box<Expr>),
    Extern(Box<FuncProto>),
    Expr(Box<Expr>)
}

#[derive(Debug, RustcEncodable)]
pub struct File(pub Vec<Box<Item>>);

#[derive(Debug, RustcEncodable)]
pub enum BinOp {
    Add,  // `+`
    Sub,  // `-`
    Mul,  // `*`
    Div,  // `/`
    Rem,  // `%`

    And,  // `&&`
    Or,   // `||`

    Eq,   // `==`
    Ne,   // `!=`
    Gt,   // `>`
    Ge,   // `>=`
    Lt,   // `<`
    Le,   // `<=`
}

#[derive(Debug, RustcEncodable)]
pub enum UnOp {
    Neg,   // `-`
    Not,   // `!`
}

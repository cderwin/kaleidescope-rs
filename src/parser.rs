use ast::*;
use lexer::Lexer;
use precedence::Op;
use tokens::{Token, Delim, Operator};

pub struct Parser {
    token: Token,
    lexer: Lexer
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser { lexer: lexer, token: Token::Eof };
        p.next_token();
        p
    }

    pub fn parse(&mut self) -> Box<File> {
        self.parse_file()
    }

    fn next_token(&mut self) {
        loop {
            let token = self.lexer.next_token();
            match token {
                Ok(Token::Whitespace) | Ok(Token::Comment) => continue,

                Ok(tok) => {
                    self.token = tok;
                    break
                }

                Err(e) => panic!(e)
            }
        }
    }

    fn expect(&mut self, t: Token) -> Option<String> {
        if self.token == t {
            self.next_token();
            None
        } else {
            panic!("expected {}, not {}", t, self.token);
        }
    }

    /// NUM_EXPR := NUM
    fn parse_number_expr(&mut self) -> Box<Expr> {
        let value = match self.token {
            Token::Number(val) => val,
            _ => unreachable!()
        };

        self.next_token();
        Box::new(Expr::Number(value))
    }

    /// PAREN_EXPR ::= '(' EXPR ')'
    fn parse_paren_expr(&mut self) -> Box<Expr> {
        self.next_token();
        let expr = self.parse_expr();
        self.expect(Token::CloseDelim(Delim::Paren));
        Box::new(Expr::Paren(expr))
    }
 
    /// IDENT_EXPR := IDENT | IDENT '(' EXPR [ ','  EXPR ] * ')'
    fn parse_ident_expr(&mut self) -> Box<Expr> {
        let ident_name = match self.token {
            Token::Ident(ref name) => name.clone(),
            _ => unreachable!()
        };

        self.next_token();

        if self.token != Token::OpenDelim(Delim::Paren) {
            return Box::new(Expr::Name(ident_name))
        }

        self.next_token();

        let mut args: Vec<Box<Expr>> = Vec::new();

        loop {
            let arg = self.parse_expr();
            args.push(arg);

            if self.token == Token::CloseDelim(Delim::Paren) {
                break
            }

            self.expect(Token::Comma);
        }

        self.next_token();

        Box::new(Expr::Call(ident_name, args))
    }

    /// PRIMARY_EXPR ::= [ '-' | '!' ] ? [ IDENT_EXPR | NUMBER_EXPR | PAREN_EXPR ]
    fn parse_primary(&mut self) -> Box<Expr> {
        let unary_op = match self.token {
            Token::BinOp(Operator::Minus) => {
                self.next_token();
                Some(UnOp::Neg)
            }

            Token::Bang => {
                self.next_token();
                Some(UnOp::Not)
            }

            _ => None
        };


        let expr = match self.token {
            Token::Ident(_) => self.parse_ident_expr(),
            Token::Number(_) => self.parse_number_expr(),
            Token::OpenDelim(Delim::Paren) => self.parse_paren_expr(),
            ref t => panic!("Unexpected token, `{}`", t)
        };

        if let Some(op) = unary_op {
            Box::new(Expr::Unary(op, expr))
        } else {
            expr
        }
    }

    /// BINOP_RHS ::= [ BINOP PRIMARY ]*
    /// BINOP ::= '+' | '-' | '*' | '/' | '%' | '&&' | '||'
    ///         | '==' | '!=' | '>' | '>=' | '<' | '<='
    fn parse_binop_rhs(&mut self, min_precedence: usize, mut lhs: Box<Expr>) -> Box<Expr> {
        loop {
            let op = match Op::from_token(&self.token) {
                Some(op) => op,
                None => return lhs
            };

            if op.precedence() < min_precedence {
                return lhs
            }

            self.next_token();

            let mut rhs = self.parse_primary();

            let next_precedence = match Op::from_token(&self.token) {
                Some(op) => op.precedence(),
                None => 0
            };

            if next_precedence > min_precedence {
                rhs = self.parse_binop_rhs(min_precedence + 1, rhs);
            }

            lhs = Box::new(Expr::Binary(op.to_ast_binop(), lhs, rhs))
        }
    }

    /// EXPR ::= PRIMARY BINOP_RHS ?
    fn parse_expr(&mut self) -> Box<Expr> {
        let lhs = self.parse_primary();
        self.parse_binop_rhs(0, lhs)
    }

    /// PROTOTYPE ::= IDENT '(' IDENT [ ',' IDENT ] * ')'
    fn parse_proto(&mut self) -> Box<FuncProto> {
        let name = match self.token {
            Token::Ident(ref name) => name.clone(),
            _ => unreachable!()
        };

        self.next_token();
        self.expect(Token::OpenDelim(Delim::Paren));

        let mut args: Vec<String> = Vec::new();

        loop {
            let arg_name = match self.token {
                Token::Ident(ref name) => name.clone(),
                ref t => panic!("Expected identifier, not {}", t)
            };

            self.next_token();
            args.push(arg_name);

            if let Token::CloseDelim(Delim::Paren) = self.token {
                break;
            }

            self.expect(Token::Comma);
        }

        self.next_token();
        Box::new(FuncProto(name, args))
    }

    /// FUNC_DEF ::= 'def' PROTOTYPE EXPR
    fn parse_def(&mut self) -> Box<Item> {
        self.next_token();
        let proto = self.parse_proto();
        let expr = self.parse_expr();
        Box::new(Item::Function(proto, expr))
    }

    /// EXTERN ::= 'extern' PROTOTYPE
    fn parse_extern(&mut self) -> Box<Item> {
        self.next_token();
        let proto = self.parse_proto();
        Box::new(Item::Extern(proto))
    }

    /// TOP_LEVEL_EXPR ::= EXPR
    fn parse_top_level_expr(&mut self) -> Box<Item> {
        let expr = self.parse_expr();
        Box::new(Item::Expr(expr))
    }

    fn parse_file(&mut self) -> Box<File> {
        let mut items : Vec<Box<Item>> = Vec::new();

        loop {
            let item = match self.token {
                Token::Def => self.parse_def(),
                Token::Extern => self.parse_extern(),
                Token::Semicolon => continue,
                Token::Eof => break,
                _ => self.parse_top_level_expr()
            };

            items.push(item);
        }

        Box::new(File(items))
    }
}

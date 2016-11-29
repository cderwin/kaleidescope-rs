use tokens::{Delim, Operator, Token};


pub struct Lexer {
    body: String,
    ch: Option<char>,
    index: usize,
    next_offset: usize,
}

impl Lexer {
    pub fn new(body: String) -> Self {
        let mut lexer = Lexer {
            body: body,
            ch: Some('\x00'),
            index: 0,
            next_offset: 0,
        };

        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        if self.next_offset < self.body.len() {
            self.ch = self.body[self.next_offset..].chars().next();
            self.index = self.next_offset;
            self.next_offset += match self.ch {
                Some(c) => c.len_utf8(),
                None => 0,
            };
        } else {
            self.index = self.next_offset;
            self.ch = None;
        }
    }

    fn next_ch(&self) -> Option<char> {
        self.body[self.next_offset..].chars().next()
    }

    fn body_from(&self, start: usize) -> String {
        self.body[start..self.index].to_string()
    }

    fn binop(&mut self, op: Operator) -> Token {
        if let Some('=') = self.ch {
            self.advance();
            Token::BinOpEq(op)
        } else {
            Token::BinOp(op)
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        // Detect eof
        let c = match self.ch {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };


        // Whitespace
        if c.is_whitespace() {
            self.advance();

            while let Some(c) = self.ch {
                if !c.is_whitespace() {
                    break;
                }

                self.advance();
            }

            return Ok(Token::Whitespace);
        }

        // Comments
        if c == '#' {
            loop {
                self.advance();

                match self.ch {
                    Some('\n') | Some('\r') | None => break,
                    _ => {}
                }
            }

            return Ok(Token::Comment);
        }

        // identifiers: [a-zA-Z][a-zA-Z0-9]*
        if c.is_alphabetic() {
            let ident_start = self.index;
            self.advance();

            while self.ch.unwrap_or('\x00').is_alphanumeric() {
                self.advance();
            }

            return match self.body_from(ident_start).as_ref() {
                "def" => Ok(Token::Def),
                "extern" => Ok(Token::Extern),
                s => Ok(Token::Ident(s.to_string())),
            };
        }

        // numbers: [0-9.]+
        if c.is_numeric() || c == '.' {
            let num_start = self.index;

            if c != '.' {
                self.advance();

                while self.ch.unwrap_or('\x00').is_numeric() {
                    self.advance();
                }
            }

            // TODO: make more clear why if/else would not work
            if self.ch.unwrap_or('\x00') == '.' {
                self.advance();

                while self.ch.unwrap_or('\x00').is_numeric() {
                    self.advance();
                }
            }

            let num_literal = self.body_from(num_start);
            let float: f64 = num_literal.parse().unwrap();

            return Ok(Token::Number(float));
        }

        // Small tokens
        match c {
            // Delimiters
            '(' => {
                self.advance();
                return Ok(Token::OpenDelim(Delim::Paren));
            }

            ')' => {
                self.advance();
                return Ok(Token::CloseDelim(Delim::Paren));
            }

            // Expression tokens
            '=' => {
                self.advance();
                if let Some('=') = self.ch {
                    self.advance();
                    return Ok(Token::EqEq);
                }

                return Ok(Token::Eq);
            }

            '<' => {
                self.advance();
                if let Some('=') = self.ch {
                    self.advance();
                    return Ok(Token::Le)
                }

                return Ok(Token::Lt)
            }

            '>' => {
                self.advance();
                if let Some('=') = self.ch {
                    self.advance();
                    return Ok(Token::Ge);
                }

                return Ok(Token::Gt)
            }

            '!' => {
                self.advance();
                if let Some('=') = self.ch {
                    self.advance();
                    return Ok(Token::Ne)
                }

                return Ok(Token::Bang)
            }

            '&' => {
                if let Some('&') = self.next_ch() {
                    self.advance();
                    self.advance();
                    return Ok(Token::AndAnd)
                }

                return Err("`&` must be followed by `&` to form valid token".to_string())
            }

            '|' => {
                if let Some('|') = self.next_ch() {
                    self.advance();
                    self.advance();
                    return Ok(Token::OrOr)
                }

                return Err("`|` must be followed by `|` to form valid token".to_string())
            }

            '+' => {
                self.advance();
                return Ok(self.binop(Operator::Plus));
            }

            '-' => {
                self.advance();
                return Ok(self.binop(Operator::Minus));
            }

            '*' => {
                self.advance();
                return Ok(self.binop(Operator::Star));
            }

            '/' => {
                self.advance();
                return Ok(self.binop(Operator::Slash));
            }

            '%' => {
                self.advance();
                return Ok(self.binop(Operator::Percent));
            }

            // Seperators
            
            ',' => {
                self.advance();
                return Ok(Token::Comma)
            }

            ';' => {
                self.advance();
                return Ok(Token::Semicolon)
            }

            // If no token matches, return error
            _ => {
                let msg = format!("Character not recognized: `{}`", c);
                return Err(msg);
            }
        }
    }
}

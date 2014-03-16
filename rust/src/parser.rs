use lexer::TokenValue;
use lexer;
use ast::{Ast, NumberExprAst, BindingExprAst, LetExprAst, BinaryExprAst};
use std::from_str;

static BinaryPlusPrec: uint = 20;
static BinaryMinusPrec: uint = 20;
static BinaryMulPrec: uint = 40;
static BinaryDivPrec: uint = 40;

pub struct Parser<'a> {
    next_token: Option<TokenValue>,
    current_token: Option<TokenValue>,
    lexer: lexer::Lexer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            next_token: None,
            current_token: None,
            lexer: lexer::Lexer::new(input)
        }
    }

    pub fn bump(&mut self) {
        if self.next_token.is_some() {
            self.current_token = self.next_token.take();
        } else {
            self.current_token = self.lexer.next_token();
        }
    }

    pub fn peek(&mut self) {
        self.next_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) {
        
    }

    pub fn parse_number(&mut self) -> Ast {
        let num: i32 = match self.current_token {
            Some(ref token) => from_str(token.value).unwrap(),
            None => { fail!("Invalid token found.") }
        };
        let expr = NumberExprAst(num);
        self.bump();
        return expr;
    }

    /// An expression is of the form:
    /// 
    /// (iden K..)
    /// 
    /// Where `K` is of any type.
    /// 
    /// @effect(Fail)
    /// 
    pub fn parse_expression(&mut self) -> Ast {
        let mut tok = self.current_token.take().unwrap();
        
        if tok.token != lexer::LPAREN {
            fail!("Expected an expression to begin with `(` (left parentheses).");
        }

        // Parse an iden
        self.bump();

        tok = self.current_token.take().unwrap();

        if tok.token != lexer::IDEN {
            fail!("Expected an identifier next.");
        }

        self.bump();
        //self.parse_form(tok);

        NumberExprAst(1)
    }

    pub fn parse_form(&mut self, tok: lexer::TokenValue) -> Ast {
        println!("{}", tok.value);
        return NumberExprAst(4);
        match tok.value {
            ~"defn" => fail!("Functions are not implemented yet."),
            ~"let" => {
                // Parse the bindings.
                match self.current_token.take() {
                    Some(r) => {
                        if r.value == ~"[" {
                            NumberExprAst(9)
                        } else {
                            fail!("Expected `[` token.")
                        }
                    },
                    None => fail!("Expected `[` token, but found none.")
                }
            },
            _ => fail!("Unimplemented form.")
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use ast::*;

    #[test]
    fn parse_number() {
        let mut parser = Parser::new(&"9");
        parser.bump();
        match parser.parse_number() {
            NumberExprAst(r) => assert_eq!(r, 9),
            _ => fail!("Not expected.")
        }
    }

    #[test]
    fn parse_expression() {
        let mut parser = Parser::new(&"(let [x 5])");
        parser.bump();
        parser.parse_expression();
        //match parser.parse_expression() {
        //    NumberExprAst(r) => assert_eq!(r, 9),
        //    _ => fail!("Not expected.")
        //}
    }
}
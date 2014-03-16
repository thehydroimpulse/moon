use lexer::TokenValue;
use lexer;
use ast;
use ast::{Ast, NumberExprAst, LetExprAst, BinaryExprAst};
use std::from_str;

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            lexer: lexer::Lexer::new(input)
        }
    }

    pub fn bump(&mut self) -> Option<TokenValue> {
        self.lexer.next_token()
    }

    pub fn parse(&mut self) {
        
    }

    pub fn parse_number(&mut self, tok: TokenValue) -> Ast {
        let num: i32 = match from_str::<i32>(tok.value) {
            Some(r) => r,
            None => fail!("Unable to pase number.")
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
        let mut tok = self.bump().unwrap();
        
        match tok.token {
            lexer::LPAREN => {
                // Parse an iden
                tok = self.bump().unwrap();

                if tok.token != lexer::IDEN {
                    fail!("Expected an identifier next.");
                }

                self.parse_form(tok)
            },
            lexer::INTEGER => {
                self.parse_number(tok)
            },
            _ => fail!("Expected an expression to begin with `(` (left parentheses) or a number.")
        }
    }

    pub fn parse_form(&mut self, tok: lexer::TokenValue) -> Ast {

        match tok.value {
            ~"defn" => fail!("Functions are not implemented yet."),
            ~"let" => {
                // Parse the bindings.
                match self.current_token.take() {
                    Some(r) => {
                        if r.value == ~"[" {
                            let bindings = ~[ast::BindingExprAst];
                            // Loop until we find the end bracket or fail.
                            loop {
                                self.bump();
                                let curr = self.current_token.take().unwrap();
                                let expr = match curr.token { 
                                    lexer::IDEN => {
                                        self.bump();
                                        // Parse another expression
                                        let value = ~self.parse_expression();
                                        let b = ast::BindingExprAst(curr.value, value);
                                        //bindings.push();
                                    },
                                    lexer::RBRACKET => break,
                                    _ => {}
                                };
                            }

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
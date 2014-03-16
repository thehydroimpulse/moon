use lexer::{Token, TokIden, TokInt, RBRACKET, LBRACKET, LPAREN, RPAREN, COLON, INTEGER, PLUS, MINUS};
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

    pub fn bump(&mut self) -> Option<Token> {
        self.lexer.next_token()
    }

    pub fn parse(&mut self) {
        
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

        match tok {
            lexer::LPAREN => {
                // Parse an iden
                match self.bump().unwrap() {
                    TokIden(s) => {
                        self.parse_form(tok)
                    },
                    _ => fail!("Expected an iden token.")
                }
            },
            TokInt(i) => {
                NumberExprAst(i)
            },
            _ => fail!("Expected an expression to begin with `(` (left parentheses) or a number.")
        }
    }

    pub fn parse_form(&mut self, tok: Token) -> Ast {

        match tok {
            TokIden(~"defn") => fail!("Functions are not implemented yet."),
            TokIden(~"let")  => {
                // Parse the bindings.
                match self.bump() {
                    Some(r) => {
                        if r == TokIden(~"[") {
                            let bindings = ~[ast::BindingExprAst];
                            // Loop until we find the end bracket or fail.
                            loop {

                                let expr = match self.bump().unwrap() {
                                    TokIden(r) => {
                                        println!("curr{}", r);
                                        // Parse another expression
                                        let value = ~self.parse_expression();
                                        let b = ast::BindingExprAst(r, value);
                                        //bindings.push();
                                    },
                                    _ => fail!("oops")
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
    fn parse_expression() {
        let mut parser = Parser::new(&"(let [x 5])");
        parser.parse_expression();
        //match parser.parse_expression() {
        //    NumberExprAst(r) => assert_eq!(r, 9),
        //    _ => fail!("Not expected.")
        //}
    }
}
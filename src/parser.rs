use lexer::{Token, TokIden, TokInt, RBRACKET, LBRACKET, LPAREN, RPAREN, COLON, INTEGER, PLUS, MINUS};
use lexer;
use ast::{Ast, NumberExprAst, BindingExprAst, LetExprAst, BinaryExprAst, Empty};
use ast;

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
        let tok = self.bump().unwrap();
        println!("(expr {})", tok);
        match tok {
            lexer::LPAREN => {
                println!("found lparen");
                // Parse an iden
                match self.bump().unwrap() {
                    TokIden(s) => {
                        self.parse_form(TokIden(s))
                    },
                    _ => fail!("Expected an iden token.")
                }
            },
            TokIden(s) => {
                fail!("Unexpected iden. Expected an expression.")
            }
            TokInt(i) => {
                NumberExprAst(i)
            },
            _ => fail!("Expected an expression to begin with `(` (left parentheses) or a number.")
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        self.lexer.peek_token()
    }

    pub fn parse_form(&mut self, tok: Token) -> Ast {
        match tok {
            TokIden(~"defn") => fail!("Functions are not implemented yet."),
            TokIden(~"let")  => {
                // Parse the bindings.
                match self.bump() {
                    Some(r) => {
                        if r == LBRACKET {
                            let mut bindings: ~[~Ast] = ~[];
                            let mut found_ending = false;
                            // Loop until we find the end bracket or fail.
                            loop {
                                match self.bump() {
                                    Some(tt) => {
                                        let expr = match tt {
                                            TokIden(r) => {
                                                println!("(iden {})", r);
                                                // Parse another expression
                                                let value = ~self.parse_expression();
                                                let b = BindingExprAst(r, value);
                                                bindings.push(~b);
                                            },
                                            RBRACKET => {
                                                found_ending = true;
                                                break
                                            },
                                            _ => break
                                        };
                                    },
                                    None => break
                                }
                            }

                            if !found_ending {
                                fail!("Expected ending `]` bracket.");
                            }

                            match self.peek() {
                                Some(m) => {
                                    match m {
                                        RPAREN => {
                                            LetExprAst(bindings, ~Empty)
                                        },
                                        _ => {
                                            LetExprAst(bindings, ~self.parse_expression())
                                        }
                                    }
                                },
                                None => fail!("Expected ending `)`.")
                            }
                        } else {
                            fail!("Expected `[` token.")
                        }
                    },
                    None => fail!("Expected `[` token, but found none.")
                }
            },
            _ => fail!("Unimplemented form. {}", tok)
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
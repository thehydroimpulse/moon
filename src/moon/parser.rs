use lexer::{Token,TokIden,TokInt,RBRACKET,LBRACKET,LPAREN,RPAREN};
use lexer;
use ast::{Ast,NumberExprAst,BindingExprAst,LetExprAst,Empty};

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
            TokIden(_) => {
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
                                        match tt {
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
                                        }
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
    fn parse_let_expression() {
        let mut parser = Parser::new(&"(let [x 5])");
        let expr = LetExprAst(~[~BindingExprAst(~"x", ~NumberExprAst(5))], ~Empty);
        assert_eq!(expr, parser.parse_expression());
    }

    #[test]
    fn parse_multiple_let_bindings() {
        let mut parser = Parser::new(&"(let [x 5
                                             y 10])");
        let expr = LetExprAst(~[
            ~BindingExprAst(~"x", ~NumberExprAst(5)),
            ~BindingExprAst(~"y", ~NumberExprAst(10))
        ], ~Empty);
        assert_eq!(expr, parser.parse_expression());
    }
}
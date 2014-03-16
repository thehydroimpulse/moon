use lexer::TokenValue;
use lexer;
use ast::{Ast, NumberExprAst, VariableExprAst, BinaryExprAst};
use std::from_str;

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
    fn parse_binary() {
        let mut parser = Parser::new(&"9+5");
        parser.bump();
        match parser.parse_number() {
            NumberExprAst(r) => assert_eq!(r, 9),
            _ => fail!("Not expected.")
        }


    }
}
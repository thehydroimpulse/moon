use lexer::TokenValue;
use lexer;
use ast::{Ast, NumberExprAst, VariableExprAst, BinaryExprAst};
use std::from_str;

pub struct Parser<'a> {
    current_token: Option<TokenValue>,
    lexer: lexer::Lexer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            current_token: None,
            lexer: lexer::Lexer::new(input)
        }
    }

    pub fn bump(&mut self) {
        self.current_token = self.lexer.next_token();
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
}
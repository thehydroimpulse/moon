use lexer::TokenValue;
use lexer;

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
}
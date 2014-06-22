use std::str::Chars;
use std::io::Reader;
use span::Span;

#[deriving(PartialEq, Show)]
pub enum Token {
    PLUS,
    MINUS,
    TIMES,
    AT,
    CARET,
    LPAREN,
    RPAREN,
    Noop
}

/// The lexer is the system that has any notion of an input stream. The
/// stream can come from many sources, thus, we use the Reader trait
/// that is implemented for strings, files, tcp, etc...
///
/// The lexer should also not consume the whole stream at once, as
/// that will be a bottleneck for larger input sources.
pub struct Lexer<'a> {
    source: &'a str,
    iter: Chars<'a>,
    token: Token,
    span: Span
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source: source,
            iter: source.chars(),
            token: Noop,
            span: Span::new(0, 0)
        }
    }


    /// Parse and fetch the next token in the text stream.
    /// This doesn't return the token, it stores it
    /// in the lexer.
    pub fn bump(&mut self) {

    }

    pub fn expect(&mut self, tok: &Token) {
        if self.token == *tok {
            self.bump();
        } else {
            fail!("Expected {} but found {}", *tok, self.token);
        }
    }
}

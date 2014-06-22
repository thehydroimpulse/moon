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
    Noop,
    Done
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
        // Fetch the next char.
        let c = match self.iter.next() {
            Some(c) => c,
            None => {
                self.token = Done;
                return;
            }
        };

        // Begin parsing an s-expression.
        if c == '(' {

        }
    }

    /// Expect a given token to come next, otherwise
    /// fail. Failing will result in a formatted
    /// error message that uses the Span and lets
    /// the user know what went wrong.
    pub fn expect(&mut self, tok: &Token) {
        if self.token == *tok {
            self.bump();
        } else {
            fail!("Expected {} but found {}", *tok, self.token);
        }
    }

    /// Optionally eat a given token. If the token is not
    /// found, this will simply return false and
    /// won't fail.
    pub fn eat(&mut self, tok: &Token) -> bool {
        if self.token == *tok {
            self.bump();
            true
        } else {
            false
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn noop() {
        let lexer = Lexer::new("foo");
        assert_eq!(lexer.token, Noop);
    }

}

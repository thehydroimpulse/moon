use std::str::Chars;
use std::io::Reader;
use std::str::SendStr;
use span::Span;
use std::str::Owned;

#[deriving(PartialEq, Show)]
pub enum Token {
    Ident(String),
    Str(String),
    Keyword(SendStr),
    Colon,
    Caret,
    LParen,
    RParen,
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

        match c {
            '(' => { self.token = LParen },
            ')' => { self.token = RParen },
            '^' => { self.token = Caret },
            ':' => {
                let mut keyword = String::new();
                self.iter.next().while_some(|a| {
                    if self.is_keyword(a) {
                        keyword.push_char(a);
                        self.iter.next()
                    } else {
                        None
                    }
                });

                self.token = Keyword(Owned(keyword));
            },
            '"' => {
                let mut concat = String::new();
                self.iter.next().while_some(|a| {
                    if a == '"' {
                        None
                    } else {
                        concat.push_char(a);
                        self.iter.next()
                    }
                });

                self.token = Str(concat);
            },
            // An identifier is composed of any
            // characters that contain no whitespace
            // and doesn't start with the designated
            // characters above.
            ch => {
                let mut ident = String::new();
                ident.push_char(ch);
                self.iter.next().while_some(|a| {
                    if self.is_ident(a) {
                        ident.push_char(a);
                        self.iter.next()
                    } else {
                        None
                    }
                });

                self.token = Ident(ident);
            }
        }
    }

    /// Determine if a char is allowed within an identifier.
    pub fn is_ident(&self, ch: char) -> bool {
        if ch.is_whitespace() || ch == ':' || ch == '(' || ch == ')' || ch == '"'
            || ch == '@' || ch == '!' {
            false
        } else {
            true
        }
    }

    /// Verify if the char is within the allowed character set for keywords. This
    /// only checks a single character at a time.
    #[inline]
    pub fn is_keyword(&self, a: char) -> bool {
        if a.is_whitespace() || a == '(' || a == ')' || a == '^' || a == ':' {
            false
        } else {
            true
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
    use span::Span;
    use std::str::{Slice, Owned};

    #[test]
    fn noop() {
        let lexer = Lexer::new("foo");
        assert_eq!(lexer.token, Noop);
    }

    #[test]
    fn zeroed_span() {
        let lexer = Lexer::new("foo");
        assert_eq!(lexer.span, Span::new(0, 0));
    }

    #[test]
    fn bump_empty() {
        let mut lexer = Lexer::new("");
        lexer.bump();
        assert_eq!(lexer.token, Done);
    }

    #[test]
    fn bump_lparen() {
        let mut lexer = Lexer::new("(");
        lexer.bump();
        assert_eq!(lexer.token, LParen);
    }

    #[test]
    fn bump_rparen() {
        let mut lexer = Lexer::new(")");
        lexer.bump();
        assert_eq!(lexer.token, RParen);
    }

    #[test]
    fn bump_simple_keyword() {
        let mut lexer = Lexer::new(":foo");
        lexer.bump();
        assert_eq!(lexer.token, Keyword(Slice("foo")));
    }

    #[test]
    fn bump_keyword_dash() {
        let mut lexer = Lexer::new(":foo-");
        lexer.bump();
        assert_eq!(lexer.token, Keyword(Slice("foo-")));
    }

    #[test]
    fn bump_keyword_dash_middle() {
        let mut lexer = Lexer::new(":foo-bar");
        lexer.bump();
        assert_eq!(lexer.token, Keyword(Slice("foo-bar")));
    }

    #[test]
    fn bump_string() {
        let mut lexer = Lexer::new("\"hah-w091:--:\"");
        lexer.bump();
        assert_eq!(lexer.token, Str("hah-w091:--:".to_string()));
    }

    #[test]
    fn bump_ident() {
        let mut lexer = Lexer::new("foo-bar-&#^");
        lexer.bump();
        assert_eq!(lexer.token, Ident("foo-bar-&#^".to_string()));
    }

    #[test]
    fn should_eat_lparen() {
        let mut lexer = Lexer::new("(");
        lexer.bump();
        assert_eq!(lexer.eat(&LParen), true);
    }
}

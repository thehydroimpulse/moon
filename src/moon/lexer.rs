use std::str::Chars;
use std::io::Reader;
use std::str::SendStr;
use span::Span;
use std::str::Owned;
use std::from_str::from_str;

fn is_num_type(n: char) -> bool {
    if n != '6' && n != '4' && n != '8' && n != '3' && n != '2' {
        false
    } else {
        true
    }
}

/// A `Token` represents a special symbol within
/// the language. These do not contain any extra metadata
/// with it, that's stored elsewhere.
///
/// Unlike most languages, characters like `+`, `-`, `*`, `/`
/// are **not** considered tokens, but, in fact regular identifiers.
/// This allows us to implement these without the compiler
/// to have any sort of knowledge of them.
#[deriving(PartialEq, Show, Clone)]
pub enum Token {
    Ident(String),
    Str(String),
    Integer(i64),
    LBracket,
    RBracket,
    Comma,
    Keyword(SendStr),
    Colon,
    Caret,
    LParen,
    RParen,
    Noop,
    Done
}

#[deriving(PartialEq, Show, Clone)]
pub enum IntegerParserState {
    NumInit,
    /// We have found a type (such as `6u`, `9i`, etc..),
    /// however, we might have more information coming up, such as
    /// the specialization of the type.
    NumTyped(String),

    /// This means we have seen a typed integer, but not a fully
    /// specialized one.
    NumTypedPartial(String),

    /// The integer type has been more specific. This allows us
    /// to know when we have successfully specialized an integer.
    /// However, we still need an intermediate step to where
    /// the integer has been typed, but not fully specialized.
    NumTypedSpecialized(String)
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
    done: bool,
    span: Span
}

impl<'a> Lexer<'a> {
    /// Return a brand new lexer containing an iterator
    /// of chars and a bunch of state to keep track of (such
    /// as the current Span).
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source: source,
            iter: source.chars(),
            token: Noop,
            done: false,
            span: Span::new(0, 0)
        }
    }

    pub fn collect(&mut self) -> Vec<Token> {
        let mut vec = Vec::new();

        self.bump().while_some(|tok| {
            if tok != Done && tok != Noop {
                vec.push(tok);
            }

            if !self.done {
                self.bump()
            } else {
                None
            }
        });

        vec
    }

    /// Parse and fetch the next token in the text stream.
    /// This doesn't return the token, it stores it
    /// in the lexer.
    pub fn bump(&mut self) -> Option<Token> {
        // Fetch the next token so that we can start matching
        // against it. If we don't return anything, we'll set
        // the token to `Done`, resulting in a completed lexer.
        //
        // FIXME: I don't think I'm checking for a `None` case
        //        anywhere else.
        let c = match self.iter.next() {
            Some(c) => c,
            None => {
                self.done  = true;
                self.token = Done;
                return None;
            }
        };

        match c {
            ' ' => { return self.bump() },
            '\n' => { return self.bump() },
            '\t' => { return self.bump() },
            ',' => { self.token = Comma },
            // Parse some simple tokens.
            '(' => { self.token = LParen },
            ')' => { self.token = RParen },
            '^' => { self.token = Caret },
            '[' => { self.token = LBracket },
            ']' => { self.token = RBracket },
            num @ '0'..'9' | num @ '-' => {
                // Hold a collection of numbered chars so that
                // we can convert to a proper number format later.
                let mut combined = String::new();

                // We need to keep track of the current state
                // for the integer. This let's us know whether
                // we're parsing a specialization, or the user
                // has supplied an invalid form.
                let mut state = NumInit;

                // We need to push the one char we grabbed before
                // all the while_some stuff.
                combined.push_char(num);

                // Iterate over the next char until we return
                // `None` or the iterator returns `None`.
                self.iter.next().while_some(|n| {

                    match n {
                        // Ok, so we have a simple number, that's
                        // pretty easy. Simply push the char
                        // to the string.
                        //
                        // However, we need to first check
                        // what state we're in, currently.
                        '0'..'9' => {
                            match state.clone() {
                                // We haven't seen any further information
                                // about the integer, so we can just
                                // push the char onto the string normally.
                                NumInit => combined.push_char(n),
                                NumTyped(ref kind) => {
                                    if !is_num_type(n) {
                                        fail!("Unexpected specializer {}", n);
                                    }

                                    let mut s = kind.clone();
                                    s.push_char(n);
                                    state = NumTypedPartial(s);
                                },
                                NumTypedPartial(ref kind) => {
                                    if !is_num_type(n) {
                                        fail!("Unexpected specializer {}", n);
                                    }

                                    if kind.as_slice() == "u8" {
                                        fail!("Unexpected character {}, expected nothing.", n);
                                    }

                                    let mut s = kind.clone();
                                    s.push_char(n);
                                    let rev = kind.as_slice().chars().last();
                                    state = NumTypedSpecialized(s);
                                },
                                NumTypedSpecialized(ref kind) => {
                                    println!("x {}", n);
                                }
                            }
                        },
                        'u' => state = NumTyped("u".to_string()),
                        'i' => state = NumTyped("i".to_string()),
                        '\n' => return None,
                        n if n.is_whitespace() => return None,
                        _ => fail!("A number must be isolated from other tokens.")
                    }

                    self.iter.next()
                });

                let num = match from_str::<i64>(combined.as_slice()) {
                    Some(num) => num,
                    None => fail!("Not a number.")
                };

                self.token = Integer(num);
            },
            // Parse a simple keyword. Keywords are somewhat like
            // strings, but, they're more limited in the format, but
            // a lot nicer to use.
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
            // Parse the beginning of a string until we find the ending
            // quote. Anything can be placed within the double quotes.
            //
            // This doesn't support any escaping yet, so that'll break.
            // To support it, we'll need to keep track of the previous
            // token. Finding a `\` character will result in it
            // being saved until we look at the next character; which
            // we'll look back at the previous character to figure
            // out our current state. Escaping will only be set if
            // it matches a good escape pattern.
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

                if self.is_ident(ch) {
                    ident.push_char(ch);
                } else {
                    // Push an `Unknown` token here.
                }

                self.iter.next().while_some(|a| {
                    if self.is_ident(a) {
                        ident.push_char(a);
                        self.iter.next()
                    } else {
                        None
                    }
                });

                if ident.len() > 0 {
                    self.token = Ident(ident);
                }
            }
        }

        Some(self.token.clone())
    }

    /// Determine if a char is allowed within an identifier.
    /// An identifier can be used for:
    ///
    /// * Function names
    /// * Binding names
    /// * Other
    ///
    /// Because many of the built-in forms have identifiers
    /// such as `+`, `-`, `*`, `/`, we can't make these tokens,
    /// nor can we exclude them from the identifier format.
    pub fn is_ident(&self, ch: char) -> bool {
        if ch.is_whitespace() || ch == ':' || ch == '(' || ch == ')' || ch == '"'
            || ch == '@' || ch == '!' || ch == '\n' || ch == '\t' || ch == ',' {
            false
        } else {
            true
        }
    }

    /// Verify if the char is within the allowed character set for keywords. This
    /// only checks a single character at a time.
    #[inline]
    pub fn is_keyword(&self, a: char) -> bool {
        if a.is_whitespace() || a == '(' || a == ')' || a == '^' || a == ':' || a == ',' {
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

    #[test]
    #[should_fail]
    fn should_expect_lparen() {
        let mut lexer = Lexer::new(")");
        lexer.bump();
        lexer.expect(&LParen);
    }

    #[test]
    fn should_expect() {
        let mut lexer = Lexer::new("(");
        lexer.bump();
        lexer.expect(&LParen);
    }
}

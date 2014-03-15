use std::str::Chars;
use std::fmt::{Show,Formatter};
use std::fmt;
use std::cast::transmute;

pub struct TokenValue {
    token: Token,
    value: ~str
}

impl TokenValue {
    pub fn new(tok: Token, val: ~str) -> TokenValue {
        TokenValue {
            token: tok,
            value: val
        }
    }
}

/// Lexer tokens.
pub enum Token {
    PLUS,
    MINUS,
    INTEGER, // Any type of integer
    IDEN, // Identifier
    LPAREN,
    RPAREN,
    COLON
}

/// Lexer
pub struct Lexer<'a> {
    iter: Chars<'a>,
    done: bool
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        let it = input.chars();
        Lexer {
            iter: it,
            done: false
        }
    }

    pub fn next_token(&mut self) -> Option<TokenValue> {
        for c in self.iter {
            match c {
                ' ' => continue,
                '\r' => continue,
                '\n' => continue,
                '(' => return Some(TokenValue::new(LPAREN, ~"(")),
                ')' => return Some(TokenValue::new(RPAREN, ~")")),
                '+' => return Some(TokenValue::new(PLUS, ~"+")),
                '-' => return Some(TokenValue::new(MINUS, ~"-")),
                ':' => return Some(TokenValue::new(COLON, ~":")),
                '9' => return Some(TokenValue::new(INTEGER, ~"9")),
                '8' => return Some(TokenValue::new(INTEGER, ~"8")),
                '7' => return Some(TokenValue::new(INTEGER, ~"7")),
                '6' => return Some(TokenValue::new(INTEGER, ~"6")),
                '5' => return Some(TokenValue::new(INTEGER, ~"5")),
                '4' => return Some(TokenValue::new(INTEGER, ~"4")),
                '3' => return Some(TokenValue::new(INTEGER, ~"3")),
                '2' => return Some(TokenValue::new(INTEGER, ~"2")),
                '1' => return Some(TokenValue::new(INTEGER, ~"1")),
                '0' => return Some(TokenValue::new(INTEGER, ~"0")),
                _ => {

                    let mut it = self.iter;
                    let mut current = ~"";
                    let mut previous = ~"";
                        
                    // Push the current character
                    current.push_char(c);

                    for character in it {
                        // Lets save the current buffer as the previous.
                        // This ensures that if this character makes the invalidation,
                        // we can return the previously valid string.
                        previous = current.clone();

                        // Push the new chracter onto the current buffer.
                        current.push_char(character);

                    //     // FIXME(TheHydroImpulse): A huge fucking hack.
                        if !is_iden(current) && is_iden(previous) {
                            return Some(TokenValue::new(IDEN, previous));
                    //      println!("ooo");
                        }
                    }

                    if !is_iden(current) && is_iden(previous) {
                        return Some(TokenValue::new(IDEN, previous));
                    }
                }
            }
        }
        None
    }
}

// An iden is identified by:
// /^([A-Za-z-+*^][A-Za-z0-9-_]*)$/
pub fn is_iden<'b>(ch: &'b str) -> bool {
    // Create a new iterator
    let mut iter = ch.chars().peekable();

    // Verify the first character.
    if !verify_iden_start(iter.next().unwrap()) {
        return false;
    }

    for x in iter {
        match x {
            '\n' => return false,
            '\r' => return false,
            ' ' => return false,
            _  =>  if !verify_iden_start(x) { return false }
        }
    }

    true
}


pub fn verify_iden_start(ch: char) -> bool {
    match ch {
        'A' => true,
        'a' => true,
        'B' => true,
        'b' => true,
        'C' => true,
        'c' => true,
        'D' => true,
        'd' => true,
        'E' => true,
        'e' => true,
        'F' => true,
        'f' => true,
        'G' => true,
        'g' => true,
        'H' => true,
        'h' => true,
        'I' => true,
        'i' => true,
        'J' => true,
        'j' => true,
        'K' => true,
        'k' => true,
        'L' => true,
        'l' => true,
        'M' => true,
        'm' => true,
        'N' => true,
        'n' => true,
        'O' => true,
        'o' => true,
        'P' => true,
        'p' => true,
        'Q' => true,
        'q' => true,
        'R' => true,
        'r' => true,
        'S' => true,
        's' => true,
        'T' => true,
        't' => true,
        'U' => true,
        'u' => true,
        'V' => true,
        'v' => true,
        'W' => true,
        'w' => true,
        'Y' => true,
        'y' => true,
        'Z' => true,
        'z' => true,
        '+' => true,
        '-' => true,
        '*' => true,
        _ => false 
    }
}


impl Eq for Token {
    fn eq(&self, other: &Token) -> bool {
        match *self {
            PLUS => match *other { PLUS => true, _ => false },
            MINUS => match *other { MINUS => true, _ => false },
            INTEGER => match *other { INTEGER => true, _ => false },
            IDEN => match *other { IDEN => true, _ => false },
            LPAREN => match *other { LPAREN => true, _ => false },
            RPAREN => match *other { RPAREN => true, _ => false },
            COLON => match *other { COLON => true, _ => false }
        }
    }
}

impl Show for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let c = match *self {
            PLUS => &"+",
            MINUS => &"-",
            INTEGER => &"integer",
            IDEN => &"iden",
            LPAREN => &"(",
            RPAREN => &")",
            COLON => ":"
        };

        write!(f.buf, "({})", c)
    }
}


#[cfg(test)]
mod test {
    use super::Lexer;
    use super::LPAREN;
    use super::TokenValue;

    #[test]
    fn next_token() {
        // Should spit out two tokens (LPAREN,RPAREN)
        let mut lex = Lexer::new(&"()");
        assert!(lex.next_token().unwrap().value == ~"(");
        assert!(lex.next_token().unwrap().value == ~")");  
    }

    #[test]
    fn return_complex_tokens() {
        let mut lex = Lexer::new(&"(+(():830");

        assert!(lex.next_token().unwrap().value == ~"(");
        assert!(lex.next_token().unwrap().value == ~"+");
        assert!(lex.next_token().unwrap().value == ~"(");
        assert!(lex.next_token().unwrap().value == ~"(");
        assert!(lex.next_token().unwrap().value == ~")");
        assert!(lex.next_token().unwrap().value == ~":");
        assert!(lex.next_token().unwrap().value == ~"8");
        assert!(lex.next_token().unwrap().value == ~"3");
        assert!(lex.next_token().unwrap().value == ~"0");
    }

    #[test]
    fn should_tokenize_identifier() {
        let mut lex = Lexer::new(&"(let)");

        assert_eq!(lex.next_token().unwrap().value, ~"(");
        assert_eq!(lex.next_token().unwrap().value, ~"let");
    }
}
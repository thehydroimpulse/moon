use std::fmt::{Show,Formatter};
use std::fmt;
use std::cast::transmute;
use std::str::CharRange;

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

struct LexerIterator<'a> {
    string: &'a str,
    pos: uint,
    line: uint,
    next: uint,
    current: Option<char>,
    peek: Option<char>,
    previous: Option<char>
}

impl<'a> LexerIterator<'a> {
    pub fn new(string: &'a str) -> LexerIterator<'a> {
        LexerIterator {
            string: string,
            pos: 0,
            next: 0,
            line: 1,
            current: None,
            peek: None,
            previous: None
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if self.string.len() != 0 && self.pos < self.string.len() {
            let CharRange {ch, next} = self.string.char_range_at(self.pos);
            self.pos = next;

            if self.current.is_some() {
                self.previous = self.current;
            }

            self.current = Some(ch);
            Some(ch)
        } else {
            None
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        if self.string.len() != 0 && self.pos < self.string.len() {
            let CharRange {ch, next} = self.string.char_range_at(self.pos);
            self.peek = Some(ch);
            Some(ch)
        } else {
            None
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
    COLON,
    LBRACKET,
    RBRACKET
}


pub enum TokValue {
    TokInt(Token, i32),
    TokStr(Token, ~str),
    Tok(Token)
}

/// Lexer
pub struct Lexer<'a> {
    iter: *mut LexerIterator<'a>,
    done: bool
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            iter: unsafe { transmute(~LexerIterator::new(input)) },
            done: false
        }
    }

    pub fn next_token(&mut self) -> Option<TokenValue> {
        unsafe {
            for c in *self.iter {
                match c {
                    ' ' => continue,
                    '\r' => continue,
                    '\n' => {
                        (*self.iter).line += 1;
                        continue 
                    },
                    '[' => return Some(TokenValue::new(LBRACKET, ~"[")),
                    ']' => return Some(TokenValue::new(RBRACKET, ~"]")),
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

                        let mut current = ~"";
                        let mut previous = ~"";
                            
                        // Push the current character
                        current.push_char(c);
                        println!("{}", current);

                        loop {
                            // Try the next character. If we get a valid identifier
                            // than we can move the position of the iterator.
                            match (*self.iter).peek() {
                                Some(character) => {
                                    // Lets save the current buffer as the previous.
                                    // This ensures that if this character makes the invalidation,
                                    // we can return the previously valid string.
                                    previous = current.clone();

                                    // Push the new chracter onto the current buffer.
                                    current.push_char(character);

                                    if is_iden(current) {
                                        (*self.iter).next();
                                    }

                                    if !is_iden(current) && is_iden(previous) {
                                        return Some(TokenValue::new(IDEN, previous));
                                    }
                                },
                                None => { break }
                            }
                        }

                        if !is_iden(current) && is_iden(previous) {
                            return Some(TokenValue::new(IDEN, previous));
                        }
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
        'X' => true,
        'x' => true,
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
            COLON => match *other { COLON => true, _ => false },
            RBRACKET => match *other { RBRACKET => true, _ => false },
            LBRACKET => match *other { LBRACKET => true, _ => false }
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
            COLON => ":",
            LBRACKET => "[",
            RBRACKET => "]"
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
        assert_eq!(lex.next_token().unwrap().value, ~")");
    }

    #[test]
    fn should_forget_whitespace() {
        let mut lex = Lexer::new(&"( )");
        assert_eq!(lex.next_token().unwrap().value, ~"(");
        assert_eq!(lex.next_token().unwrap().value, ~")");
    }

    #[test]
    fn should_count_lines() {
        let mut lex = Lexer::new(&"(\n)");
        assert_eq!(lex.next_token().unwrap().value, ~"(");
        assert_eq!(lex.next_token().unwrap().value, ~")");

        assert_eq!(unsafe { (*lex.iter).line }, 2);
    }

    #[test]
    fn should_identify_an_iden_across_lines() {
        let mut lex = Lexer::new(&"(\nlet\n)");
        assert_eq!(lex.next_token().unwrap().value, ~"(");
        assert_eq!(lex.next_token().unwrap().value, ~"let");
        assert_eq!(lex.next_token().unwrap().value, ~")");
    }

    #[test]
    fn should_test_iden_patterns() {
        let mut lex = Lexer::new(&"(\n*true*\n)");
        assert_eq!(lex.next_token().unwrap().value, ~"(");
        assert_eq!(lex.next_token().unwrap().value, ~"*true*");
        assert_eq!(lex.next_token().unwrap().value, ~")");
    }
}
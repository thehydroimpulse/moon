use std::str::Chars;
use std::fmt::{Show,Formatter};
use std::fmt;
use std::cast::transmute;

pub struct TokenValue<'a> {
  token: Token,
  value: &'a str
}

/// Lexer
pub struct Lexer<'a> {
  line: int,
  column: int,
  pos: int,
  iter: Chars<'a>,
  curr_ch: Option<char>,
  done: bool
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


impl<'a> TokenValue<'a> {
  pub fn new(tok: Token, val: &'a str) -> TokenValue<'a> {
    TokenValue {
      token: tok,
      value: val
    }
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

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Lexer<'a> {
    let it = input.chars();
    Lexer {
      line: 0,
      column: 0,
      pos: 0,
      iter: it,
      curr_ch: None,
      done: false
    }
  }

  pub fn bump(&mut self) {
    self.curr_ch = self.iter.next();
  }

  pub fn nextch_is(&mut self, ch: char) -> bool {
    self.bump();
    self.curr_ch.unwrap() == ch
  }

  pub fn next_token(&mut self) -> Option<TokenValue<'a>> {
    
    for c in self.iter {
      match c {
        '(' => return Some(TokenValue::new(LPAREN, &"(")),
        ')' => return Some(TokenValue::new(RPAREN, &")")),
        '+' => return Some(TokenValue::new(PLUS, &"+")),
        '-' => return Some(TokenValue::new(MINUS, &"-")),
        ':' => return Some(TokenValue::new(COLON, &":")),
        '9' => return Some(TokenValue::new(INTEGER, &"9")),
        '8' => return Some(TokenValue::new(INTEGER, &"8")),
        '7' => return Some(TokenValue::new(INTEGER, &"7")),
        '6' => return Some(TokenValue::new(INTEGER, &"6")),
        '5' => return Some(TokenValue::new(INTEGER, &"5")),
        '4' => return Some(TokenValue::new(INTEGER, &"4")),
        '3' => return Some(TokenValue::new(INTEGER, &"3")),
        '2' => return Some(TokenValue::new(INTEGER, &"2")),
        '1' => return Some(TokenValue::new(INTEGER, &"1")),
        '0' => return Some(TokenValue::new(INTEGER, &"0")),
        _ => {
          println!("--");
          let mut it = self.iter;

          // Try and parse an identifier
          let mut iden = ~"";
          let mut prev = ~"";
          iden.push_char(c);

          for a in it {
            // FIXME(TheHydroImpulse): A huge fucking hack.
            if !is_iden(iden) && is_iden(prev) {
              return Some(TokenValue::new(IDEN, unsafe {
                transmute(prev.as_slice())
              }));
            }

            prev = iden.clone();
            iden.push_char(a);
            println!("iden = {}, is_iden(iden) = {}, prev = {}, ch = {}", 
                iden, is_iden(iden), prev, a);
          }

          if !is_iden(iden) && is_iden(prev) {
            return Some(TokenValue::new(IDEN, unsafe {
              transmute(prev.as_slice())
            }));
          }
        }
      }
    }

    None
  }

}


#[cfg(test)]
mod test {
  use super::Lexer;
  use super::LPAREN;
  use super::TokenValue;

  #[test]
  fn bump() {
    let mut lex = Lexer::new(&"Hello World");
    lex.bump();
    assert_eq!(lex.curr_ch.unwrap(), 'H');
  }

  #[test]
  fn nextch() {
    let mut lex = Lexer::new(&"Woot");
    assert!(lex.nextch_is('W'));
  }

  #[test]
  fn next_token() {
    // Should spit out two tokens (LPAREN,RPAREN)
    let mut lex = Lexer::new(&"()");
    assert!(lex.next_token().unwrap().value == &"(");
    assert!(lex.next_token().unwrap().value == &")");  
  }

  #[test]
  fn return_complex_tokens() {
    let mut lex = Lexer::new(&"(+(():830");

    assert!(lex.next_token().unwrap().value == &"(");
    assert!(lex.next_token().unwrap().value == &"+");
    assert!(lex.next_token().unwrap().value == &"(");
    assert!(lex.next_token().unwrap().value == &"(");
    assert!(lex.next_token().unwrap().value == &")");
    assert!(lex.next_token().unwrap().value == &":");
    assert!(lex.next_token().unwrap().value == &"8");
    assert!(lex.next_token().unwrap().value == &"3");
    assert!(lex.next_token().unwrap().value == &"0");
  }

  #[test]
  fn should_tokenize_identifier() {
    let mut lex = Lexer::new(&"(let)");
    println!("1");
    assert_eq!(lex.next_token().unwrap().value, &"(");
    assert_eq!(lex.next_token().unwrap().value, &"let");
  }

}
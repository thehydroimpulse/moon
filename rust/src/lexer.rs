use std::str::Chars;
use std::iter::Peekable;
use std::fmt::{Show,Formatter};
use std::fmt;

pub struct TokenValue<'a> {
  token: Token,
  value: &'a str
}

/// Lexer
pub struct Lexer<'a> {
  line: int,
  column: int,
  pos: int,
  iter: Peekable<char, Chars<'a>>,
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

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Lexer<'a> {
    let it = input.chars().peekable();
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

  pub fn peekch_is(&mut self, ch: char) -> bool {
    *self.iter.peek().unwrap() == ch
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
        _ => return None
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
  fn peekable() {
    let mut lex = Lexer::new(&"Haha");
    lex.bump();
    assert!(lex.peekch_is('a'));
  }

  #[test]
  fn next_token() {
    // Should spit out two tokens (LPAREN,RPAREN)
    let mut lex = Lexer::new(&"()");
    assert!(lex.next_token().unwrap().value == &"("); //TokenValue::new(LPAREN, "("));
  }

}
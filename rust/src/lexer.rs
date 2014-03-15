use std::str::Chars;
use std::iter::Peekable;

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

}


#[cfg(test)]
mod test {
  use super::Lexer;

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

}
use std::str::Chars;

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

  
  
}
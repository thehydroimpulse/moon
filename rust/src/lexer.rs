use std::str::Chars;

/// Lexer
pub struct Lexer<'a> {
  line: int,
  column: int,
  pos: int,
  iter: Chars<'a>
}

/// Lexer tokens.
pub enum Token {
  PLUS,
  MINUS,
  INTEGER, // Any type of integer
  IDEN // Identifier
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Lexer<'a> {
    let it = input.chars();
    Lexer {
      line: 0,
      column: 0,
      pos: 0,
      iter: it
    }
  }

  pub fn bump(&mut self) {
    
  }
}
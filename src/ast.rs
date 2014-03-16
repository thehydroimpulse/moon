use std::rc::Rc;
use lexer;
use std::fmt::{Show,Formatter};
use std::fmt;
use std::cast::transmute;

#[deriving(Eq, Clone, Show)]
pub enum Ast {
    LetExprAst(~[~Ast], ~Ast),
    BindingExprAst(~str, ~Ast),
    NumberExprAst(i32),
    BinaryExprAst(lexer::Token, ~Ast, ~Ast),
    Empty
}

#[cfg(test)]
mod test {
    use super::Ast;
 
}
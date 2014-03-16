use std::rc::Rc;
use lexer;
use std::fmt::{Show,Formatter};
use std::fmt;
use std::cast::transmute;

#[deriving(Clone)]
#[deriving(Eq)]
pub enum Ast {
    LetExprAst(~[~Ast], ~Ast),
    BindingExprAst(~str, ~Ast),
    NumberExprAst(i32),
    BinaryExprAst(lexer::Token, ~Ast, ~Ast),
    Empty
}

impl Show for Ast {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f.buf, "{}", "")
    }
}


#[cfg(test)]
mod test {
    use super::Ast;
 
}
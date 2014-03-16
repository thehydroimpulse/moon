use std::rc::Rc;
use lexer;

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
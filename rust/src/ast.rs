use std::rc::Rc;
use lexer;

pub enum Ast {
    VariableExprAst(~str),
    NumberExprAst(i32),
    BinaryExprAst(lexer::Token, ~Ast, ~Ast)
}


#[cfg(test)]
mod test {
    use super::Ast;
 
}
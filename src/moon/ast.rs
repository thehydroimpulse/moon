#[deriving(Eq, Clone, Show)]
pub enum Ast {
    LetExprAst(~[~Ast], ~Ast),
    BindingExprAst(~str, ~Ast),
    NumberExprAst(int),
    //BinaryExprAst(lexer::Token, ~Ast, ~Ast),
    Empty
}

#[cfg(test)]
mod test {
 
}
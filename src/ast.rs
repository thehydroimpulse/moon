use std::rc::Rc;
use lexer;
use std::fmt::{Show,Formatter};
use std::fmt;
use std::cast::transmute;

#[deriving(Clone)]
pub enum Ast {
    LetExprAst(~[~Ast], ~Ast),
    BindingExprAst(~str, ~Ast),
    NumberExprAst(i32),
    BinaryExprAst(lexer::Token, ~Ast, ~Ast),
    Empty
}

impl Eq for Ast {
    fn eq(&self, other: &Ast) -> bool {
        match *self {
            Empty => { match *other { Empty => true, _ => false }},
            _ => false
        }
    }
}

unsafe fn serialize_ast(s: Ast) -> ~str {
    let mut buf = ~"";
    let c = s.clone();
    match s {
        LetExprAst(ref bindings, ref body) => {
            
        },
        _ => {}
    }

    buf
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
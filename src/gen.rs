use vm::{OpCode, XAdd};
use ast::{Ast,NumberExprAst};
use lexer::{TokInt};


pub fn gen_add(a: &Ast, b: &Ast) -> OpCode {
    match *a {
        NumberExprAst(i) => match *b {
            NumberExprAst(ii) => XAdd(i, ii),
            _ => fail!("[gen_add] expected an integer.")
        },
        _ => fail!("[gen_add] expected an integer.")
    }
}
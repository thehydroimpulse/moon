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


#[cfg(test)]
mod test {
    use super::*;
    use ast::{NumberExprAst};
    use vm::{XAdd};

    #[test]
    fn test_gen_add() {
        let op = gen_add(&NumberExprAst(5), &NumberExprAst(2));
        match op {
            XAdd(a, b) => {
                assert_eq!(a, 5);
                assert_eq!(b, 2);
            },
            _ => fail!("Failed")
        }
    }
}
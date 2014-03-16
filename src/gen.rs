use vm::{OpCode, XAdd, XMul, XSub, XDiv};
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

pub fn gen_sub(a: &Ast, b: &Ast) -> OpCode {
    match *a {
        NumberExprAst(i) => match *b {
            NumberExprAst(ii) => XSub(i, ii),
            _ => fail!("[gen_add] expected an integer.")
        },
        _ => fail!("[gen_add] expected an integer.")
    }
}

pub fn gen_mul(a: &Ast, b: &Ast) -> OpCode {
    match *a {
        NumberExprAst(i) => match *b {
            NumberExprAst(ii) => XMul(i, ii),
            _ => fail!("[gen_add] expected an integer.")
        },
        _ => fail!("[gen_add] expected an integer.")
    }
}

pub fn gen_div(a: &Ast, b: &Ast) -> OpCode {
    match *a {
        NumberExprAst(i) => match *b {
            NumberExprAst(ii) => XDiv(i, ii),
            _ => fail!("[gen_add] expected an integer.")
        },
        _ => fail!("[gen_add] expected an integer.")
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use ast::{NumberExprAst};
    use vm::{XAdd,XSub,XMul,XDiv};

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

    #[test]
    fn test_gen_sub() {
        let op = gen_sub(&NumberExprAst(5), &NumberExprAst(2));
        match op {
            XSub(a, b) => {
                assert_eq!(a, 5);
                assert_eq!(b, 2);
            },
            _ => fail!("Failed")
        }
    }

    #[test]
    fn test_gen_mul() {
        let op = gen_mul(&NumberExprAst(5), &NumberExprAst(2));
        match op {
            XMul(a, b) => {
                assert_eq!(a, 5);
                assert_eq!(b, 2);
            },
            _ => fail!("Failed")
        }
    }

    #[test]
    fn test_gen_div() {
        let op = gen_div(&NumberExprAst(5), &NumberExprAst(2));
        match op {
            XDiv(a, b) => {
                assert_eq!(a, 5);
                assert_eq!(b, 2);
            },
            _ => fail!("Failed")
        }
    }
}
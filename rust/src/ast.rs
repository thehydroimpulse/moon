use std::rc::Rc;
use lexer;

pub trait Ast {}


pub struct VariableExpr {
    name: ~str
}

impl Ast for VariableExpr {}
impl VariableExpr {
    pub fn new(name: ~str) -> VariableExpr {
        VariableExpr {
            name: name
        }
    }
}


pub struct NumberExpr {
    value: i32
}

impl Ast for NumberExpr {}
impl NumberExpr {
    pub fn new(value: i32) -> NumberExpr {
        NumberExpr {
            value: value
        }
    }
}

pub struct BinaryExpr {
    op: lexer::Token,
    lhs: ~Ast,
    rhs: ~Ast
}

impl Ast for BinaryExpr {}
impl BinaryExpr {
    pub fn new(op: lexer::Token, lhs: ~Ast, rhs: ~Ast) -> BinaryExpr {
        BinaryExpr {
            op: op,
            lhs: lhs,
            rhs: rhs
        }
    }
}

#[cfg(test)]
mod test {
    use super::Ast;
    use super::VariableExpr;

    #[test]
    fn should_create_new_var() {
        let var = VariableExpr::new(~"hello");
        assert_eq!(var.name, ~"hello");
    }

}
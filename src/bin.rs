#[feature(globs)];
#[allow(dead_code)];
#[allow(unused_variable)];
#[allow(unused_imports)];
#[allow(experimental)];

extern crate serialize;

use std::num::{pow, log10};

mod vm;
mod gc;
mod lexer;
mod ast;
mod parser;

fn main() {
    let mut parser = parser::Parser::new(&"(let [x 19])");
    parser.parse_expression();
}
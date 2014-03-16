#[feature(globs)];
#[allow(dead_code)];
#[allow(unused_variable)];
#[allow(unused_imports)];
#[allow(experimental)];

mod vm;
mod gc;
mod lexer;
mod ast;
mod parser;

fn main() {
    let mut parser = parser::Parser::new(&"(let [x 1])");
    parser.parse_expression();
}
#[feature(globs)];
mod vm;
mod gc;
mod lexer;
mod ast;
mod parser;

fn main() {
    let mut parser = parser::Parser::new(&"(let [x 1])");
    parser.bump();
    parser.parse_expression();
}
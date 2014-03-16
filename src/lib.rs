#[feature(globs)];
#[allow(dead_code)];
#[allow(unused_variable)];
#[allow(unused_imports)];
#[allow(experimental)];

extern crate serialize;

mod vm;
mod gc;
mod lexer;
mod ast;
mod parser;
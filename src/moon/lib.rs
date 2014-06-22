#![crate_id = "moon"]
#![crate_type = "lib"]
#![feature(globs, macro_rules)]
#![allow(experimental)]

pub mod vm;
pub mod ast;
pub mod lexer;
pub mod span;
pub mod result;

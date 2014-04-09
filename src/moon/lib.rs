#![crate_id = "moon"]
#![crate_type = "lib"]
#![feature(globs)]
#![allow(experimental)]

mod vm;
mod gc;
mod ast;
mod gen;
mod reader;
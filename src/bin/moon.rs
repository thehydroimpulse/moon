#![feature(globs)]

extern crate moon;
use std::io;

static color_normal: &'static str = "\x1B[0m";
static color_red: &'static str = "\x1B[31m";
static color_cyan: &'static str = "\x1B[36m";
static color_magenta: &'static str = "\x1B[35m";
static color_bold: &'static str = "\x1B[1m";

fn main() {
    let mut running = true;

    println!("moon alpha.");

    while running {
        print!("{:s}$moon{:s}{:s}>{:s} ", color_magenta, color_normal,
            color_bold, color_normal);
        for line in io::stdin().lines() {
            match line {
                Ok(line) => {
                    if line == "exit\n".to_string() {
                        running = false;
                        break;
                    }

                    print!("{}", line);
                },
                _ => {}
            }

            break;
        }
    }
}

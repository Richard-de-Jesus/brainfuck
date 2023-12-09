
use interpreter::*;
use std::{env, fs};

fn main() {

    let arg: Vec<_> = env::args().collect();

    let source = fs::read_to_string(arg[1].clone())
        .expect("file does not exists");

    execute(lexer(&source));
}

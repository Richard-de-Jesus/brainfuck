
use interpreter::*;
use std::{env, fs, time::Instant};


fn test_output(f: &str, opt: bool, debug: bool) {
    
    let foo = lexer(f, opt);
    let len = foo.len();

    execute(foo);
    if debug { 
        println!("code size: {len}"); 
    }
}
// tests lexer speed, opt will switch
// optimizations.
fn bench_lexer(f: &str, opt: bool, debug: bool) {

    let start = Instant::now();
    let foo = lexer(f, opt);

    let x = start.elapsed();
    if debug { 
        println!("code size: {}", foo.len()); 
    }

    println!("bench of lexer: {x:?}");
    // just here so the compiler will not 
    // optimize away the tokens vector.
    if f.len() == 424242 {
        println!("lucky number. here: {foo:?}");
    }
}

fn main() {

    let args: Vec<_> = env::args().collect();

    let source = fs::read_to_string(args[1].clone())
        .expect("file does not exists");
    // check options
    let options = |y| args.iter().any(|x| x == y); 

    let optimize = options("-O");
    let debug = options("--debug");
    
    if options("--bench")  {
        bench_lexer(&source, optimize, debug);
    } 
    if !options("-n") {
        test_output(&source, optimize, debug);
    }
    if options("--source") {
        let x = lexer(&source, optimize);
        println!("source: ");

        println!("{}", tokens_to_source(x));
    }
}


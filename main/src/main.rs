
use interpreter::*;
use std::{env, fs, time::Instant};


fn test_output(f: &str, opt: bool) {
    
    let foo = lexer(f, opt);
    execute(foo);
}
// tests lexer speed, opt will switch
// optimizations.
fn bench_lexer(f: &str, opt: bool) {

    let start = Instant::now();
    let foo = lexer(f, opt);

    let x = start.elapsed();
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

    let optimize = args.iter().any(|x| x == "-O");

    if args.iter().any(|x| x == "--bench")  {
        bench_lexer(&source, optimize);
    } else {
        test_output(&source, optimize);
    }
}


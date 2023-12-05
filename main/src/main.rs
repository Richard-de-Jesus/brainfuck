
use interpreter::*;


fn main() {
    let source = ",.";

    execute(lexer(source));
}

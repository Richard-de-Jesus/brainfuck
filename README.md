# brainfuck
For now, i will only write the interpreter. 
. Lexer: Done.
. Virtual machine: Done.

. Compiler: To do.
# performace

time to execute mandebrolt.bf in my PC:
no optimizations: 117 seconds.
with optimizations: 56 seconds.

# Usage
cargo run --release -- [FILE] [OPTIONS]

Options:
    . -n: do not run the program.

    . --bench: show how much time to lex the source code.

    . -O: Optimize source code.

    . --source: get source from tokens. (optimizations change the code).

    . --debug: get size of code. (optimiations reduce the size).

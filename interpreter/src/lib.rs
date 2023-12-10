use std::{io::stdin, collections::HashMap};

const DATA_SIZE: usize = 30_000;
#[derive(Debug)]
pub struct Program {
    data: [u8; DATA_SIZE],
    ptr: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    RightAngle(u8), // >
    LeftAngle(u8),  // <
    Plus(u8),
    Minus(u8),
    Dot,
    Comma,
    OpenBrace,  // [
    CloseBrace, // ]
}

macro_rules! optm {
    ($src: ident $idx: ident $val: ident $tok: expr) => {

        while $src[$idx] == $tok {
            $val += 1;
            $idx += 1;
        }
    };
}

// optimize tokens, doing constant folding on 
// Plus and Minus operations. example: 
// Plus(1), Plus(1), Plus(1) becomes Plus(3). 
fn optimize(src: &Vec<Token>) -> Vec<Token> {
    use Token as T;
    let mut output = Vec::with_capacity(src.len() / 2);

    let mut idx = 0;
    let mut value: u8;
    loop {
        value = 0;
        if idx == src.len() {
            break;
        }

        match src[idx] {
            T::Plus(n) => {                
                optm!(src idx value T::Plus(n));

                output.push(T::Plus(value));
            },
            T::Minus(n) => {
                optm!(src idx value T::Minus(n));

                output.push(T::Minus(value));
            },

            T::LeftAngle(n) => {
                optm!(src idx value T::LeftAngle(n));

                output.push(T::LeftAngle(value));
            },
            T::RightAngle(n) => {
                optm!(src idx value T::RightAngle(n));

                output.push(T::RightAngle(value));
            }
            _ => { 
                output.push(src[idx].clone());
                idx += 1;
            },
        };
    }
    output
}
// take source and convert to tokens. if opt is true,
// source will be optimized.
pub fn lexer(file: &str, opt: bool) -> Vec<Token> {
    use Token as T;

    let mut output = Vec::with_capacity(file.len());

    for ch in file.chars() {

        let token = match ch {
            '>' => T::RightAngle(1),
            '<' => T::LeftAngle(1),
            '+' => T::Plus(1),
            '-' => T::Minus(1),
            '.' => T::Dot,
            ',' => T::Comma,
            '[' => T::OpenBrace,
            ']' => T::CloseBrace,
            _ => {
                continue;
            } 
            
        };
        output.push(token);
    }
    if opt {
        output = optimize(&output);
    }

    output
}

pub fn execute(tokens: Vec<Token>) {
    use Token as T;

    let mut pg = Program {
        data: [0u8; DATA_SIZE],
        ptr: 0
    };

    let brace_list = build_brace_list(&tokens);

    let mut code_position = 0;
    while code_position < tokens.len() {

        match tokens[code_position] {
            T::Plus(n) => { 
                pg.data[pg.ptr] = pg.data[pg.ptr]
                    .wrapping_add(n); 
            },
            T::Minus(n) => { 
                pg.data[pg.ptr] = pg.data[pg.ptr]
                    .wrapping_sub(n); 
            },

            T::LeftAngle(n) => {
                let n = n as usize;
                if pg.ptr >= n {
                    pg.ptr -= n;
                } else {
                    panic!("tried to go below cell 0");
                }
            },
            T::RightAngle(n) => {
                let n = n as usize;
                if pg.ptr <= DATA_SIZE - n {
                    pg.ptr += n;
                } else {
                    panic!("tried to go above cell {DATA_SIZE}");
                }
            },

            T::Dot => print!("{}", pg.data[pg.ptr] as char),
            T::Comma => {
                let mut input = String::new();
                
                stdin().read_line(&mut input)
                    .expect("input error");

                pg.data[pg.ptr] = input.bytes()
                    .nth(0).unwrap();
            }

            T::OpenBrace => {
                if pg.data[pg.ptr] == 0 {
                    code_position = brace_list[&code_position];
                }
            }
            T::CloseBrace => {
                if pg.data[pg.ptr] != 0 {
                    code_position = brace_list[&code_position];
                }
            }
        };
        code_position += 1;
    }
}

type Braces = HashMap<usize, usize>;
fn build_brace_list(input: &Vec<Token>) -> Braces {

    let mut temp_brace_stack = vec![];
    let mut brace_list = HashMap::new();

    for (index, command) in input.iter().enumerate() {

        if *command == Token::OpenBrace {
            temp_brace_stack.push(index);
        }
        if *command == Token::CloseBrace {
            let start = temp_brace_stack.pop().unwrap();
            
            brace_list.insert(start, index);
            brace_list.insert(index, start);
        }
    }
    brace_list
}
// recommend function, lexes source and executes.
// if optimize is true, code will be optimized.
pub fn run(source: &str, optimize: bool) {
    execute(lexer(source, optimize));
}

// take tokens and convert to source code,
// comments are not included, optimizations 
// are going to reduce the source.
pub fn tokens_to_source(tokens: Vec<Token>) -> String {
    use Token as T;

    let mut output = String::with_capacity(tokens.len());

    let mut push_tok = |ch| {
        output.push(ch);
    };
    for tok in tokens {
        match tok {
            T::Plus(_) => push_tok('+'),
            T::Minus(_) => push_tok('-'),
            T::LeftAngle(_) => push_tok('<'),
            T::RightAngle(_) => push_tok('>'),
            T::Dot => push_tok('.'),
            T::Comma => push_tok(','),
            T::OpenBrace => push_tok('['),
            T::CloseBrace => push_tok(']'),
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token as T;

    #[test]
    fn test_lexer() {

        let source = "+ - <> [] 21092 \
        foobar @$#²³£¬¢£ . , ";

        let expected = vec![
            T::Plus(1), T::Minus(1),
            T::LeftAngle(1), T::RightAngle(1),
            T::OpenBrace, T::CloseBrace,
            T::Dot, T::Comma,
        ];

        assert_eq!(expected, lexer(source, false));
    }
    #[test]
    fn test_optimization() {
        let source = "++++ ++++ -- -- -- . . \
                      [++] wdkj";

        let expected = vec![
            T::Plus(8), T::Minus(6),
            T::Dot, T::Dot,
            T::OpenBrace, T::Plus(2),
            T::CloseBrace,
        ];

        assert_eq!(expected, lexer(source, true));

    }
}

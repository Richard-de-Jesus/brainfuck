use std::{io::stdin, collections::HashMap};

const DATA_SIZE: usize = 30_000;
#[derive(Debug)]
pub struct Program {
    data: [u8; DATA_SIZE],
    ptr: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    RightAngle, // >
    LeftAngle,  // <
    Plus(u8),
    Minus(u8),
    Dot,
    Comma,
    OpenBrace,  // [
    CloseBrace, // ]
}
// optimize tokens, doing constant folding on 
// Plus and Minus operations. example: 
// Plus(1), Plus(1), Plus(1) becomes Plus(3). 
#[allow(unused)]
fn optimize(src: &Vec<Token>) -> Vec<Token> {
    use Token as T;
    let mut output = Vec::with_capacity(src.len() / 2);

    let mut idx = 0;
    loop {
        let mut value = 0;
        if idx == src.len() {
            break;
        }

        match src[idx] {
            T::Plus(n) => {

                while src[idx] == T::Plus(n) {
                    value += 1;
                    idx += 1;
                }
                output.push(T::Plus(value));
            },
            T::Minus(n) => {

                while src[idx] == T::Minus(n) {
                    value += 1;
                    idx += 1;
                }
                output.push(T::Minus(value));
            },
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
pub fn lexer(file: &str, opt: bool) -> (Vec<Token>, Program) {
    use Token as T;

    let mut output = Vec::with_capacity(file.len());

    for ch in file.chars() {

        let token = match ch {
            '>' => T::RightAngle,
            '<' => T::LeftAngle,
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

    (output, Program {
        data: [0u8; DATA_SIZE],
        ptr: 0
    })
}

pub fn execute((tokens, mut pg): (Vec<Token>, Program)) {
    use Token as T;

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

            T::LeftAngle => {
                if pg.ptr > 0 {
                    pg.ptr -= 1;
                }
            },
            T::RightAngle => {
                if pg.ptr < DATA_SIZE - 1 {
                    pg.ptr += 1;
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
            T::LeftAngle, T::RightAngle,
            T::OpenBrace, T::CloseBrace,
            T::Dot, T::Comma,
        ];

        assert_eq!(expected, lexer(source, false).0);
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

        assert_eq!(expected, lexer(source, true).0);

    }
}

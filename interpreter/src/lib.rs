use std::{io::stdin, collections::HashMap};

const DATA_SIZE: usize = 30_000;

pub struct Program {
    data: [u8; DATA_SIZE],
    ptr: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    RightAngle, // >
    LeftAngle,  // <
    Plus,
    Minus,
    Dot,
    Comma,
    OpenBrace,  // [
    CloseBrace, // ]
}

pub fn lexer(file: &str) -> Vec<Token> {
    use Token as T;

    let mut output = Vec::with_capacity(file.len());

    for ch in file.chars() {
        // flag is set if char is 
        // not a command
        let mut flag = false;

        let token = match ch {
            '>' => T::RightAngle,
            '<' => T::LeftAngle,
            '+' => T::Plus,
            '-' => T::Minus,
            '.' => T::Dot,
            ',' => T::Comma,
            '[' => T::OpenBrace,
            ']' => T::CloseBrace,
            _ => {
                flag = true;
                T::Plus
            } 
            
        };
        if !flag {
            output.push(token);
        }
    }
    output
}

pub fn execute(tokens: Vec<Token>) {
    use Token as T;

    let brace_list = build_brace_list(&tokens);

    let mut pg = Program {
        data: [0u8; DATA_SIZE],
        ptr: 0
    };

    let mut code_position = 0;
     while code_position < tokens.len() {

        match tokens[code_position] {
            T::Plus => { 
                pg.data[pg.ptr] = pg.data[pg.ptr]
                    .wrapping_add(1); 
            },
            T::Minus => { 
                pg.data[pg.ptr] = pg.data[pg.ptr]
                    .wrapping_sub(1); 
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

#[cfg(test)]
mod tests {
    use super::*;
    use Token as T;

    #[test]
    fn test_lexer() {

        let source = "+ - <> [] 21092 \
        foobar @$#²³£¬¢£ . , ";

        let expected = vec![
            T::Plus, T::Minus,
            T::LeftAngle, T::RightAngle,
            T::OpenBrace, T::CloseBrace,
            T::Dot, T::Comma,
        ];

        assert_eq!(expected, lexer(source));
    }
}

use std::io::stdin;


const DATA_SIZE: usize = 30_000;
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

    let mut data = [0u8; DATA_SIZE];
    let mut ptr = 0;


    for tok in tokens {
        
        match tok {
            T::Plus => { 
                data[ptr] = data[ptr].wrapping_add(1); 
            },
            T::Minus => { 
                data[ptr] = data[ptr].wrapping_sub(1); 
            },
            T::LeftAngle => ptr -= 1,
            T::RightAngle => ptr +=1,
            T::Dot => print!("{}", data[ptr] as char),
            T::Comma => {
                let mut input = String::new();
                
                stdin().read_line(&mut input)
                    .expect("input error");

                data[ptr] = input.bytes()
                    .nth(0).unwrap();
            }
            _ => todo!("loops"),
        };
    }
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

#[allow(unused)]
#[derive(Debug)]
struct Data {
    data: [u8; 30_000],
    pointer: usize,
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

mod consume_number;
pub mod token;

use self::consume_number::consume_number;
use self::token::Token;

pub fn tokenize(code: String) -> Vec<Token> {
    let mut chars = code.chars().peekable();

    let mut tokens: Vec<Token> = Vec::new();
    while let Some(c) = chars.peek() {
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                tokens.push(consume_number(&mut chars))
            }
            '+' | '*' => {
                tokens.push(Token::Operator(c.to_string()));
                chars.next();
            }
            _ => panic!("unexpected char {:?}", c),
        }
    }
    tokens
}

#[cfg(test)]
mod tests {

    use self::super::*;

    #[test]
    fn sigle_number() {
        assert_eq!(tokenize("10".to_string()), vec![Token::Number(10)]);
    }

    #[test]
    fn binary_add() {
        assert_eq!(
            tokenize("10+20".to_string()),
            vec![
                Token::Number(10),
                Token::Operator("+".to_string()),
                Token::Number(20)
            ]
        );
    }

    #[test]
    fn binary_mul() {
        assert_eq!(
            tokenize("10*20".to_string()),
            vec![
                Token::Number(10),
                Token::Operator("*".to_string()),
                Token::Number(20)
            ]
        );
    }

    #[test]
    #[should_panic(expected = "unexpected char \'a\'")]
    fn illegal_number() {
        tokenize("1a0".to_string());
    }
}

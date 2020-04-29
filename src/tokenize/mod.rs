mod consume_number;
pub mod token;

use self::consume_number::consume_number;
use self::token::Token;

pub fn tokenize(code: String) -> Vec<Token> {
    let mut chars = code.chars().peekable();

    let mut tokens: Vec<Token> = Vec::new();
    while let Some(c) = chars.peek() {
        if c.is_ascii_digit() {
            tokens.push(consume_number(&mut chars));
        } else if c == &'+' || c == &'*' {
            tokens.push(Token::Operator(c.to_string()));
            chars.next();
        } else if c == &' ' || c == &'\n' {
            chars.next();
        } else {
            panic!("unexpected char {:?}", c);
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
    fn binary_calc_with_whitespace() {
        assert_eq!(
            tokenize("1 + 2 * 3".to_string()),
            vec![
                Token::Number(1),
                Token::Operator("+".to_string()),
                Token::Number(2),
                Token::Operator("*".to_string()),
                Token::Number(3)
            ]
        );
    }

    #[test]
    fn binary_calc_with_newline() {
        assert_eq!(
            tokenize("1\n+\n2\n*\n3".to_string()),
            vec![
                Token::Number(1),
                Token::Operator("+".to_string()),
                Token::Number(2),
                Token::Operator("*".to_string()),
                Token::Number(3)
            ]
        );
    }

    #[test]
    #[should_panic(expected = "unexpected char \'a\'")]
    fn illegal_number() {
        tokenize("1a0".to_string());
    }
}

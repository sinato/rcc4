use super::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub fn consume_number(chars: &mut Peekable<Chars>) -> Token {
    let mut s = String::from("");
    while let Some(c) = chars.peek() {
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => s += &c.to_string(),
            _ => break,
        }
        chars.next();
    }
    let num: u64 = s.parse().unwrap_or_else(|err| panic!(err));
    Token::Number(num)
}

#[cfg(test)]
mod tests {

    use self::super::*;

    fn run(code: String) -> Token {
        let mut chars = code.chars().peekable();
        consume_number(&mut chars)
    }

    #[test]
    fn sigle_number() {
        assert_eq!(run("10".to_string()), Token::Number(10));
    }

    #[test]
    fn single_number_with_a_separated() {
        assert_eq!(run("77a99".to_string()), Token::Number(77));
    }
}

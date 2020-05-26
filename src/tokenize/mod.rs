pub mod token;
pub mod tokens;

use self::token::{ManagedToken, Token};
use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
    cursor_line: u32,
    cursor_location: u32,
}
impl<'a> Tokenizer<'a> {
    pub fn new(code: &'a String) -> Tokenizer {
        Tokenizer {
            chars: code.chars().peekable(),
            cursor_line: 0,
            cursor_location: 0,
        }
    }

    pub fn tokenize(code: &'a String) -> Vec<ManagedToken> {
        let mut tokenizer = Tokenizer::new(code);

        let mut tokens: Vec<ManagedToken> = Vec::new();
        while let Some(c) = tokenizer.peek() {
            if c.is_ascii_digit() {
                tokens.push(tokenizer.consume_number());
            } else if c == &'+' || c == &'*' || c == &'=' {
                tokens.push(tokenizer.consume_operator());
            } else if c == &'{' || c == &'}' {
                tokens.push(tokenizer.consume_bracket());
            } else if c == &'[' || c == &']' {
                tokens.push(tokenizer.consume_sbracket());
            } else if c == &'(' || c == &')' {
                tokens.push(tokenizer.consume_parenthesis());
            } else if c == &' ' || c == &'\n' {
                tokenizer.next();
            } else if c == &';' {
                tokens.push(tokenizer.consume_semicolon());
            } else if c == &',' {
                tokens.push(tokenizer.consume_comma());
            } else if c.is_ascii_alphabetic() {
                tokens.push(tokenizer.consume_identifier());
            } else {
                panic!("unexpected char {:?}", c);
            }
        }
        tokens
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            if c == &'\n' {
                self.cursor_line += 1;
                self.cursor_location = 0;
            } else {
                self.cursor_location += 1;
            }
        }
        self.chars.next()
    }

    pub fn consume_number(&mut self) -> ManagedToken {
        let line = self.cursor_line;
        let location = self.cursor_location;

        let mut s = String::from("");
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                s += &c.to_string();
            } else {
                break;
            }
            self.next();
        }
        let num: u64 = s.parse().unwrap_or_else(|err| panic!(err));
        ManagedToken::new(Token::Number(num), line, location)
    }
    fn consume_identifier(&mut self) -> ManagedToken {
        let line = self.cursor_line;
        let location = self.cursor_location;

        let mut s = String::from("");
        while let Some(c) = self.peek() {
            if c.is_alphabetic() || c.is_ascii_digit() {
                s += &c.to_string();
            } else {
                break;
            }
            self.next();
        }
        // treat as reserved keyword
        if s == "return" {
            ManagedToken::new(Token::Return, line, location)
        } else if s == "int" {
            ManagedToken::new(Token::Type(s), line, location)
        } else {
            ManagedToken::new(Token::Identifier(s), line, location)
        }
    }
    fn consume_operator(&mut self) -> ManagedToken {
        let line = self.cursor_line;
        let location = self.cursor_location;

        let c = self.next().unwrap();
        ManagedToken::new(Token::Operator(c.to_string()), line, location)
    }
    fn consume_bracket(&mut self) -> ManagedToken {
        let line = self.cursor_line;
        let location = self.cursor_location;

        let c = self.next().unwrap();
        ManagedToken::new(Token::Bracket(c.to_string()), line, location)
    }
    fn consume_sbracket(&mut self) -> ManagedToken {
        let line = self.cursor_line;
        let location = self.cursor_location;

        let c = self.next().unwrap();
        ManagedToken::new(Token::SBracket(c.to_string()), line, location)
    }
    fn consume_parenthesis(&mut self) -> ManagedToken {
        let line = self.cursor_line;
        let location = self.cursor_location;

        let c = self.next().unwrap();
        ManagedToken::new(Token::Parenthesis(c.to_string()), line, location)
    }
    fn consume_semicolon(&mut self) -> ManagedToken {
        let line = self.cursor_line;
        let location = self.cursor_location;
        self.next();
        ManagedToken::new(Token::Semicolon, line, location)
    }
    fn consume_comma(&mut self) -> ManagedToken {
        let line = self.cursor_line;
        let location = self.cursor_location;
        self.next();
        ManagedToken::new(Token::Comma, line, location)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[cfg(test)]
    mod consumer {

        use super::*;

        fn run(code: String) -> Token {
            let mut tokenizer = Tokenizer::new(&code);
            From::from(tokenizer.consume_number())
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

    // return Vec<Token> (not Vec<ManagedToken> for test readability)
    fn get_only_tokenized_tokens(code: &str) -> Vec<Token> {
        Tokenizer::tokenize(&code.to_string())
            .into_iter()
            .map(|mtoken| From::from(mtoken))
            .collect::<Vec<Token>>()
    }

    #[test]
    fn sigle_number() {
        assert_eq!(get_only_tokenized_tokens("10"), vec![Token::Number(10),])
    }

    #[test]
    fn binary_add() {
        assert_eq!(
            get_only_tokenized_tokens("10+20"),
            vec![
                Token::Number(10),
                Token::Operator("+".to_string()),
                Token::Number(20),
            ]
        )
    }

    #[test]
    fn binary_mul() {
        assert_eq!(
            get_only_tokenized_tokens("10*20"),
            vec![
                Token::Number(10),
                Token::Operator("*".to_string()),
                Token::Number(20),
            ]
        )
    }

    #[test]
    fn binary_calc_with_whitespace() {
        assert_eq!(
            get_only_tokenized_tokens("1 + 2 * 3"),
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
            get_only_tokenized_tokens("1\n+\n2\n*\n3"),
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
    fn crazy_tokenize() {
        // should panic...
        assert_eq!(
            get_only_tokenized_tokens("1a0"),
            vec![Token::Number(1), Token::Identifier("a0".to_string()),]
        );
    }

    #[test]
    fn check_management_info() {
        let actual = Tokenizer::tokenize(&"10 + 20\n30 * 40".to_string());
        let expect = vec![
            ManagedToken::new(Token::Number(10), 0, 0),
            ManagedToken::new(Token::Operator("+".to_string()), 0, 3),
            ManagedToken::new(Token::Number(20), 0, 5),
            ManagedToken::new(Token::Number(30), 1, 0),
            ManagedToken::new(Token::Operator("*".to_string()), 1, 3),
            ManagedToken::new(Token::Number(40), 1, 5),
        ];
        assert_eq!(actual, expect);
    }
}

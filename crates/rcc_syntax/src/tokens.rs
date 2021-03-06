use super::token::{ManagedToken, Token};
use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::vec::IntoIter;

type Result<T> = std::result::Result<T, ConsumeError>;

#[derive(Debug, Clone)]
pub struct Tokens {
    tokens: Peekable<IntoIter<ManagedToken>>,
}
impl Tokens {
    pub fn new(tokens: Vec<ManagedToken>) -> Tokens {
        Tokens {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn peek(&self) -> Option<ManagedToken> {
        let mut tokens = self.clone();
        tokens.next()
    }

    pub fn peek2(&self) -> Option<ManagedToken> {
        let mut tokens = self.clone();
        tokens.next();
        tokens.next()
    }

    pub fn next(&mut self) -> Option<ManagedToken> {
        self.tokens.next()
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn check_next_operator(&mut self, target_operator: &str) -> Option<ManagedToken> {
        if let Some(token) = self.peek() {
            if let Token::Operator(operator) = token.get_token() {
                if operator == target_operator {
                    return Some(token);
                }
            }
        }
        None
    }

    pub fn check_next_is_expression_node(&self) -> bool {
        if let Some(token) = self.peek() {
            if let Token::Identifier(_) = token.get_token() {
                return true;
            }
            if let Token::Number(_) = token.get_token() {
                return true;
            }
        }
        false
    }

    pub fn consume_expression(&mut self) -> Result<Vec<ManagedToken>> {
        let mut target_tokens: Vec<ManagedToken> = Vec::new();
        while let Some(token) = self.tokens.peek() {
            match token.get_token() {
                Token::Number(_) | Token::Operator(_) => {
                    target_tokens.push(self.tokens.next().unwrap());
                }
                _ => {
                    break;
                }
            }
        }
        Ok(target_tokens)
    }

    pub fn consume_to_binary_operator(&mut self, operator: String) -> Vec<ManagedToken> {
        let mut target_tokens: Vec<ManagedToken> = Vec::new();
        while let Some(token) = self.peek() {
            if let Token::Operator(op) = token.get_token() {
                if op == &operator {
                    self.next();
                    break;
                }
            }
            if let Token::Semicolon = token.get_token() {
                break;
            }
            target_tokens.push(self.next().unwrap());
        }
        target_tokens
    }

    pub fn consume_type(&mut self) -> Result<ManagedToken> {
        match self.tokens.peek() {
            Some(token) => match token.get_token() {
                Token::Type(_) => Ok(self.tokens.next().unwrap()),
                _ => Err(ConsumeError::Consume(Some(token.clone()))),
            },
            None => Err(ConsumeError::Consume(None)),
        }
    }

    pub fn consume_identifier(&mut self) -> Result<ManagedToken> {
        match self.tokens.peek() {
            Some(token) => match token.get_token() {
                Token::Identifier(_) => Ok(self.tokens.next().unwrap()),
                _ => Err(ConsumeError::Consume(Some(token.clone()))),
            },
            None => Err(ConsumeError::Consume(None)),
        }
    }

    pub fn consume_parenthesis(&mut self) -> Result<ManagedToken> {
        match self.tokens.peek() {
            Some(token) => match token.get_token() {
                Token::Parenthesis(_) => Ok(self.tokens.next().unwrap()),
                _ => Err(ConsumeError::Consume(Some(token.clone()))),
            },
            None => Err(ConsumeError::Consume(None)),
        }
    }

    pub fn consume_bracket(&mut self) -> Result<ManagedToken> {
        match self.tokens.peek() {
            Some(token) => match token.get_token() {
                Token::Bracket(_) => Ok(self.tokens.next().unwrap()),
                _ => Err(ConsumeError::Consume(Some(token.clone()))),
            },
            None => Err(ConsumeError::Consume(None)),
        }
    }

    pub fn consume_return(&mut self) -> Result<ManagedToken> {
        match self.tokens.peek() {
            Some(token) => match token.get_token() {
                Token::Return => Ok(self.tokens.next().unwrap()),
                _ => Err(ConsumeError::Consume(Some(token.clone()))),
            },
            None => Err(ConsumeError::Consume(None)),
        }
    }

    pub fn consume_comma(&mut self) -> Result<ManagedToken> {
        match self.tokens.peek() {
            Some(token) => match token.get_token() {
                Token::Comma => Ok(self.tokens.next().unwrap()),
                _ => Err(ConsumeError::Consume(Some(token.clone()))),
            },
            None => Err(ConsumeError::Consume(None)),
        }
    }

    pub fn consume_semicolon(&mut self) -> Result<ManagedToken> {
        match self.tokens.peek() {
            Some(token) => match token.get_token() {
                Token::Semicolon => Ok(self.tokens.next().unwrap()),
                _ => Err(ConsumeError::Consume(Some(token.clone()))),
            },
            None => Err(ConsumeError::Consume(None)),
        }
    }
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tokens =========================\n")?;
        let mut tokens = self.clone();
        while let Some(mtoken) = tokens.next() {
            write!(f, "{:?}\n", mtoken.get_token())?;
        }
        write!(f, "================================\n")
    }
}

#[derive(Debug, PartialEq)]
pub enum ConsumeError {
    Consume(Option<ManagedToken>),
}

impl fmt::Display for ConsumeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsumeError::Consume(optional_token) => match optional_token {
                Some(token) => write!(f, "{:?}", token),
                None => write!(f, "next token not found"),
            },
        }
    }
}

impl Error for ConsumeError {
    fn description(&self) -> &str {
        match *self {
            ConsumeError::Consume(_) => "consume error",
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[cfg(test)]
    mod consumer {

        use super::*;

        #[test]
        fn consume_to_binary_operator_add() {
            let mut tokens = Tokens::new(
                vec![
                    Token::Number(10),
                    Token::Number(20),
                    Token::Operator("+".to_owned()),
                ]
                .into_iter()
                .map(|token| ManagedToken::new(token, 0, 0))
                .collect(),
            );
            let actual: Vec<Token> = tokens
                .consume_to_binary_operator("+".to_string())
                .into_iter()
                .map(|mtoken| From::from(mtoken))
                .collect();
            assert_eq!(actual, vec![Token::Number(10), Token::Number(20)]);
        }

        #[test]
        fn consume_to_binary_operator_eol() {
            let mut tokens = Tokens::new(
                vec![Token::Number(10), Token::Number(20)]
                    .into_iter()
                    .map(|token| ManagedToken::new(token, 0, 0))
                    .collect(),
            );
            let actual: Vec<Token> = tokens
                .consume_to_binary_operator("+".to_string())
                .into_iter()
                .map(|mtoken| From::from(mtoken))
                .collect();
            assert_eq!(actual, vec![Token::Number(10), Token::Number(20)]);
        }
    }
}

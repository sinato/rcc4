use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Number(u64),
    Operator(String),
}

impl Token {
    pub fn get_number(self) -> Result<u64, TokenError> {
        if let Token::Number(num) = self {
            return Ok(num);
        }
        Err(TokenError::UnexpectedType(self))
    }

    pub fn get_operator(self) -> Result<String, TokenError> {
        if let Token::Operator(op) = self {
            return Ok(op);
        }
        Err(TokenError::UnexpectedType(self))
    }
}

#[derive(Debug)]
pub enum TokenError {
    UnexpectedType(Token),
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenError::UnexpectedType(token) => write!(f, "unexpected type token {:?}", token),
        }
    }
}

impl Error for TokenError {
    fn description(&self) -> &str {
        match *self {
            TokenError::UnexpectedType(_) => "unexpected type",
        }
    }
}

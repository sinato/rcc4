use super::super::tokenize::token::{ManagedToken, TokenError};
use super::super::tokenize::tokens::ConsumeError;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Consume(ConsumeError),
    Token(TokenError),
    Unexpect(Option<ManagedToken>),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Consume(err) => write!(f, "parse error: {}", err.to_string()),
            ParseError::Token(err) => write!(f, "token error: {}", err.to_string()),
            ParseError::Unexpect(optional_token) => {
                write!(f, "parse error: unexpected token {:?}", optional_token)
            }
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::Consume(_) => "parse error: consuming tokens",
            ParseError::Token(_) => "parse error: consuming tokens",
            ParseError::Unexpect(_) => "parse error: unexpected token",
        }
    }
}

impl From<ConsumeError> for ParseError {
    fn from(err: ConsumeError) -> ParseError {
        ParseError::Consume(err)
    }
}

impl From<TokenError> for ParseError {
    fn from(err: TokenError) -> ParseError {
        ParseError::Token(err)
    }
}

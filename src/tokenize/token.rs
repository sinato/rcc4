use std::error::Error;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct ManagedToken {
    token: Token,
    line: u32,
    location: u32,
}

impl ManagedToken {
    pub fn new(token: Token, line: u32, location: u32) -> ManagedToken {
        ManagedToken {
            token,
            line,
            location,
        }
    }
    pub fn get_token(&self) -> &Token {
        &self.token
    }
}
impl From<ManagedToken> for Token {
    fn from(managed_token: ManagedToken) -> Token {
        managed_token.token
    }
}
impl fmt::Display for ManagedToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}
impl fmt::Debug for ManagedToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Number(u64),
    Type(String),
    Identifier(String),
    Operator(String),
    Bracket(String),
    Parenthesis(String),
    Return,
    Comma,
    Semicolon,
}

impl Token {
    pub fn get_identifier(&self) -> Result<String, TokenError> {
        if let Token::Identifier(identifier) = self {
            return Ok(identifier.to_owned());
        }
        Err(TokenError::UnexpectedType(self.clone()))
    }

    pub fn get_type(&self) -> Result<String, TokenError> {
        if let Token::Type(ty) = self {
            return Ok(ty.to_owned());
        }
        Err(TokenError::UnexpectedType(self.clone()))
    }

    pub fn get_number(&self) -> Result<u64, TokenError> {
        if let Token::Number(num) = self {
            return Ok(*num);
        }
        Err(TokenError::UnexpectedType(self.clone()))
    }

    pub fn get_operator(&self) -> Result<String, TokenError> {
        if let Token::Operator(op) = self.clone() {
            return Ok(op);
        }
        Err(TokenError::UnexpectedType(self.clone()))
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Token::Number(number) => format!("number: {}", number),
            Token::Type(ty) => format!("type: {}", ty),
            Token::Identifier(identifier) => format!("identifier: {}", identifier),
            Token::Operator(operator) => format!("operator: {}", operator),
            Token::Bracket(bracket) => format!("bracket: {}", bracket),
            Token::Parenthesis(parenthesis) => format!("parenthesis: {}", parenthesis),
            Token::Return => format!("return"),
            Token::Comma => format!("comma"),
            Token::Semicolon => format!("semicolon"),
        };
        write!(f, "{}", string)
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

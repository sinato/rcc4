use super::super::tokenize::token::{Token, TokenError};
use inkwell::support::LLVMString;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CompileError {
    Emit(TokenError),
    LLVM(LLVMString),
    Undeclared(String),
    Unexpect(Token),
    NotFound(String),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompileError::Emit(err) => write!(f, "{}", err.to_string()),
            CompileError::LLVM(err) => write!(f, "{}", err.to_string()),
            CompileError::Undeclared(identifier) => {
                write!(f, "undeclared identifier {}", identifier)
            }
            CompileError::Unexpect(token) => write!(f, "unexpected token {}", token),
            CompileError::NotFound(cause) => write!(f, "{} not found", cause),
        }
    }
}

impl Error for CompileError {
    fn description(&self) -> &str {
        match *self {
            CompileError::Emit(_) => "compile error",
            CompileError::LLVM(_) => "llvm error",
            CompileError::Undeclared(_) => "undeclared",
            CompileError::Unexpect(_) => "unexpected",
            CompileError::NotFound(_) => "notfound",
        }
    }
}

impl From<TokenError> for CompileError {
    fn from(err: TokenError) -> CompileError {
        CompileError::Emit(err)
    }
}

impl From<LLVMString> for CompileError {
    fn from(err: LLVMString) -> CompileError {
        CompileError::LLVM(err)
    }
}

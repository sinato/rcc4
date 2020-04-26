use super::super::tokenize::token::TokenError;
use inkwell::support::LLVMString;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CompileError {
    Emit(TokenError),
    LLVM(LLVMString),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompileError::Emit(err) => write!(f, "{}", err.to_string()),
            CompileError::LLVM(err) => write!(f, "{}", err.to_string()),
        }
    }
}

impl Error for CompileError {
    fn description(&self) -> &str {
        match *self {
            CompileError::Emit(_) => "compile error",
            CompileError::LLVM(_) => "llvm error",
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

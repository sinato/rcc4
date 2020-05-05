pub mod error;
pub mod expression;
pub mod function;
pub mod program;
pub mod statement;
pub mod testutil;
pub mod util;

use super::tokenize::tokens::Tokens;
use error::ParseError;
use program::Program;

type Result<T> = std::result::Result<T, ParseError>;

pub fn parse(tokens: &mut Tokens) -> Result<Program> {
    Program::parse(tokens)
}

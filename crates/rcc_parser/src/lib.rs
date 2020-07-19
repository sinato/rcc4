pub mod error;
pub mod expression;
pub mod function;
pub mod program;
pub mod statement;
pub mod testutil;
pub mod util;

use error::ParseError;
use program::Program;
use rcc_syntax::tokens::Tokens;

type Result<T> = std::result::Result<T, ParseError>;

pub fn parse(tokens: &mut Tokens) -> Result<Program> {
    Program::parse(tokens)
}

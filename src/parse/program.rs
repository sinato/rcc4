use super::super::tokenize::tokens::Tokens;
use super::error::ParseError;
use super::function::Function;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub functions: Vec<Function>,
}
impl Program {
    /// parse and get program
    ///
    /// program := function+
    pub fn parse(tokens: &mut Tokens) -> Result<Program> {
        let mut functions: Vec<Function> = vec![];
        while let Some(_token) = tokens.peek() {
            functions.push(Function::parse(tokens)?);
        }
        Ok(Program { functions })
    }

    pub fn to_string(&self) -> String {
        let mut s = "".to_owned();
        s += "Program =========================\n";
        for function in &self.functions {
            s += &format!("{}", function.to_string());
        }
        s += "================================\n";
        s
    }
}

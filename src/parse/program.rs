use super::super::tokenize::token::ManagedToken;

use super::super::tokenize::tokens::Tokens;
use super::error::ParseError;
use super::expression::ExpressionNode;
use super::statement::{DeclareStatement, ExpressionStatement, ReturnStatement};

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub identifier: ManagedToken,
    pub return_type: ManagedToken,
    pub argument_types: Vec<ManagedToken>,
    pub block: Vec<ExpressionNode>,
}
impl Program {
    pub fn new(
        identifier: ManagedToken,
        return_type: ManagedToken,
        argument_types: Vec<ManagedToken>,
        block: Vec<ExpressionNode>,
    ) -> Program {
        Program {
            identifier,
            return_type,
            argument_types,
            block,
        }
    }

    pub fn parse(mut tokens: Tokens) -> Result<Box<Program>> {
        let return_type = tokens.consume_type()?;
        let identifier = tokens.consume_identifier()?;
        tokens.consume_parenthesis()?; // consume (
        let argument_types: Vec<ManagedToken> = vec![];
        tokens.consume_parenthesis()?; // consume )

        tokens.consume_bracket()?; // consume {
        DeclareStatement::parse(&mut tokens)?;
        ExpressionStatement::parse(&mut tokens)?;
        let expression = *ReturnStatement::parse(&mut tokens)?;
        let block = vec![expression];
        tokens.consume_bracket()?; // consume }

        assert!(tokens.len() == 0);
        Ok(Box::new(Program::new(
            identifier,
            return_type,
            argument_types,
            block,
        )))
    }

    pub fn to_string(&self) -> String {
        let mut s = "".to_owned();
        s += "AST ============================\n";
        s += &format!("function: {}\n", self.identifier);
        s += &format!("return_type: {}\n", self.return_type);
        s += &format!("argument_types:\n");
        for argument in self.argument_types.iter() {
            s += &format!("{:?}\n", argument);
        }
        s += &format!("block:\n");
        for expression in self.block.iter() {
            s += &expression.to_string(1);
        }
        s += "================================\n";
        s
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::tokenize::token::Token;
    use super::super::testutil::*;
    use super::*;

    #[test]
    fn main_func() {
        let actual = *Program::parse(Tokens::new(
            vec![
                Token::Type("int".to_owned()),
                Token::Identifier("main".to_owned()),
                Token::Parenthesis("(".to_owned()),
                Token::Parenthesis(")".to_owned()),
                Token::Bracket("{".to_owned()),
                Token::Return,
                Token::Number(10),
                Token::Semicolon,
                Token::Bracket("}".to_owned()),
            ]
            .into_iter()
            .map(|token| ManagedToken::new(token, 0, 0))
            .collect(),
        ))
        .unwrap();

        let expect = Program::new(
            ManagedToken::new(Token::Identifier("main".to_owned()), 0, 0),
            ManagedToken::new(Token::Type("int".to_owned()), 0, 0),
            vec![],
            vec![*num(10)],
        );
        assert_eq!(actual, expect);
    }
}

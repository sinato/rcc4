pub mod error;
pub mod expression;
pub mod node;
mod statement;
pub mod testutil;

use super::tokenize::token::ManagedToken;
use super::tokenize::tokens::Tokens;
use error::ParseError;
use node::FunctionNode;
use statement::parse_return_statement;

pub fn parse(mut tokens: Tokens) -> Result<Box<FunctionNode>, ParseError> {
    let return_type = tokens.consume_identifier()?;
    let identifier = tokens.consume_identifier()?;
    tokens.consume_parenthesis()?; // consume (
    let argument_types: Vec<ManagedToken> = vec![];
    tokens.consume_parenthesis()?; // consume )

    tokens.consume_bracket()?; // consume {
    let expression = *parse_return_statement(&mut tokens)?;
    let block = vec![expression];
    tokens.consume_bracket()?; // consume }

    assert!(tokens.len() == 0);
    Ok(Box::new(FunctionNode::new(
        identifier,
        return_type,
        argument_types,
        block,
    )))
}

#[cfg(test)]
mod tests {

    use super::super::tokenize::token::Token;
    use super::testutil::*;
    use super::*;

    #[test]
    fn main_func() {
        let actual = *parse(Tokens::new(
            vec![
                Token::Identifier("int".to_owned()),
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

        let expect = FunctionNode::new(
            ManagedToken::new(Token::Identifier("main".to_owned()), 0, 0),
            ManagedToken::new(Token::Identifier("int".to_owned()), 0, 0),
            vec![],
            vec![*num(10)],
        );
        assert_eq!(actual, expect);
    }
}

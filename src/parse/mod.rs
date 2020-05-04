mod expression;
pub mod node;

use super::tokenize::token::ManagedToken;
use super::tokenize::tokens::{ConsumeError, Tokens};
use expression::parse_expression;
use node::{ExpressionNode, FunctionNode};

pub fn parse(mut tokens: Tokens) -> Result<Box<FunctionNode>, ConsumeError> {
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

pub fn parse_return_statement(tokens: &mut Tokens) -> Result<Box<ExpressionNode>, ConsumeError> {
    tokens.consume_return()?;
    let expression_node = parse_expression(tokens);
    tokens.consume_semicolon()?;
    Ok(expression_node)
}

#[cfg(test)]
mod tests {

    use self::super::super::tokenize::token::Token;
    use self::super::*;

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
            vec![ExpressionNode::new(Token::Number(10), vec![])],
        );
        assert_eq!(actual, expect);
    }
}

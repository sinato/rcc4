mod expression;
pub mod node;

use super::tokenize::token::ManagedToken;
use super::tokenize::tokens::{ParseError, Tokens};
use expression::parse_expression;
use node::FunctionNode;

pub fn parse(tokens: Vec<ManagedToken>) -> Result<Box<FunctionNode>, ParseError> {
    let mut tokens = Tokens::new(tokens);

    let return_type = tokens.consume_identifier()?;
    let identifier = tokens.consume_identifier()?;
    tokens.consume_parenthesis()?; // consume (
    let argument_types: Vec<ManagedToken> = vec![];
    tokens.consume_parenthesis()?; // consume )

    tokens.consume_bracket()?; // consume {
    let res = tokens.consume_expression()?;
    let expression = *parse_expression(&mut Tokens::new(res));
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

    use self::super::super::tokenize::token::Token;
    use self::super::node::ExpressionNode;
    use self::super::*;

    #[test]
    fn main_func() {
        let actual = *parse(
            vec![
                Token::Identifier("int".to_owned()),
                Token::Identifier("main".to_owned()),
                Token::Parenthesis("(".to_owned()),
                Token::Parenthesis(")".to_owned()),
                Token::Bracket("{".to_owned()),
                Token::Number(10),
                Token::Bracket("}".to_owned()),
            ]
            .into_iter()
            .map(|token| ManagedToken::new(token, 0, 0))
            .collect(),
        )
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

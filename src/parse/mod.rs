mod expression;
pub mod node;

use super::tokenize::token::ManagedToken;
use super::tokenize::tokens::{ParseError, Tokens};
use expression::parse_expression;
use node::Node;

pub fn parse(tokens: Vec<ManagedToken>) -> Result<Box<Node>, ParseError> {
    let mut tokens = Tokens::new(tokens);
    tokens.consume_identifier()?; // consume int
    tokens.consume_identifier()?; // consume main
    tokens.consume_parenthesis()?; // consume (
    tokens.consume_parenthesis()?; // consume )
    tokens.consume_bracket()?; // consume {

    let res = tokens.consume_expression()?;
    tokens.consume_bracket()?; // consume }

    assert!(tokens.len() == 0);

    Ok(parse_expression(&mut Tokens::new(res)))
}

#[cfg(test)]
mod tests {

    use self::super::super::tokenize::token::Token;
    use self::super::*;

    #[test]
    fn main_func() {
        let actual: Node = *parse(
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
        let expect: Node = Node {
            operator: Token::Number(10),
            operand: vec![],
        };
        assert_eq!(actual, expect);
    }
}
